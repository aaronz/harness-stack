#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/scripts/constitution.sh"
source "$SCRIPT_DIR/scripts/task-json.sh"
source "$SCRIPT_DIR/scripts/opencode-wrapper.sh"
source "$SCRIPT_DIR/scripts/phases.sh"

parse_args() {
    RESUME_ITERATION=""
    MODEL=""
    MAX_IMPLEMENTATION_ROUNDS=10
    PRD_INPUT=""
    LOG_FILE=""
    VERBOSE="false"

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --resume)
                RESUME_ITERATION="$2"
                shift 2
                ;;
            --model|-m)
                MODEL="$2"
                shift 2
                ;;
            --rounds|-r)
                MAX_IMPLEMENTATION_ROUNDS="$2"
                shift 2
                ;;
            --prd|-p)
                PRD_INPUT="$2"
                shift 2
                ;;
            --log)
                LOG_FILE="$2"
                shift 2
                ;;
            --verbose|-v)
                VERBOSE="true"
                shift
                ;;
            *)
                shift
                ;;
        esac
    done

    MODEL="${MODEL:-minimax-cn/MiniMax-M2.7}"
}

parse_args "$@"

# Logging setup
WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"
SESSION_LOG_DIR="$WORKSPACE_DIR/sessions"

mkdir -p "$SESSION_LOG_DIR"

# Logging function with timestamp
log() {
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local message="[$timestamp] $1"
    
    if [ "$VERBOSE" = "true" ]; then
        echo "$message"
    fi
    
    if [ -n "$LOG_FILE" ]; then
        echo "$message" >> "$LOG_FILE"
    fi
}

# Console output control - suppress if not verbose
log_echo() {
    if [ "$VERBOSE" = "true" ]; then
        echo "$1"
    else
        echo "$1" >> /dev/null
    fi
}

log "=============================================="
log "SpecKit Iteration Development v3.0 (Refactored)"
log "=============================================="
log "Working directory: $WORKSPACE_DIR"
log "Log file: $LOG_FILE"
CONSTITUTION_PATH="${CONSTITUTION_PATH:-$WORKSPACE_DIR/iterations/.specify/memory/constitution.md}"

if [ ! -f "$CONSTITUTION_PATH" ]; then
    CONSTITUTION_PATH="$WORKSPACE_DIR/iterations/iteration-1/constitution.md"
fi

if [ ! -f "$CONSTITUTION_PATH" ]; then
    log "Warning: Constitution file not found, will use default"
    CONSTITUTION_PATH=""
fi

if [ -n "$PRD_INPUT" ]; then
    if [ -d "$PRD_INPUT" ]; then
        mapfile -t prd_files < <(find "$PRD_INPUT" -maxdepth 1 -name "*.md" | sort)
        if [ ${#prd_files[@]} -eq 0 ]; then
            echo "Error: No .md files found in folder: $PRD_INPUT"
            exit 1
        fi
        OUTPUTS_DIR="${OUTPUTS_DIR:-$WORKSPACE_DIR/iterations}"
        PRD_PATH="$OUTPUTS_DIR/_prd_combined.md"
        cat "${prd_files[@]}" > "$PRD_PATH"
        echo "Using PRD folder: $PRD_INPUT (merged ${#prd_files[@]} files)"
    elif [ -f "$PRD_INPUT" ]; then
        PRD_PATH="$PRD_INPUT"
        echo "Using PRD file: $PRD_PATH"
    else
        echo "Error: PRD path does not exist: $PRD_INPUT"
        exit 1
    fi
else
    PRD_PATH="$WORKSPACE_DIR/PRD.md"
fi

if [ -n "$RESUME_ITERATION" ]; then
    NEXT_ITERATION="$RESUME_ITERATION"
    OUTPUTS_DIR="$WORKSPACE_DIR/iterations/iteration-${NEXT_ITERATION}"
    if [ ! -d "$OUTPUTS_DIR" ]; then
        echo "Error: Specified iteration does not exist: $OUTPUTS_DIR"
        exit 1
    fi
    echo "Resuming iteration #${NEXT_ITERATION}"
else
    LAST_ITERATION=$(ls -d "$WORKSPACE_DIR/iterations/iteration-"* 2>/dev/null | sed 's/.*iteration-//' | sort -n | tail -1)
    NEXT_ITERATION=${LAST_ITERATION:-0}
    NEXT_ITERATION=$((NEXT_ITERATION + 1))
    OUTPUTS_DIR="$WORKSPACE_DIR/iterations/iteration-${NEXT_ITERATION}"
    mkdir -p "$OUTPUTS_DIR"
fi

SESSION_EXPORT_DIR="$SESSION_LOG_DIR/iteration-${NEXT_ITERATION}"
mkdir -p "$SESSION_EXPORT_DIR"

if [ -z "$LOG_FILE" ]; then
    LOG_FILE="$SESSION_LOG_DIR/iteration-${NEXT_ITERATION}_$(date +%Y%m%d_%H%M%S).log"
fi

CONSTITUTION=$(load_constitution)

log "Iteration directory: $OUTPUTS_DIR"
log "Model: $MODEL"
log "Max implementation rounds: $MAX_IMPLEMENTATION_ROUNDS"
log "Constitution: $(get_constitution_summary)"
log ""

log "[1/6] Running PRD gap analysis..."
save_checkpoint "$NEXT_ITERATION" "phase1"
run_phase_gap_analysis "$PRD_PATH" "$OUTPUTS_DIR" "$CONSTITUTION"

log ""
log "[2/6] Constitution check..."
save_checkpoint "$NEXT_ITERATION" "phase2"
run_phase_constitution "$CONSTITUTION_PATH" "$OUTPUTS_DIR/gap-analysis.md" "$OUTPUTS_DIR"

log ""
log "[3/6] Updating Spec..."
save_checkpoint "$NEXT_ITERATION" "phase3"
run_phase_spec "$PRD_PATH" "$OUTPUTS_DIR/gap-analysis.md" "$CONSTITUTION" "$OUTPUTS_DIR" "$NEXT_ITERATION"

log ""
log "[4/6] Updating Plan and Tasks..."
save_checkpoint "$NEXT_ITERATION" "phase4"
run_phase_plan "$OUTPUTS_DIR/spec_v${NEXT_ITERATION}.md" "$CONSTITUTION" "$OUTPUTS_DIR/gap-analysis.md" "$OUTPUTS_DIR" "$NEXT_ITERATION"

TASKS_JSON="$OUTPUTS_DIR/tasks_v${NEXT_ITERATION}.json"

log ""
log "[5/6] Per-Task implementation loop..."
save_checkpoint "$NEXT_ITERATION" "phase5"
run_phase_implementation "$TASKS_JSON" "$OUTPUTS_DIR/spec_v${NEXT_ITERATION}.md" "$OUTPUTS_DIR" "$CONSTITUTION" "$MAX_IMPLEMENTATION_ROUNDS"

log ""
log "[6/6] Verification report..."
save_checkpoint "$NEXT_ITERATION" "phase6"
run_phase_verification "$OUTPUTS_DIR/gap-analysis.md" "$OUTPUTS_DIR/tasks_v${NEXT_ITERATION}.md" "$TASKS_JSON" "$OUTPUTS_DIR"

log ""
log "=============================================="
log "SpecKit Iteration completed!"
log "=============================================="
log "Iteration directory: $OUTPUTS_DIR"
log "Task file: $TASKS_JSON"
log "Verification report: $OUTPUTS_DIR/verification-report.md"

log "Log saved to: $LOG_FILE"
