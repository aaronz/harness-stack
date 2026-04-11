#!/usr/bin/env bash
# Common functions for methodology scripts

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILL_SOURCE_DIR="$SCRIPT_DIR/skill-source"
WORKSPACE_DIR="${WORKSPACE_DIR:-}"

METHODOLOGIES=("openspec" "gstack" "planning-with-files" "speckit" "superpowers" "everything-claude-code")
CLI_INIT_METHODOLOGIES=("openspec" "speckit")

SCRIPT_FILES=(
    "iterate-prd.sh"
    "iterate-prd.ps1"
    "prompts.md"
)

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# ============================================
# Shared Iteration Script Functions
# ============================================

# Default values
DEFAULT_MODEL="opencode/minimax-m2.5-free"

# Global iteration variables (set by parse_args)
RESUME_ITERATION=""
MODEL="$DEFAULT_MODEL"
MAX_IMPLEMENTATION_ROUNDS=10
PRD_INPUT=""
LOG_FILE=""
VERBOSE="false"

# Logging setup
SESSION_LOG_DIR=""

# ============================================
# Argument Parsing
# ============================================

parse_args() {
    RESUME_ITERATION=""
    MODEL="$DEFAULT_MODEL"
    MAX_IMPLEMENTATION_ROUNDS=10
    PRD_INPUT=""
    LOG_FILE=""
    VERBOSE="false"

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --resume|-R)
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
            --log|-l)
                LOG_FILE="$2"
                shift 2
                ;;
            --verbose|-v)
                VERBOSE="true"
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [options]"
                echo "Options:"
                echo "  --resume, -R <N>    Resume from iteration N"
                echo "  --model, -m <M>     Set model (default: $DEFAULT_MODEL)"
                echo "  --rounds, -r <N>    Max implementation rounds (default: 10)"
                echo "  --prd, -p <P>       PRD file or directory"
                echo "  --log, -l <F>       Log file path"
                echo "  --verbose, -v       Enable verbose output"
                echo "  --help, -h          Show this help"
                exit 0
                ;;
            *)
                shift
                ;;
        esac
    done
}

# ============================================
# Logging Functions
# ============================================

init_logging() {
    local workspace="${1:-$(pwd)}"
    SESSION_LOG_DIR="$workspace/sessions"
    mkdir -p "$SESSION_LOG_DIR"

    if [ -z "$LOG_FILE" ] && [ -n "$OUTPUTS_DIR" ]; then
        local iteration_name=$(basename "$OUTPUTS_DIR" | sed 's/iteration-//')
        LOG_FILE="$SESSION_LOG_DIR/iteration-${iteration_name}_$(date +%Y%m%d_%H%M%S).log"
    fi
}

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

log_echo() {
    if [ "$VERBOSE" = "true" ]; then
        echo "$1"
    fi
}

log_section() {
    log "=============================================="
    log "$1"
    log "=============================================="
}

# ============================================
# Checkpoint Functions
# ============================================

save_checkpoint() {
    local iteration="$1"
    local phase="$2"
    local checkpoint_file="$OUTPUTS_DIR/.checkpoint"
    
    mkdir -p "$OUTPUTS_DIR"
    cat > "$checkpoint_file" << EOF
iteration=$iteration
phase=$phase
timestamp=$(date '+%Y-%m-%d %H:%M:%S')
EOF
    log "Checkpoint saved: iteration=$iteration, phase=$phase"
}

load_checkpoint() {
    local checkpoint_file="$OUTPUTS_DIR/.checkpoint"
    
    if [ -f "$checkpoint_file" ]; then
        source "$checkpoint_file"
        echo "$iteration:$phase"
    else
        echo ""
    fi
}

# ============================================
# Iteration Management
# ============================================

get_next_iteration() {
    local workspace="$1"
    local pattern="${2:-iteration-}"
    
    LAST_ITERATION=$(ls -d "$workspace/outputs/${pattern}"* 2>/dev/null | sed "s/.*${pattern}//" | sort -n | tail -1 || echo "0")
    NEXT_ITERATION=$((LAST_ITERATION + 1))
    echo "$NEXT_ITERATION"
}

