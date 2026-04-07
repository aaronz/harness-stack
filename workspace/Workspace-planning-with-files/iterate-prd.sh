#!/bin/bash
# Planning with Files 迭代脚本 v2.0 - 通用PRD迭代框架

set -e

MODEL=${1:-"opencode/minimax-m2.5-free"}
WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"
PRD_PATH="$WORKSPACE_DIR/PRD.md"
OUTPUTS_DIR="$WORKSPACE_DIR/outputs/iteration-2"
IMPL_DIR="$WORKSPACE_DIR/outputs"

mkdir -p "$OUTPUTS_DIR"

echo "=============================================="
echo "Planning with Files 迭代开发 v2.0"
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
echo "[2/5] 更新任务计划..."

opencode run -m "$MODEL" "更新任务计划文档。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 现有任务计划
./outputs/task_plan.md

## 任务
1. 基于差距分析，更新task_plan.md
2. 添加P0/P1/P2任务清单
3. 更新进度追踪

## 输出
更新后的任务计划保存到: ./outputs/iteration-2/task_plan_v2.md"

echo ""
echo "[3/5] 执行增量开发..."

opencode run -m "$MODEL" "基于任务计划执行迭代开发。

## 任务计划
./outputs/iteration-2/task_plan_v2.md

## PRD
$(cat $PRD_PATH)

## 实现目录
./outputs/

## 任务
1. 优先实现P0任务
2. 更新进度到progress.md
3. 添加新发现到findings.md
4. 确保Build通过

## 验证
- npm run build 必须通过"

echo ""
echo "[4/5] 更新发现文档..."

opencode run -m "$MODEL" "更新findings.md文档。

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务执行情况
./outputs/progress.md

## 任务
1. 记录新发现的问题
2. 记录解决方案
3. 更新LLM Provider对比分析（如有）

## 输出
更新后的发现保存到: ./outputs/iteration-2/findings_v2.md"

echo ""
echo "[5/5] 生成验证报告..."

opencode run -m "$MODEL" "生成迭代验证报告。

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务计划
./outputs/iteration-2/task_plan_v2.md

## 实现状态
检查./outputs/目录下的代码

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
echo "Planning with Files 迭代完成!"
echo "=============================================="
