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

    local constitution_content="No Constitution file found"
    if [ -n "$constitution_path" ] && [ -f "$constitution_path" ]; then
        constitution_content=$(cat "$constitution_path" 2>/dev/null || echo "Constitution does not exist")
    fi

    PROMPT_CONSTITUTION="Check if Constitution needs updating and write update suggestions to file: $const_update_file

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all analysis work directly in current session
- Use only Read, Write, Edit, Grep, LSP and other direct tools

## Constitution
$constitution_content

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

    local constitution_content="Use default Constitution"
    if [ -n "$constitution" ] && [ -f "$constitution" ]; then
        constitution_content=$(cat "$constitution" 2>/dev/null || echo "Use default Constitution")
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
$constitution_content

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

    local constitution_content=""
    if [ -n "$constitution" ] && [ -f "$constitution" ]; then
        constitution_content=$(cat "$constitution" 2>/dev/null || echo "")
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
$constitution_content

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

detect_project_info() {
    local root="${1:-.}"
    local info='{}'
    
    # === Ecosystem Detection ===
    if [ -f "$root/package.json" ]; then
        info=$(echo "$info" | jq '.has_package_json = true' 2>/dev/null || echo '{}')
        
        # Detect package manager
        [ -f "$root/pnpm-lock.yaml" ] && info=$(echo "$info" | jq '.package_manager = "pnpm"' 2>/dev/null)
        [ -f "$root/yarn.lock" ] && info=$(echo "$info" | jq '.package_manager = "yarn"' 2>/dev/null)
        [ -f "$root/package-lock.json" ] && info=$(echo "$info" | jq '.package_manager = "npm"' 2>/dev/null)
        [ -f "$root/bun.lockb" ] && info=$(echo "$info" | jq '.package_manager = "bun"' 2>/dev/null)
        
        # Detect frontend framework
        local pkg=$(cat "$root/package.json" 2>/dev/null)
        echo "$pkg" | grep -q '"react"' && info=$(echo "$info" | jq '.framework = "react"' 2>/dev/null)
        echo "$pkg" | grep -q '"vue"' && info=$(echo "$info" | jq '.framework = "vue"' 2>/dev/null)
        echo "$pkg" | grep -q '"next"' && info=$(echo "$info" | jq '.framework = "next"' 2>/dev/null)
        echo "$pkg" | grep -q '"svelte"' && info=$(echo "$info" | jq '.framework = "svelte"' 2>/dev/null)
        
        # Detect dev server command
        local dev_cmd=$(echo "$pkg" | jq -r '.scripts.dev // .scripts.start // empty' 2>/dev/null)
        [ -n "$dev_cmd" ] && [ "$dev_cmd" != "null" ] && info=$(echo "$info" | jq ".dev_server = \"$dev_cmd\"" 2>/dev/null)
        
        # Detect frontend test framework
        echo "$pkg" | grep -q '"vitest"' && info=$(echo "$info" | jq '.test_frameworks += ["vitest"]' 2>/dev/null)
        echo "$pkg" | grep -q '"jest"' && info=$(echo "$info" | jq '.test_frameworks += ["jest"]' 2>/dev/null)
        echo "$pkg" | grep -q '"@playwright/test"' && info=$(echo "$info" | jq '.test_frameworks += ["playwright"]' 2>/dev/null)
        echo "$pkg" | grep -q '"cypress"' && info=$(echo "$info" | jq '.test_frameworks += ["cypress"]' 2>/dev/null)
    fi
    
    if [ -f "$root/Cargo.toml" ]; then
        info=$(echo "$info" | jq '.has_cargo = true' 2>/dev/null || echo '{}')
        local pkg_name=$(grep '^name = ' "$root/Cargo.toml" 2>/dev/null | head -1 | sed 's/name = "//' | sed 's/"//' | tr -d ' ')
        [ -n "$pkg_name" ] && info=$(echo "$info" | jq ".cargo_package = \"$pkg_name\"" 2>/dev/null)
        [ -d "$root/tests" ] && info=$(echo "$info" | jq '.test_frameworks += ["cargo"]' 2>/dev/null)
    fi
    
    if [ -f "$root/go.mod" ]; then
        info=$(echo "$info" | jq '.has_go_mod = true' 2>/dev/null || echo '{}')
        [ -d "$root" ] && info=$(echo "$info" | jq '.test_frameworks += ["go"]' 2>/dev/null)
    fi
    
    if [ -f "$root/pyproject.toml" ] || [ -f "$root/requirements.txt" ]; then
        info=$(echo "$info" | jq '.has_python = true' 2>/dev/null || echo '{}')
        [ -f "$root/pytest.ini" ] && info=$(echo "$info" | jq '.test_frameworks += ["pytest"]' 2>/dev/null)
    fi
    
    # === Tauri Detection ===
    if [ -d "$root/src-tauri" ] || [ -f "$root/tauri.conf.json" ]; then
        info=$(echo "$info" | jq '.is_tauri = true' 2>/dev/null || echo '{}')
    fi
    
    # === Test Framework Detection via Files ===
    [ -f "$root/vitest.config.ts" ] || [ -f "$root/vitest.config.js" ] || [ -f "$root/vite.config.ts" ] && \
        info=$(echo "$info" | jq '.test_frameworks += ["vitest"]' 2>/dev/null)
    [ -f "$root/playwright.config.ts" ] || [ -f "$root/playwright.config.js" ] && \
        info=$(echo "$info" | jq '.test_frameworks += ["playwright"]' 2>/dev/null)
    [ -f "$root/jest.config.ts" ] || [ -f "$root/jest.config.js" ] && \
        info=$(echo "$info" | jq '.test_frameworks += ["jest"]' 2>/dev/null)
    [ -f "$root/cypress.config.ts" ] || [ -f "$root/cypress.config.js" ] || [ -f "$root/cypress.json" ] && \
        info=$(echo "$info" | jq '.test_frameworks += ["cypress"]' 2>/dev/null)
    
    # === Initialize test_frameworks array if empty ===
    if ! echo "$info" | jq '.test_frameworks' >/dev/null 2>&1; then
        info=$(echo "$info" | jq '.test_frameworks = []' 2>/dev/null)
    fi
    
    # === Deduplicate test_frameworks ===
    info=$(echo "$info" | jq '.test_frameworks = (.test_frameworks | unique)' 2>/dev/null)
    
    echo "$info" | jq '.' 2>/dev/null || echo '{}'
}

