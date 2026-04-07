#!/bin/bash
# GStack 迭代脚本 v2.0 - 7步审查后迭代

set -e

MODEL=${1:-"opencode/minimax-m2.5-free"}
WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"
PRD_PATH="$WORKSPACE_DIR/PRD.md"
OUTPUTS_DIR="$WORKSPACE_DIR/outputs/iteration-2"
IMPL_DIR="$WORKSPACE_DIR/outputs/worktrees/ai-ready-evaluator"

mkdir -p "$OUTPUTS_DIR"

echo "=============================================="
echo "GStack 迭代开发 v2.0"
echo "=============================================="

echo ""
echo "[1/6] 执行PRD差距分析..."

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

## P0问题（必须修复）
...

## P1问题（应该修复）
...

## P2问题（可以修复）
...

## 技术债务
...
GAPEOF
)

opencode run -m "$MODEL" "$GAP_ANALYSIS" > "$OUTPUTS_DIR/gap-analysis.md"
echo "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"

echo ""
echo "[2/6] 生成增量文档..."

INCREMENT=$(cat << 'INCEOF'
# GStack 迭代增量文档 v2.0

## 一、迭代目标
[基于差距分析的核心目标]

## 二、待实现功能
### P0
- [ ] [功能]

### P1
- [ ] [功能]

## 三、技术债务
- [ ] [债务项] - [修复建议]

## 四、验收标准
1. [具体条件]
INCEOF
)

opencode run -m "$MODEL" "$(echo "$INCREMENT"; cat "$OUTPUTS_DIR/gap-analysis.md")" > "$OUTPUTS_DIR/increment.md"
echo "增量文档完成: $OUTPUTS_DIR/increment.md"

echo ""
echo "[3/6] Office Hours - 需求理解..."

opencode run -m "$MODEL" "使用 /office-hours 命令进行需求理解深化。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 要求
重点关注差距分析中识别的P0问题，重新审视产品设计。

## 输出
更新设计文档到: ./outputs/iteration-2/design-v2.md"

echo ""
echo "[4/6] CEO Review + Eng Review..."

opencode run -m "$MODEL" "使用 /plan-ceo-review 和 /plan-eng-review 命令进行审查。

## 增量文档
$(cat $OUTPUTS_DIR/increment.md)

## 现有设计
./outputs/design.md

## 审查重点
1. 架构调整是否合理？
2. 数据持久化方案
3. 多Provider支持方案

## 输出
审查结果保存到: ./outputs/iteration-2/review-v2.md"

echo ""
echo "[5/6] 执行实现..."

opencode run -m "$MODEL" "使用 gstack 的实现模式执行迭代开发。

## 增量文档
$(cat $OUTPUTS_DIR/increment.md)

## 审查结果
./outputs/iteration-2/review-v2.md

## 目录
outputs/worktrees/ai-ready-evaluator/

## 任务
1. 优先实现P0功能
2. 确保核心功能完整
3. 保持代码风格一致
4. 添加必要的错误处理

## 验证
- npm run build 必须通过"

echo ""
echo "[6/6] 验证与QA..."

opencode run -m "$MODEL" "使用 /review 和 /qa 命令进行验证。

## 代码审查
./outputs/iteration-2/review-v2.md

## 要求
1. 找到所有P0问题的修复
2. 确认build通过
3. 生成验证报告

## 输出
验证报告保存到: ./outputs/iteration-2/verification-report.md"

echo ""
echo "=============================================="
echo "GStack 迭代完成!"
echo "=============================================="
