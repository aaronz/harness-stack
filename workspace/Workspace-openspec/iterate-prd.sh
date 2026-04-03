#!/bin/bash
# OpenSpec 迭代脚本 v2.0
# 对比实现与PRD差距，输出增量文档，执行迭代开发

set -e

MODEL=${1:-"opencode/minimax-m2.5-free"}
WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"
PRD_PATH="$WORKSPACE_DIR/PRD.md"
IMPLEMENTATION_DIR="$WORKSPACE_DIR/outputs/proposal"
OUTPUTS_DIR="$WORKSPACE_DIR/outputs/iteration-2"

mkdir -p "$OUTPUTS_DIR"

echo "=============================================="
echo "OpenSpec 迭代开发 v2.0"
echo "=============================================="

# 读取PRD内容
echo "[1/5] 读取PRD文档..."
PRD_CONTENT=$(cat "$PRD_PATH")

# Step 1: Gap Analysis
echo ""
echo "[2/5] 执行PRD差距分析..."
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
echo "差距分析完成: $OUTPUTS_DIR/gap_analysis.md"

# Step 2: Generate Increment Document
echo ""
echo "[3/5] 生成增量文档..."
INCREMENT=$(cat << 'INCEOF'
基于差距分析，生成第二轮迭代的增量文档：

## 输出格式
# AI-Ready Evaluator 迭代增量文档 v2.0

## 一、迭代目标
[基于差距分析，列出本轮迭代的核心目标]

## 二、待实现功能清单
### P0 (必须实现)
- [ ] [功能名称] - [差距描述] - [预估工时]

### P1 (应该实现)
- [ ] [功能名称] - [差距描述] - [预估工时]

### P2 (可以实现)
- [ ] [功能名称] - [差距描述] - [预估工时]

## 三、技术债务清单
- [ ] [债务项] - [原因] - [修复建议]

## 四、验收标准
1. [具体可验证的验收条件]

## 五、风险与依赖
- [已识别的风险] - [应对策略]
INCEOF
)

opencode run -m "$MODEL" "$(echo "$INCREMENT"; echo ""; echo "## 差距分析结果"; cat "$OUTPUTS_DIR/gap-analysis.md")" > "$OUTPUTS_DIR/increment.md"
echo "增量文档完成: $OUTPUTS_DIR/increment.md"

# Step 3: Execute Iteration
echo ""
echo "[4/5] 执行迭代开发..."
ITERATION_TASK=$(cat << 'ITEOF'
基于增量文档，执行第二轮开发：

## 增量文档位置
./outputs/iteration-2/increment.md

## PRD位置
./PRD.md

## 实现目录
./outputs/proposal/

## 任务
1. 优先实现P0级别的功能
2. 确保所有核心功能完整
3. 保持代码风格一致
4. 更新相应的文档

## 验证要求
- 代码必须通过 TypeScript 编译
- 必须有基本的错误处理
- 遵循原有架构模式

## 输出
完成后，更新 ./outputs/iteration-2/increment.md，标记已完成的任务。
ITEOF
)

opencode run -m "$MODEL" "$ITERATION_TASK"
echo "迭代开发完成"

# Step 4: Generate Verification Report
echo ""
echo "[5/5] 生成验证报告..."
VERIFY=$(cat << 'VEOF'
验证第二轮迭代的产出：

## 验证清单
1. 检查所有P0功能是否已实现
2. 对比 PRD.md 的完整度
3. 检查代码质量
4. 生成验证报告

## 输出格式
# 迭代验证报告

## 实现状态
| 功能 | 状态 | 备注 |
|------|------|------|

## PRD完整度
[计算当前实现与PRD的匹配度百分比]

## 遗留问题
- [ ] [问题] - [优先级] - [建议]

## 下一步建议
1. [具体建议]
VEOF
)

opencode run -m "$MODEL" "$(echo "$VEOF"; echo ""; echo "## 当前实现"; find ./outputs/proposal/packages -name "*.ts" -o -name "*.tsx" | head -20)" > "$OUTPUTS_DIR/verification-report.md"
echo "验证报告完成: $OUTPUTS_DIR/verification-report.md"

echo ""
echo "=============================================="
echo "迭代开发完成!"
echo "=============================================="
echo "产出目录: $OUTPUTS_DIR"
echo "- gap-analysis.md: 差距分析"
echo "- increment.md: 增量文档"
echo "- verification-report.md: 验证报告"
