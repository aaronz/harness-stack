$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PrdFile = Join-Path $ScriptDir "..\..\PRD.md"

$Model = if ($args.Count -gt 0) { $args[0] } else { "opencode/minimax-m2.5-free" }

if (-not (Test-Path $PrdFile)) {
    Write-Host "Error: PRD.md not found at $PrdFile" -ForegroundColor Red
    exit 1
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Superpowers workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: brainstorming -> writing-plans -> subagent-driven-development -> verification -> finishing" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

$OutputDir = Join-Path $ScriptDir "outputs"
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

$DesignFile = Join-Path $OutputDir "design.md"
$PlanFile = Join-Path $OutputDir "plan.md"
$VerifyReport = Join-Path $OutputDir "verification-report.md"

Write-Host "[Step 1/5] Brainstorming - 需求理解与设计..." -ForegroundColor Green
$BrainstormPrompt = @"
请使用 brainstorming skill 分析 PRD.md 中的 AI Coding可落地性评估系统需求。

## PRD文件
$PrdFile

## 核心需求
- 评估维度: 上下文完备性(25%)、逻辑原子性(25%)、边界明确性(20%)、可验证性(15%)、技术约束清晰度(15%)
- 评分算法: AI就绪分 = Σ(维度得分×权重) × 复杂度惩罚系数
- 成功指标: AI代码采纳率>80%、评估效率<2分钟

## 流程
1) 探索项目上下文
2) 提出视觉辅助(如有UI问题)
3) 提出澄清问题
4) 提出2-3个方案及权衡
5) 展示设计sections获取批准

## 输出
请将设计文档保存到: $DesignFile
"@
opencode run -m "$Model" $BrainstormPrompt

Write-Host ""
Write-Host "[Step 2/5] Writing Plans - 创建实现计划..." -ForegroundColor Green
$PlansPrompt = @"
请使用 writing-plans skill 基于已批准的设计创建详细实现计划。

## 设计文档
$DesignFile

## 计划要求
- 分解为2-5分钟可完成的原子任务
- 每个任务有精确文件路径、完整代码、验证步骤
- 使用 subagent-driven-development skill 进行任务分解

## PRD核心要求
- 评估维度模型、评分算法、报告生成器
- S/A/B/C等级、风险热力图

## 输出
请将计划保存到: $PlanFile
"@
opencode run -m "$Model" $PlansPrompt

Write-Host ""
Write-Host "[Step 3/5] Subagent-Driven Development - 执行实现..." -ForegroundColor Green
$SDDPrompt = @"
请使用 subagent-driven-development skill 执行实现计划。

## 实现计划
$PlanFile

## 实现内容
- 评估维度模型
- 评分算法
- S/A/B/C报告生成器
- 风险热力图

## 执行要求
每个任务由fresh subagent执行，两阶段review(规范合规性->代码质量)
"@
opencode run -m "$Model" $SDDPrompt

Write-Host ""
Write-Host "[Step 4/5] Verification - 验证..." -ForegroundColor Green
$VerifyPrompt = @"
请使用 verification-before-completion skill 进行最终验证。

## 实现产出
请验证Step 3的实现产出

## 验证要点
- 功能完整性
- 代码质量
- 测试覆盖(80%+)

## PRD成功指标
- AI代码采纳率>80%
- 返工率降低50%
- 评估效率<2分钟

## 输出
请将验证报告保存到: $VerifyReport
"@
opencode run -m "$Model" $VerifyPrompt

Write-Host ""
Write-Host "[Step 5/5] Finishing - 完成开发..." -ForegroundColor Green
$FinishPrompt = @"
请使用 finishing-a-development-branch skill 完成开发。

## 验证报告
$VerifyReport

## 完成要求
验证测试通过，展示选项(merge/PR/keep/discard)，清理worktree
"@
opencode run -m "$Model" $FinishPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Superpowers workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