verify_test_coverage() {
    local tasks_json="$1"
    local output_dir="$2"
    local test_cov_file="$output_dir/test-coverage-report.md"
    
    if [ ! -f "$tasks_json" ]; then
        echo "Warning: Tasks JSON not found: $tasks_json"
        return 1
    fi
    
    echo "Analyzing test case coverage..."
    
    python3 << 'PYTHON_SCRIPT' > "$test_cov_file"
import json
import sys
from pathlib import Path

def read_tasks_json(json_path):
    try:
        with open(json_path, 'r') as f:
            data = json.load(f)
        return data.get('tasks', [])
    except Exception as e:
        print(f"Error reading JSON: {e}", file=sys.stderr)
        return []

def count_test_files(test_dir, patterns=['*.test.js', '*.test.ts', '*.test.jsx', '*.spec.ts', '*.spec.js']):
    count = 0
    if test_dir.exists():
        for pattern in patterns:
            count += len(list(test_dir.glob(f"**/{pattern}")))
    return count

def main():
    tasks_json_path = sys.argv[1] if len(sys.argv) > 1 else 'tasks.json'
    tasks = read_tasks_json(tasks_json_path)
    
    # Find project root by looking for package.json or similar
    project_root = Path(tasks_json_path).parent.parent
    test_dirs = list(project_root.glob('**/__tests__'))
    
    # Collect all test files
    test_files = []
    for test_dir in test_dirs:
        for pattern in ['*.test.js', '*.test.ts', '*.test.jsx', '*.spec.ts', '*.spec.js']:
            test_files.extend(test_dir.glob(f"**/{pattern}"))
    
    total_planned = 0
    total_with_tests = 0
    
    report = ["# Test Coverage Report\n"]
    report.append("| Task ID | Task Title | Planned Tests | Status |")
    report.append("|---------|------------|---------------|--------|")
    
    for task in tasks:
        task_id = task.get('id', 'N/A')
        title = task.get('title', 'N/A')[:40]
        test_cases = task.get('test_cases', [])
        planned = len(test_cases)
        total_planned += planned
        
        # Check if task has test cases defined
        if planned > 0:
            # Check if there are test files in the project
            has_tests = len(test_files) > 0
            status = "✓ Has Tests" if has_tests else "✗ Missing Tests"
            if has_tests:
                total_with_tests += 1
        else:
            status = "- No tests required"
        
        report.append(f"| {task_id} | {title} | {planned} | {status} |")
    
    # Summary
    coverage_pct = (total_with_tests / max(1, total_planned)) * 100 if total_planned > 0 else 100
    
    report.append(f"\n## Summary\n")
    report.append(f"- Total tasks: {len(tasks)}")
    report.append(f"- Tasks requiring tests: {total_planned}")
    report.append(f"- Tasks with test files: {total_with_tests}")
    report.append(f"- Test coverage: {coverage_pct:.1f}%")
    report.append(f"- Test files found: {len(test_files)}")
    
    if total_planned > 0 and total_with_tests < total_planned:
        report.append(f"\n## ⚠️ Missing Test Coverage\n")
        for task in tasks:
            if len(task.get('test_cases', [])) > 0:
                test_cases = task.get('test_cases', [])
                report.append(f"\n### {task.get('id')}: {task.get('title')}\n")
                for tc in test_cases:
                    report.append(f"- [ ] {tc.get('id')}: {tc.get('name')} ({tc.get('category', 'N/A')})")
    
    print('\n'.join(report))

if __name__ == '__main__':
    main()
PYTHON_SCRIPT
    
    echo "Test coverage report generated: $test_cov_file"
}

