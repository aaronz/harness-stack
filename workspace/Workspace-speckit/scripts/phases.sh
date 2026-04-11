#!/bin/bash

run_phase_gap_analysis() {
    local prd_path="$1"
    local output_dir="$2"
    local constitution="$3"

    local gap_file="$output_dir/gap-analysis.md"

    if check_file_quiet "$gap_file"; then
        echo "Skipping Gap Analysis (exists)"
        return 0
    fi

    PROMPT_GAP_ANALYSIS="Analyze the gap between current implementation and PRD, and write the complete gap analysis report to file: $gap_file

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all analysis work directly in current session
- Use only Read, Write, Edit, Grep, LSP and other direct tools

## Task
1. Read current implementation directory structure (src/ directory, iterations/src/ directory, etc.)
2. Read PRD.md to identify core functional requirements
3. Compare implementation vs PRD gaps

## Analysis Dimensions
1. Feature Completeness: Are all features described in PRD implemented?
2. API Completeness: Are APIs complete? Are all CRUD operations present?
3. Frontend Completeness: Are all pages/components described in PRD implemented?
4. Data Model: Are all data entities from PRD modeled?
5. Configuration Management: Are all configuration items from PRD implemented?
6. Test Coverage: Are necessary tests present?

## Common Gap Identification
- Missing feature modules
- Incomplete implementations
- Unconnected modules
- Hardcoded/magic numbers
- Missing error handling
- Missing type definitions

## Output Requirements
Write the complete gap analysis report to: $gap_file

Report must include:
1. Gap list (table format: Gap | Severity | Module | Fix Suggestion)
2. P0/P1/P2 issue classification (must include P0 blocking issues)
3. Technical debt list
4. Implementation progress summary"

    generate_if_missing "$gap_file" "$PROMPT_GAP_ANALYSIS" 5
}

run_phase_constitution() {
    local constitution_path="$1"
    local gap_analysis="$2"
    local output_dir="$3"

    local const_update_file="$output_dir/constitution_updates.md"

    if check_file_quiet "$const_update_file"; then
        echo "Skipping Constitution check (exists)"
        return 0
    fi

    PROMPT_CONSTITUTION="Check if Constitution needs updating and write update suggestions to file: $const_update_file

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all analysis work directly in current session
- Use only Read, Write, Edit, Grep, LSP and other direct tools

## Constitution
$(cat $constitution_path 2>/dev/null || echo "Constitution does not exist")

## Gap Analysis
$(cat $gap_analysis)

## Task
1. Check if existing Constitution covers new P0 issues
2. If update needed, propose Constitution revision suggestions
3. Ensure new design decisions conform to Constitution

## Output Requirements
Write Constitution update suggestions to: $const_update_file"

    generate_if_missing "$const_update_file" "$PROMPT_CONSTITUTION" 5
}

run_phase_spec() {
    local prd_path="$1"
    local gap_analysis="$2"
    local constitution="$3"
    local output_dir="$4"
    local iteration="$5"

    local spec_file="$output_dir/spec_v${iteration}.md"

    if check_file_quiet "$spec_file"; then
        echo "Skipping Spec update (exists)"
        return 0
    fi

    PROMPT_SPEC="Based on PRD and gap analysis, update the specification document and write to file: $spec_file

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all analysis work directly in current session
- Use only Read, Write, Edit, Grep, LSP and other direct tools

## PRD
$(cat $prd_path)

## Gap Analysis
$(cat $gap_analysis)

## Constitution
$(cat $constitution 2>/dev/null || echo "Use default Constitution")

## Task
1. Based on gap analysis, update spec.md
2. Ensure new features have corresponding specification definitions
3. Add functional requirement IDs (FR-XXX)

## Output Requirements
Write the updated specification document to: $spec_file"

    generate_if_missing "$spec_file" "$PROMPT_SPEC" 5
}

