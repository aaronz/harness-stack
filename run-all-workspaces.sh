#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MODEL="${1:-opencode/minimax-m2.5-free}"
WORKSPACES=(
    "workspace/Workspace-openspec"
    "workspace/Workspace-speckit"
    "workspace/Workspace-superpowers"
    "workspace/Workspace-everything-claude-code"
    "workspace/Workspace-planning-with-files"
    "workspace/Workspace-gstack"
)

echo "========================================"
echo "Running PRD Implementation for All Workspaces"
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
echo "All Workspaces Complete!"
echo "========================================"