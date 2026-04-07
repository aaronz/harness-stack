#!/bin/bash
# 批量运行所有Workspace迭代脚本

set -e

MODEL=${1:-"opencode/minimax-m2.5-free"}
WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=============================================="
echo "批量执行所有Workspace迭代开发"
echo "=============================================="
echo "使用模型: $MODEL"
echo ""

ITERATION_ORDER=(
    "workspace-openspec"
    "workspace-gstack"
    "workspace-planning-with-files"
    "workspace-speckit"
    "workspace-superpowers"
    "workspace-everything-claude-code"
)

for workspace in "${ITERATION_ORDER[@]}"; do
    workspace_path="$WORKSPACE_DIR/$workspace"
    
    if [ -f "$workspace_path/iterate-prd.sh" ]; then
        echo ""
        echo "##############################################"
        echo "# 迭代: $workspace"
        echo "##############################################"
        
        cd "$workspace_path"
        bash ./iterate-prd.sh "$MODEL"
        
        echo ""
        echo "[完成] $workspace 迭代"
    else
        echo "[跳过] $workspace - 迭代脚本不存在"
    fi
done

echo ""
echo "=============================================="
echo "全部Workspace迭代完成!"
echo "=============================================="
