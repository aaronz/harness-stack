# GStack 迭代脚本 v2.0 - 通用PRD迭代框架
param(
    [string]$Model = "opencode/minimax-m2.5-free"
)

$WorkspaceDir = $PSScriptRoot
$PrdPath = "$WorkspaceDir\PRD.md"
$OutputsDir = "$WorkspaceDir\outputs\iteration-2"
$ImplDir = "$WorkspaceDir\outputs\worktrees\ai-ready-evaluator"

Write-Host "==============================================" -ForegroundColor Cyan
Write-Host "GStack 迭代开发 v2.0" -ForegroundColor Cyan
Write-Host "==============================================" -ForegroundColor Cyan

Write-Host ""
Write-Host "[1/6] 执行PRD差距分析..." -ForegroundColor Yellow

$GapAnalysis = @"
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
"@

$PrdContent = Get-Content $PrdPath -Raw
opencode run -m $Model "$GapAnalysis" | Out-File -FilePath "$OutputsDir\gap-analysis.md" -Encoding UTF8
Write-Host "差距分析完成: $OutputsDir\gap-analysis.md" -ForegroundColor Green

Write-Host ""
Write-Host "[2/6] 生成增量文档..." -ForegroundColor Yellow

$Increment = @"
# 迭代增量文档 v2.0

## 一、迭代目标
基于差距分析，列出本轮迭代的核心目标。

## 二、待实现功能清单
### P0 (必须实现)
- [ ] [功能名称]

### P1 (应该实现)
- [ ] [功能名称]

### P2 (可以实现)
- [ ] [功能名称]

## 三、技术债务清单
- [ ] [债务项] - [修复建议]

## 四、验收标准
1. [具体可验证的验收条件]
2. [Build必须通过]
3. [功能必须可用]

## 五、风险与依赖
- [已识别的风险] - [应对策略]
"@

$GapContent = Get-Content "$OutputsDir\gap-analysis.md" -Raw
opencode run -m $Model "$Increment`n`n## 差距分析结果`n$GapContent" | Out-File -FilePath "$OutputsDir\increment.md" -Encoding UTF8
Write-Host "增量文档完成: $OutputsDir\increment.md" -ForegroundColor Green

Write-Host ""
Write-Host "[3/6] Office Hours - 需求理解..." -ForegroundColor Yellow
opencode run -m $Model "使用 /office-hours 命令进行需求理解深化。`n`n## PRD`n$PrdContent`n`n## 差距分析`n$GapContent`n`n## 要求`n重点关注差距分析中识别的P0问题，重新审视产品设计。`n`n## 输出`n更新设计文档到: ./outputs/iteration-2/design-v2.md"

Write-Host ""
Write-Host "[4/6] CEO Review + Eng Review..." -ForegroundColor Yellow
opencode run -m $Model "使用 /plan-ceo-review 和 /plan-eng-review 命令进行审查。`n`n## 增量文档`n$(Get-Content "$OutputsDir\increment.md" -Raw)`n`n## 现有设计`n./outputs/design.md`n`n## 审查重点`n基于增量文档中的P0问题，重点审查架构调整、实现方案和遗漏依赖。`n`n## 输出`n审查结果保存到: ./outputs/iteration-2/review-v2.md"

Write-Host ""
Write-Host "[5/6] 执行实现..." -ForegroundColor Yellow
opencode run -m $Model "使用 gstack 的实现模式执行迭代开发。`n`n## 增量文档`n$(Get-Content "$OutputsDir\increment.md" -Raw)`n`n## 审查结果`n./outputs/iteration-2/review-v2.md`n`n## 目录`noutputs/worktrees/ai-ready-evaluator/`n`n## 任务`n1. 优先实现P0功能`n2. 确保核心功能完整`n3. 保持代码风格一致`n4. 添加必要的错误处理`n`n## 验证`nnpm run build 必须通过"

Write-Host ""
Write-Host "[6/6] 验证与QA..." -ForegroundColor Yellow
opencode run -m $Model "使用 /review 和 /qa 命令进行验证。`n`n## 代码审查`n./outputs/iteration-2/review-v2.md`n`n## 要求`n1. 确认所有P0问题已修复`n2. npm run build 必须通过`n3. 核心功能必须可用`n`n## 输出`n验证报告保存到: ./outputs/iteration-2/verification-report.md"

Write-Host ""
Write-Host "==============================================" -ForegroundColor Green
Write-Host "GStack 迭代完成!" -ForegroundColor Green
Write-Host "==============================================" -ForegroundColor Green