setup_iteration_output() {
    local workspace="$1"
    local method_name="$2"
    
    if [ -n "$RESUME_ITERATION" ]; then
        NEXT_ITERATION="$RESUME_ITERATION"
        OUTPUTS_DIR="$workspace/outputs/iteration-${NEXT_ITERATION}"
        if [ ! -d "$OUTPUTS_DIR" ]; then
            echo "Error: Resume iteration $OUTPUTS_DIR does not exist"
            exit 1
        fi
        log "Resuming iteration #${NEXT_ITERATION}"
    else
        NEXT_ITERATION=$(get_next_iteration "$workspace")
        OUTPUTS_DIR="$workspace/outputs/iteration-${NEXT_ITERATION}"
        mkdir -p "$OUTPUTS_DIR"
    fi
    
    mkdir -p "$SESSION_LOG_DIR/iteration-${NEXT_ITERATION}"
}

# ============================================
# PRD Path Resolution
# ============================================

resolve_prd_path() {
    local prd_input="$1"
    local workspace="$2"
    
    if [ -n "$prd_input" ]; then
        if [ -d "$prd_input" ]; then
            mapfile -t prd_files < <(find "$prd_input" -maxdepth 1 -name "*.md" | sort)
            if [ ${#prd_files[@]} -eq 0 ]; then
                echo "Error: No .md files found in $prd_input"
                exit 1
            fi
            local combined_path="$OUTPUTS_DIR/_prd_combined.md"
            cat "${prd_files[@]}" > "$combined_path"
            echo "$combined_path"
        elif [ -f "$prd_input" ]; then
            echo "$prd_input"
        else
            echo "Error: PRD path does not exist: $prd_input"
            exit 1
        fi
    else
        echo "$workspace/PRD.md"
    fi
}

# ============================================
# Shared Gap Analysis Prompt
# ============================================

GAP_ANALYSIS_PROMPT='分析当前实现与PRD的差距：

## 任务
1. 读取当前实现目录结构
2. 读取PRD.md识别核心功能需求
3. 对比实现与PRD的差距

## 分析维度
1. 功能完整性：PRD中描述的功能是否都已实现？
2. 接口完整性：API是否完整？CRUD是否齐全？
3. 前端完整性：PRD中描述的页面/组件是否都已实现？
4. 数据模型：PRD中的数据实体是否都已建模？
5. 配置管理：PRD中要求的配置项是否都已实现？
6. 测试覆盖：是否有必要的测试？

## 通用差距识别
- 缺失的功能模块
- 不完整的实现
- 未连接的模块
- 硬编码/魔法数字
- 错误处理缺失
- 类型定义缺失

## 输出格式
# 差距分析报告

## 差距列表
| 差距项 | 严重程度 | 模块 | 修复建议 |

## P0问题（必须修复）
...

## P1问题（应该修复）
...

## P2问题（可以修复）
...

## 技术债务
...'

needs_cli_init() {
    local method="$1"
    for m in "${CLI_INIT_METHODOLOGIES[@]}"; do
        if [[ "$m" == "$method" ]]; then
            return 0
        fi
    done
    return 1
}

print_usage() {
    echo "Usage: $0 [options] <methodology> <target-folder>"
    echo ""
    echo "Options:"
    echo "  -c, --clean    Clean .opencode/skills and .opencode/commands before install"
    echo ""
    echo "Available methodologies:"
    for m in "${METHODOLOGIES[@]}"; do
        echo "  - $m"
    done
    echo ""
    echo "Examples:"
    echo "  $0 superpowers ~/projects/my-app"
    echo "  $0 speckit ./my-workspace"
    echo "  $0 --clean planning-with-files ../other-project"
}

clean_target() {
    local target="$1"
    local skills_dir="$target/.opencode/skills"
    local commands_dir="$target/.opencode/commands"
    
    mkdir -p "$skills_dir"
    mkdir -p "$commands_dir"
    
    echo "Cleaning existing .opencode/skills, .opencode/commands, scripts"
    rm -rf "$skills_dir"/* 2>/dev/null || true
    rm -rf "$commands_dir"/* 2>/dev/null || true
    for script in "${SCRIPT_FILES[@]}"; do
        rm -f "$target/$script" 2>/dev/null || true
    done
}

copy_scripts() {
    local method="$1"
    local target="$2"
    local workspace_path="$WORKSPACE_DIR/Workspace-$method"
    
    if [[ -d "$workspace_path" ]]; then
        echo "Copying scripts from workspace"
        for script in "${SCRIPT_FILES[@]}"; do
            if [[ -f "$workspace_path/$script" ]]; then
                cp "$workspace_path/$script" "$target/"
            fi
        done
    fi
}

copy_skills_commands() {
    local method="$1"
    local source_dir="$SKILL_SOURCE_DIR/$method"
    local target="$2"
    
    local skills_dir="$target/.opencode/skills"
    local commands_dir="$target/.opencode/commands"
    
    mkdir -p "$skills_dir"
    mkdir -p "$commands_dir"
    
    local found_assets=false
    
    if [[ -d "$source_dir/.opencode/skills" ]]; then
        echo "Copying skills from $source_dir/.opencode/skills"
        cp -r "$source_dir/.opencode/skills"/* "$skills_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ -d "$source_dir/skills" ]]; then
        echo "Copying skills from $source_dir/skills"
        cp -r "$source_dir/skills"/* "$skills_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ "$method" == "gstack" ]]; then
        echo "Copying gstack skills from repo root"
        for dir in "$source_dir"/*/; do
            if [[ -f "$dir/SKILL.md" ]]; then
                dir_name=$(basename "$dir")
                mkdir -p "$skills_dir/$dir_name"
                cp -r "$dir"* "$skills_dir/$dir_name/" 2>/dev/null || true
                found_assets=true
            fi
        done
    fi
    
    if [[ -d "$source_dir/.opencode/commands" ]]; then
        echo "Copying commands from $source_dir/.opencode/commands"
        cp -r "$source_dir/.opencode/commands"/* "$commands_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ -d "$source_dir/commands" ]]; then
        echo "Copying commands from $source_dir/commands"
        cp -r "$source_dir/commands"/* "$commands_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ "$found_assets" == false ]]; then
        echo -e "${YELLOW}No skills/commands found in repo${NC}"
    fi
}

install_openspec() {
    local target="$1"
    
    if command -v openspec &> /dev/null; then
        cd "$target" && openspec init . --tools opencode --force 2>/dev/null || true
        echo -e "${GREEN}openspec init completed${NC}"
    else
        echo -e "${YELLOW}openspec CLI not found, installing...${NC}"
        npm install -g @fission-ai/openspec 2>/dev/null || true
        if command -v openspec &> /dev/null; then
            cd "$target" && openspec init . --tools opencode --force 2>/dev/null || true
            echo -e "${GREEN}openspec init completed${NC}"
        else
            echo -e "${YELLOW}Failed to install openspec, skipping init${NC}"
        fi
    fi
}

install_speckit() {
    local target="$1"
    
    if command -v specify &> /dev/null; then
        cd "$target" && specify init . --ai opencode --here --force 2>/dev/null || true
        echo -e "${GREEN}specify init completed${NC}"
    else
        echo -e "${YELLOW}specify CLI not found, installing...${NC}"
        installed=false
        
        if command -v uv &> /dev/null; then
            uv tool install specify-cli --from git+https://github.com/github/spec-kit.git 2>/dev/null && installed=true
        fi
        
        if [[ "$installed" == false ]] && command -v pipx &> /dev/null; then
            pipx install specify-cli 2>/dev/null && installed=true
        fi
        
        if [[ "$installed" == false ]] && command -v pip &> /dev/null; then
            pip install specify-cli 2>/dev/null && installed=true
        fi
        
        if [[ "$installed" == false ]] && command -v pip3 &> /dev/null; then
            pip3 install specify-cli 2>/dev/null && installed=true
        fi
        
        if command -v specify &> /dev/null; then
            cd "$target" && specify init . --ai opencode --here --force 2>/dev/null || true
            echo -e "${GREEN}specify init completed${NC}"
        else
            echo -e "${YELLOW}Failed to install specify, skipping init${NC}"
        fi
    fi
}

validate_methodology() {
    local method="$1"
    for m in "${METHODOLOGIES[@]}"; do
        if [[ "$m" == "$method" ]]; then
            return 0
        fi
    done
    return 1
}
