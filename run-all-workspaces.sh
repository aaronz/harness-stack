#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MODEL="${1:-opencode/minimax-m2.5-free}"
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