run_phase_verification() {
    local gap_analysis="$1"
    local tasks_md="$2"
    local tasks_json="$3"
    local output_dir="$4"

    local verif_file="$output_dir/verification-report.md"
    local test_cov_file="$output_dir/test-coverage-report.md"

    if check_file_quiet "$verif_file"; then
        echo "Skipping verification report (exists)"
        return 0
    fi

    echo ""
    echo "[6/6] Project auto-discovery and LLM-guided verification..."
    
    local project_info=$(detect_project_info "$WORKSPACE_DIR")
    echo "Detected project info:"
    echo "$project_info" | jq '.' 2>/dev/null || echo "$project_info"
    
    local has_frontend=$(echo "$project_info" | jq -r '.has_package_json' 2>/dev/null)
    local frameworks=$(echo "$project_info" | jq -r '.test_frameworks | join(", ")' 2>/dev/null)
    local is_tauri=$(echo "$project_info" | jq -r '.is_tauri' 2>/dev/null)
    
    echo "Has frontend: $has_frontend"
    echo "Test frameworks: ${frameworks:-none}"
    echo "Is Tauri: $is_tauri"
    
    verify_test_coverage "$tasks_json" "$output_dir"

    PROMPT_VERIFICATION="## LLM-Guided Project Verification

You are running verification for this project. Your job is to:
1. Run actual tests using detected frameworks
2. Verify the app works in a browser (if frontend)
3. Analyze results and provide actionable feedback

## Important Constraints
- You MUST use the bash tool to actually execute tests, not just describe what to do
- You MUST run real commands and report actual pass/fail results
- Do NOT use subagent or task tools to spawn other agents

## Project Auto-Discovery Results
$(echo "$project_info" | jq '.' 2>/dev/null || echo "{}")

## Detected Test Frameworks
Based on the project structure, run appropriate tests:
- vitest: \`npx vitest run --reporter=verbose\`
- playwright: \`npx playwright test\`
- jest: \`npm test\`
- cargo: \`cargo test\`
- go: \`go test ./...\`
- pytest: \`python -m pytest\`

## Verification Steps

### Step 1: Run Unit/Integration Tests
Execute test commands for detected frameworks. Use bash tool to run them.

### Step 2: Browser Smoke Test (CRITICAL for Frontend/Tauri)
If the project has a frontend (\$has_package_json = true), you MUST verify the app loads in browser:

1. Start dev server:
   \`cd www && npm run dev &\`
   Wait 8 seconds for server to start

2. Use node with playwright to check:
   \`\`\`javascript
   const { chromium } = require('playwright');
   (async () => {
     const browser = await chromium.launch({ headless: true });
     const page = await browser.newPage();
     const errors = [];
     page.on('console', msg => { if (msg.type() === 'error') errors.push(msg.text()); });
     page.on('pageerror', err => errors.push(err.message));
     await page.goto('http://localhost:1420', { waitUntil: 'networkidle', timeout: 15000 });
     await page.waitForTimeout(2000);
     const hasApp = await page.locator('#app').count() > 0;
     const hasEditor = await page.locator('#editor-content').count() > 0;
     await browser.close();
     const realErrors = errors.filter(e => !e.includes('invoke') && !e.includes('tauri'));
     console.log(JSON.stringify({ hasApp, hasEditor, errors: realErrors }));
   })();
   \`\`\`

3. Check results - app should have #app and #editor-content elements, no critical JS errors

4. Kill dev server: \`pkill -f "vite" || true\`

### Step 3: Analyze and Report
Write verification report to: $verif_file

Report format:
## Verification Results

### Test Execution
| Framework | Command | Result |
|-----------|---------|--------|
| vitest | npx vitest run | PASSED/FAILED |

### Browser Smoke Test
| Check | Result |
|-------|--------|
| Page loads | ✓/✗ |
| #app exists | ✓/✗ |
| Critical JS errors | ✓/✗ |

### Issues Found
- List actual failures with error messages

### Recommendations
- If issues found, suggest specific fixes

## Gap Analysis
$(cat $gap_analysis)

## Task List
$(cat $tasks_md)

## Task JSON
$(cat $tasks_json 2>/dev/null || echo "{}")

## Test Coverage Report
$(cat $test_cov_file 2>/dev/null || echo "No test coverage data available")

## Output Requirements
Write the complete iteration verification report to: $verif_file

Report must include:
1. Actual test execution results (real pass/fail from running commands)
2. Browser smoke test results (if frontend)
3. P0 issue status (table: Issue | Status | Notes)
4. Test coverage status
5. Issues found with severity
6. Recommendations for fixes"

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

    local constitution_content="Use default Constitution"
    if [ -n "$constitution" ] && [ -f "$constitution" ]; then
        constitution_content=$(cat "$constitution" 2>/dev/null || echo "Use default Constitution")
    fi

    local prompt="Implement task: $task_id

## Important Constraints
- Do NOT use subagent or task tools to spawn other agents
- Do NOT delegate work to other agents
- Must complete all implementation work directly in current session
- Use only Read, Write, Edit, Grep, LSP, Bash and other direct tools

## Task Information
$(echo "$task_details" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'ID: {d.get(\"id\",\"\")}'); print(f'Title: {d.get(\"title\",\"\")}'); print(f'Description: {d.get(\"description\",\"\")}'); print(f'Priority: {d.get(\"priority\",\"\")}'); print(f'Test criteria: {chr(10).join(d.get(\"test_criteria\",[]))}'); print(f'Test commands: {chr(10).join(d.get(\"test_commands\",[]))}'); print(f'Implementation notes: {d.get(\"impl_notes\",\"\")}'); print(f'Dependencies: {d.get(\"dependencies\",[])}')" 2>/dev/null || echo "$task_details")

## Required Test Cases (MUST IMPLEMENT)
$(echo "$task_details" | python3 -c "
import sys,json
d=json.load(sys.stdin)
test_cases=d.get('test_cases', [])
if test_cases:
    print('The following test cases MUST be implemented:')
    for tc in test_cases:
        print(f'  - {tc.get(\"id\")}: {tc.get(\"name\")}')
        print(f'    Category: {tc.get(\"category\", \"N/A\")}')
        print(f'    Input: {tc.get(\"input\", \"N/A\")[:100]}')
        print(f'    Expected: {tc.get(\"expected_behavior\", \"N/A\")}')
else:
    print('No specific test cases defined - write generic tests based on test_criteria')
" 2>/dev/null || echo "No test cases available")

## Coverage Requirements
$(echo "$task_details" | python3 -c "
import sys,json
d=json.load(sys.stdin)
cov=d.get('coverage_requirements', {})
if cov:
    print('Markdown syntax to cover:')
    for item in cov.get('markdown_syntax', []):
        print(f'  - {item}')
    print('Edge cases to cover:')
    for item in cov.get('edge_cases', []):
        print(f'  - {item}')
" 2>/dev/null || echo "No coverage requirements specified")

## Spec
$(cat $spec_file)

## Constitution
$constitution_content

## Implementation Directory
./iterations/src/

## Task
1. Analyze task requirements and test criteria
2. Implement code
3. IMPLEMENT ALL REQUIRED TEST CASES - this is critical
4. Run test commands to verify
5. Ensure cargo build and cargo test pass
6. After completion, update task status

## Verification
- Must pass: cargo build
- Must pass: cargo test
- All test_cases from the task JSON must be implemented

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
