#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/constitution.sh" 2>/dev/null || true

DEFAULT_MODEL="minimax-cn/MiniMax-M2.7"
MODEL="${MODEL:-$DEFAULT_MODEL}"

get_next_todo_task() {
    local json_file="$1"
    if [ ! -f "$json_file" ]; then
        echo ""
        return
    fi

    local task_id=$(cat "$json_file" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    tasks = data if isinstance(data, list) else data.get('tasks', [])
    for t in tasks:
        if t.get('status') == 'todo':
            print(t.get('id', ''))
            break
except:
    pass
" 2>/dev/null || echo "")

    echo "$task_id"
}

check_remaining_p0_p1() {
    local json_file="$1"
    if [ ! -f "$json_file" ]; then
        echo "0"
        return
    fi

    local remaining=$(cat "$json_file" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    if isinstance(data, list):
        count = sum(1 for t in data if t.get('priority') in ['P0', 'P1'] and t.get('status') != 'done')
        print(count)
    elif isinstance(data, dict) and 'tasks' in data:
        count = sum(1 for t in data['tasks'] if t.get('priority') in ['P0', 'P1'] and t.get('status') != 'done')
        print(count)
    else:
        print('0')
except:
    print('0')
" 2>/dev/null || echo "0")

    echo "${remaining:-0}"
}

has_todo_tasks_json() {
    local json_file="$1"
    if [ ! -f "$json_file" ]; then
        echo "0"
        return
    fi

    local has_todo=$(cat "$json_file" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    tasks = data if isinstance(data, list) else data.get('tasks', [])
    count = sum(1 for t in tasks if t.get('status') == 'todo')
    print(count)
except:
    print('0')
" 2>/dev/null || echo "0")

    echo "${has_todo:-0}"
}

get_task_details() {
    local json_file="$1"
    local task_id="$2"

    cat "$json_file" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    tasks = data if isinstance(data, list) else data.get('tasks', [])
    for t in tasks:
        if t.get('id') == '$task_id':
            print(json.dumps(t, indent=2))
            break
except:
    pass
" 2>/dev/null || echo "{}"
}

update_task_status() {
    local json_file="$1"
    local task_id="$2"
    local new_status="$3"

    if [ ! -f "$json_file" ]; then
        echo "Warning: JSON file not found: $json_file"
        return 1
    fi

    python3 -c "
import json
with open('$json_file', 'r') as f:
    data = json.load(f)

tasks = data if isinstance(data, list) else data.get('tasks', [])
for t in tasks:
    if t.get('id') == '$task_id':
        t['status'] = '$new_status'
        break

with open('$json_file', 'w') as f:
    json.dump(data, f, indent=2)
print('Task $task_id status updated to $new_status')
" 2>/dev/null || echo "Warning: Status update failed"
}

count_todo_tasks() {
    local json_file="$1"
    if [ ! -f "$json_file" ]; then
        echo "0"
        return
    fi

    local count=$(cat "$json_file" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    tasks = data if isinstance(data, list) else data.get('tasks', [])
    print(sum(1 for t in tasks if t.get('status') == 'todo'))
except:
    print('0')
" 2>/dev/null || echo "0")

    echo "${count:-0}"
}

count_done_tasks() {
    local json_file="$1"
    if [ ! -f "$json_file" ]; then
        echo "0"
        return
    fi

    local count=$(cat "$json_file" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    tasks = data if isinstance(data, list) else data.get('tasks', [])
    print(sum(1 for t in tasks if t.get('status') == 'done'))
except:
    print('0')
" 2>/dev/null || echo "0")

    echo "${count:-0}"
}

generate_tasks_json_fallback() {
    local task_file="$1"
    local json_file="$2"

    local first_task=1
    local current_priority=""
    local task_id=""
    local task_desc=""
    local task_lines=""

    process_task() {
        if [[ -z "$task_id" || -z "$current_priority" ]]; then
            return
        fi

        local has_todo=0
        local has_done=0

        while IFS= read -r line; do
            if [[ "$line" =~ ^[[:space:]]*-[[:space:]]*\[ ]]; then
                if [[ "$line" =~ ^[[:space:]]*-[[:space:]]*\[[[:space:]]*\] ]]; then
                    has_todo=1
                else
                    has_done=1
                fi
            fi
        done <<< "$task_lines"

        local status="todo"
        if [[ $has_done -eq 1 && $has_todo -eq 0 ]]; then
            status="done"
        fi

        local desc_json=$(echo "$task_desc" | jq -Rs '.')

        if [[ $first_task -eq 0 ]]; then
            echo ","
        fi
        first_task=0

        echo "    {"
        echo "      \"id\": \"$task_id\","
        echo "      \"priority\": \"$current_priority\","
        echo "      \"title\": \"$task_desc\","
        echo "      \"description\": $desc_json,"
        echo "      \"status\": \"$status\","
        echo "      \"test_criteria\": [\"Code compiles\", \"Tests pass\"],"
        echo "      \"test_commands\": [\"cargo build\"],"
        echo "      \"test_cases\": [],"
        echo "      \"coverage_requirements\": {},"
        echo "      \"impl_notes\": \"\","
        echo "      \"dependencies\": []"
        echo -n "    }"

        task_id=""
        task_desc=""
        task_lines=""
    }

    {
        echo "{"
        echo '  "tasks": ['
        first_task=1

        while IFS= read -r line; do
            if [[ "$line" =~ ^##[[:space:]]*P0 ]]; then
                process_task
                current_priority="P0"
            elif [[ "$line" =~ ^##[[:space:]]*P1 ]]; then
                process_task
                current_priority="P1"
            elif [[ "$line" =~ ^##[[:space:]]*P2 ]]; then
                process_task
                current_priority="P2"
            elif [[ "$line" =~ ^###[[:space:]]*([A-Z][A-Z0-9]*-[0-9][0-9]*):[[:space:]]*(.+) ]]; then
                local _mid="${BASH_REMATCH[1]}" _mdesc="${BASH_REMATCH[2]}"
                process_task
                task_id="$_mid"
                task_desc="$_mdesc"
                task_lines=""
            elif [[ -n "$current_priority" && -n "$task_id" ]]; then
                task_lines="${task_lines}${line}"$'\n'
            fi
        done < "$task_file"

        process_task

        echo ""
        echo "  ]"
        echo "}"
    } > "$json_file"
}

generate_tasks_json() {
    local task_file="$1"
    local json_file="$2"

    if [ -f "$json_file" ] && [ $(wc -c < "$json_file") -gt 10 ]; then
        echo "Skipping JSON generation (exists): $json_file"
        return 0
    fi

    if [ ! -f "$task_file" ]; then
        echo "Warning: Task file not found, cannot generate JSON: $task_file"
        return 1
    fi

    echo "Generating structured task JSON (using LLM): $json_file"

    local prompt="Based on the task Markdown file, generate a structured JSON task file with comprehensive test case specifications.

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Must complete directly in current session

## Task Markdown File
$(cat $task_file)

## Output Requirements
Write JSON to file: $json_file

JSON format must contain the following fields:
{
  \"tasks\": [
    {
      \"id\": \"Task ID (e.g., FR-001)\",
      \"priority\": \"P0|P1|P2\",
      \"title\": \"Task short title\",
      \"description\": \"Task detailed description\",
      \"status\": \"todo|done|in_progress\",
      \"test_criteria\": [\"Test criteria 1\", \"Test criteria 2\"],
      \"test_commands\": [\"cargo test --package <pkg>\", \"npm run build\"],
      \"test_cases\": [
        {
          \"id\": \"TC-001\",
          \"name\": \"Descriptive test name\",
          \"category\": \"unit|integration|render|edge_case\",
          \"description\": \"What this test verifies\",
          \"input\": \"Input markdown or scenario\",
          \"expected_behavior\": \"What the implementation should produce\"
        }
      ],
      \"coverage_requirements\": {
        \"markdown_syntax\": [\"table\", \"strikethrough\", \"horizontal_rule\"],
        \"edge_cases\": [\"empty_cells\", \"unicode_content\", \"nested_markup\"],
        \"browser_features\": [\"clipboard_paste\", \"drag_drop\"]
      },
      \"impl_notes\": \"Implementation notes\",
      \"dependencies\": [\"Dependent task ID\"]
    }
  ]
}

## Requirements
1. Each task must have clearly testable test_criteria
2. test_commands should be concrete commands that can verify task completion
3. dependencies should reference other task IDs (if any)
4. Parse status markers from Markdown (- [ ] = todo, - [x] = done)
5. test_cases MUST be specified for ALL feature tasks - this is critical for coverage
6. For markdown rendering tasks, include test_cases covering:
   - Basic syntax (e.g., basic table, basic bold)
   - Edge cases (empty cells, unicode, special characters)
   - Edge cases (nested markup, malformed input)
7. coverage_requirements should enumerate all markdown syntax or features affected
8. Output must be valid JSON, write directly to file, no other content"

    run_opencode_with_session_export "$prompt" "$SESSION_EXPORT_DIR/tasks_json_${task_file##*/}.json" "$MODEL"

    if [ -f "$json_file" ] && [ $(wc -c < "$json_file") -gt 10 ]; then
        echo "JSON generated successfully"
        return 0
    else
        echo "LLM JSON generation failed, trying script parsing..."
        generate_tasks_json_fallback "$task_file" "$json_file"
    fi
}
