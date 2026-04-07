#!/bin/bash
# Superpowers 迭代脚本 v2.0 - Skills驱动的迭代

set -e

MODEL=${1:-"minimax-cn/MiniMax-M2.7"}
WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"
PRD_PATH="$WORKSPACE_DIR/PRD.md"
OUTPUTS_DIR="$WORKSPACE_DIR/outputs/iteration-2"

mkdir -p "$OUTPUTS_DIR"

echo "=============================================="
echo "Superpowers 迭代开发 v2.0"
echo "=============================================="

echo ""
echo "[1/5] 执行PRD差距分析..."

GAP_ANALYSIS=$(cat << 'GAPEOF'
分析当前实现与PRD的差距：

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

## P0/P1/P2问题分类
## 技术债务清单
GAPEOF
)

opencode run -m "$MODEL" "$GAP_ANALYSIS" > "$OUTPUTS_DIR/gap-analysis.md"
echo "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"

echo ""
echo "[2/5] Brainstorming - 需求理解..."

opencode run -m "$MODEL" "请使用 brainstorming skill 分析差距并深化设计。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 基于差距分析提出澄清问题
2. 提出2-3个方案及权衡
3. 展示设计sections获取批准

## 输出
设计文档保存到: ./outputs/iteration-2/design_v2.md"

echo ""
echo "[3/5] Writing Plans - 创建计划..."

opencode run -m "$MODEL" "请使用 writing-plans skill 创建详细实现计划。

## 设计文档
./outputs/iteration-2/design_v2.md

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 计划要求
1. 分解为2-5分钟可完成的原子任务
2. 每个任务有精确文件路径
3. 使用subagent-driven-development skill进行任务分解
4. 优先P0任务

## 输出
计划保存到: ./outputs/iteration-2/plan_v2.md"

echo ""
echo "[4/5] Subagent-Driven Development..."

opencode run -m "$MODEL" "请使用 subagent-driven-development skill 执行实现。

## 实现计划
./outputs/iteration-2/plan_v2.md

## PRD
$(cat $PRD_PATH)

## 实现目录
./outputs/src/

## 任务
1. 每个任务由fresh subagent执行
2. 两阶段review(规范合规性→代码质量)
3. 优先完成P0任务
4. 确保Build通过

## 验证
- npm run build 必须通过"

echo ""
echo "[5/5] Verification - 验证..."

opencode run -m "$MODEL" "请使用 verification-before-completion skill 进行最终验证。

## 实现产出
检查./outputs/src/目录

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 验证要点
1. P0问题是否全部修复？
2. Build是否通过？
3. 功能是否完整？

## 输出
验证报告保存到: ./outputs/iteration-2/verification-report.md"

echo ""
echo "=============================================="
echo "Superpowers 迭代完成!"
echo "=============================================="