run_phase_plan() {
    local spec_file="$1"
    local constitution="$2"
    local gap_analysis="$3"
    local output_dir="$4"
    local iteration="$5"

    local plan_file="$output_dir/plan_v${iteration}.md"
    local tasks_file="$output_dir/tasks_v${iteration}.md"
    local tasks_json="$output_dir/tasks_v${iteration}.json"

    if check_file_quiet "$plan_file" && check_file_quiet "$tasks_file"; then
        echo "Skipping Plan/Tasks update (exists)"
        return 0
    fi

    PROMPT_PLAN="Based on Spec, update implementation plan and task list, and write them to files.

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all analysis work directly in current session
- Use only Read, Write, Edit, Grep, LSP and other direct tools

## Spec
$(cat $spec_file)

## Constitution
$(cat $constitution 2>/dev/null || echo "")

## Gap Analysis
$(cat $gap_analysis)

## Task
1. Update implementation plan
2. Update task list
3. Ensure P0 tasks are prioritized

## Output Requirements
Write the updated plan to: $plan_file
Write the updated task list to: $tasks_file"

    generate_if_missing "$plan_file" "$PROMPT_PLAN" 5
    generate_if_missing "$tasks_file" "$PROMPT_PLAN" 5

    if ! check_file_quiet "$tasks_json"; then
        generate_tasks_json "$tasks_file" "$tasks_json"
    fi
}

run_phase_implementation() {
    local tasks_json="$1"
    local spec_file="$2"
    local output_dir="$3"
    local constitution="$4"
    local max_rounds="$5"

    TASK_FILE="$output_dir/tasks_v${NEXT_ITERATION}.md"
    TASKS_JSON="$tasks_json"

    for round in $(seq 1 $max_rounds); do
        echo ""
        echo "=============================================="
        echo "Outer loop round $round/$max_rounds"
        echo "=============================================="

        if [ ! -f "$TASKS_JSON" ]; then
            if [ -f "$TASK_FILE" ]; then
                generate_tasks_json "$TASK_FILE" "$TASKS_JSON"
            else
                echo "Warning: Task file not found: $TASK_FILE"
                break
            fi
        fi

        remaining_p0_p1=$(check_remaining_p0_p1 "$TASKS_JSON")
        echo "Remaining incomplete P0/P1 tasks: $remaining_p0_p1"

        todo_count=$(count_todo_tasks "$TASKS_JSON")
        done_count=$(count_done_tasks "$TASKS_JSON")
        total_count=$((todo_count + done_count))
        echo "Task progress: $done_count/$total_count completed"

        if [ "$remaining_p0_p1" -eq 0 ] && [ "$todo_count" -eq 0 ]; then
            echo "All P0/P1 tasks completed!"
            break
        fi

        if [ "$todo_count" -eq 0 ]; then
            echo "No more todo tasks"
            break
        fi

        echo ""
        echo "Starting task-by-task implementation..."

        while true; do
            next_task=$(get_next_todo_task "$TASKS_JSON")
            if [ -z "$next_task" ]; then
                echo "All todo tasks processed"
                break
            fi

            implement_task "$next_task" "$TASKS_JSON" "$spec_file" "$constitution"

            remaining_p0_p1=$(check_remaining_p0_p1 "$TASKS_JSON")
            if [ "$remaining_p0_p1" -eq 0 ]; then
                echo ""
                echo "All P0/P1 blocking issues resolved!"
                break
            fi
        done

        if [ $round -eq $max_rounds ]; then
            echo ""
            echo "Reached maximum outer loop rounds"
            remaining_p0_p1=$(check_remaining_p0_p1 "$TASKS_JSON")
            if [ "$remaining_p0_p1" -gt 0 ]; then
                echo "Still $remaining_p0_p1 P0/P1 tasks incomplete"
            fi
            echo "Continuing to verification phase..."
            break
        fi
    done
}

