#!/bin/bash
# Everything Claude Code 迭代脚本 v2.0 - Commands驱动的迭代

set -e

MODEL=${1:-"opencode/minimax-m2.5-free"}
WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"
PRD_PATH="$WORKSPACE_DIR/PRD.md"
OUTPUTS_DIR="$WORKSPACE_DIR/outputs/iteration-2"

mkdir -p "$OUTPUTS_DIR"

echo "=============================================="
echo "Everything Claude Code 迭代开发 v2.0"
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
echo "[2/5] Plan - 创建计划..."

opencode run -m "$MODEL" "使用 /plan 命令创建实现计划。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 基于差距分析创建详细计划
2. 优先P0任务
3. 明确文件路径和依赖

## 输出
计划保存到: ./outputs/iteration-2/plan_v2.md"

echo ""
echo "[3/5] Execute - 执行实现..."

opencode run -m "$MODEL" "执行实现。

## 计划
./outputs/iteration-2/plan_v2.md

## PRD
$(cat $PRD_PATH)

## 实现目录
./outputs/src/

## 任务
1. 按计划执行实现
2. 优先完成P0任务
3. 确保Build通过

## 验证
- npm run build 必须通过"

echo ""
echo "[4/5] TDD - 测试驱动（如需要）..."

opencode run -m "$MODEL" "使用 /tdd 命令为P0功能编写测试。

## 计划
./outputs/iteration-2/plan_v2.md

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 为P0功能编写单元测试
2. 确保测试通过
3. 如无测试必要，跳过此步骤"

echo ""
echo "[5/5] Verify & Review - 验证..."

opencode run -m "$MODEL" "使用 /verify 和 /code-review 命令进行验证。

## 计划
./outputs/iteration-2/plan_v2.md

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 实现状态
检查./outputs/src/目录

## 输出格式
# 迭代验证报告

## P0问题状态
| 问题 | 状态 | 备注 |

## PRD完整度
[百分比]

## 遗留问题
## 下一步建议

## 输出
验证报告保存到: ./outputs/iteration-2/verification-report.md"

echo ""
echo "=============================================="
echo "Everything Claude Code 迭代完成!"
echo "=============================================="
