#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MODEL="${1:-opencode/minimax-m2.5-free}"

LAST_ITERATION=$(ls -d "$SCRIPT_DIR/workspace/*/outputs/iteration-"* 2>/dev/null | sed 's/.*iteration-//' | sort -n | tail -1 || echo "0")
NEXT_ITERATION=$((LAST_ITERATION + 1))

WORKSPACES=(
    "workspace/workspace-openspec"
    "workspace/workspace-speckit"
    "workspace/workspace-superpowers"
    "workspace/workspace-everything-claude-code"
    "workspace/workspace-planning-with-files"
    "workspace/workspace-gstack"
)

echo "========================================"
echo "Running PRD Implementation for All workspaces"
echo "迭代: ${NEXT_ITERATION}"
echo "模型: $MODEL"
echo "========================================"
echo ""

for workspace in "${WORKSPACES[@]}"; do
    workspace_path="$SCRIPT_DIR/$workspace"
    script_path="$workspace_path/implement-prd.sh"
    
    if [ -f "$script_path" ]; then
        echo "========================================"
        echo "Running: $workspace"
        echo "========================================"
        cd "$workspace_path"
        bash "$script_path" "$MODEL"
        echo ""
    else
        echo "Warning: $script_path not found, skipping"
    fi
done

echo "========================================"
echo "All workspaces Complete!"
echo "========================================"
echo ""
echo "各 workspace 产出目录:"
for workspace in "${WORKSPACES[@]}"; do
    workspace_name=$(basename "$workspace")
    echo "  - $workspace_name/outputs/iteration-${NEXT_ITERATION}/"
done