run_phase_verification() {
    local gap_analysis="$1"
    local tasks_md="$2"
    local tasks_json="$3"
    local output_dir="$4"

    local verif_file="$output_dir/verification-report.md"

    if check_file_quiet "$verif_file"; then
        echo "Skipping verification report (exists)"
        return 0
    fi

    PROMPT_VERIFICATION="Generate iteration verification report and write report to file: $verif_file

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all verification work directly in current session
- Use only Read, Write, Edit, Grep, LSP, Bash and other direct tools

## Gap Analysis
$(cat $gap_analysis)

## Task List
$(cat $tasks_md)

## Task JSON
$(cat $tasks_json 2>/dev/null || echo "{}")

## Implementation Status
Check ./iterations/src/ directory for code and git commit history

## Output Requirements
Write the complete iteration verification report to: $verif_file

Report must include:
1. P0 issue status (table: Issue | Status | Notes)
2. Constitution compliance check
3. PRD completeness evaluation
4. Remaining issues list
5. Next steps suggestions"

    generate_if_missing "$verif_file" "$PROMPT_VERIFICATION" 5
}

implement_task() {
    local task_id="$1"
    local task_json="$2"
    local spec_file="$3"
    local constitution="$4"

    echo ""
    echo "----------------------------------------------"
    echo "Implementing task: $task_id"
    echo "----------------------------------------------"

    local task_details=$(get_task_details "$task_json" "$task_id")
    echo "Task details:"
    echo "$task_details" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'Title: {d.get(\"title\",\"\")}'); print(f'Priority: {d.get(\"priority\",\"\")}'); print(f'Test criteria: {d.get(\"test_criteria\",\"\")}')" 2>/dev/null || echo "$task_details"

    update_task_status "$task_json" "$task_id" "in_progress"

    echo ""
    echo "Starting implementation..."

    local prompt="Implement task: $task_id

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all implementation work directly in current session
- Use only Read, Write, Edit, Grep, LSP, Bash and other direct tools

## Task Information
$(echo "$task_details" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'ID: {d.get(\"id\",\"\")}'); print(f'Title: {d.get(\"title\",\"\")}'); print(f'Description: {d.get(\"description\",\"\")}'); print(f'Priority: {d.get(\"priority\",\"\")}'); print(f'Test criteria: {chr(10).join(d.get(\"test_criteria\",[]))}'); print(f'Test commands: {chr(10).join(d.get(\"test_commands\",[]))}'); print(f'Implementation notes: {d.get(\"impl_notes\",\"\")}'); print(f'Dependencies: {d.get(\"dependencies\",[])}')" 2>/dev/null || echo "$task_details")

## Spec
$(cat $spec_file)

## Constitution
$(cat $constitution 2>/dev/null || echo "Use default Constitution")

## Implementation Directory
./iterations/src/

## Task
1. Analyze task requirements and test criteria
2. Implement code
3. Run test commands to verify
4. Ensure cargo build and cargo test pass
5. After completion, update task status

## Verification
- Must pass: cargo build
- Must pass: cargo test

## After Completion
1. Update status in task JSON file to done
2. If there is a corresponding Markdown task file, also update status to Done
3. Commit code changes"

    run_opencode_with_session_export "$prompt" "$SESSION_EXPORT_DIR/task_${task_id}.json" "$MODEL"

    echo ""
    echo "Verifying implementation..."

    local task_details_obj=$(echo "$task_details" | python3 -c "import sys,json; print(json.dumps(json.load(sys.stdin)))" 2>/dev/null)
    if [ -n "$task_details_obj" ]; then
        local test_commands=$(echo "$task_details_obj" | python3 -c "import sys,json; print(' '.join(json.load(sys.stdin).get('test_commands', ['cargo build'])))" 2>/dev/null || echo "cargo build")
        echo "Running: $test_commands"
        eval "$test_commands" 2>/dev/null && echo "Tests passed" || echo "Warning: Tests have issues, please check"
    fi

    if [ -n "$(git status --porcelain)" ]; then
        echo ""
        echo "Committing code..."
        git add -A
        git commit -m "impl($task_id): $(echo "$task_details" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('title', d.get('id', 'task'))[:50])" 2>/dev/null || echo "task implementation")"
        echo "Commit completed"
    fi

    update_task_status "$task_json" "$task_id" "done"

    local task_file="${task_json%.json}.md"
    if [ -f "$task_file" ]; then
        sed -i '' "s/^### $task_id:.*/### $task_id: Done/" "$task_file" 2>/dev/null || true
    fi

    echo ""
    echo "Task $task_id completed"
}
