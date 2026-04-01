$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceDir = Split-Path -Parent $ScriptDir
$PrdFile = Join-Path $workspaceDir "..\PRD.md"

$Model = if ($args.Count -gt 0) { $args[0] } else { "opencode/minimax-m2.5-free" }

if (-not (Test-Path $PrdFile)) {
    Write-Host "Error: PRD.md not found at $PrdFile" -ForegroundColor Red
    exit 1
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "GStack workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: Think -> Plan -> Build -> Review -> Test -> Ship -> Reflect" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

$OutputDir = Join-Path $workspaceDir "outputs"
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

$DesignFile = Join-Path $OutputDir "design.md"
$CeoReview = Join-Path $OutputDir "ceo-review.md"
$EngReview = Join-Path $OutputDir "eng-review.md"
$ReviewReport = Join-Path $OutputDir "code-review-report.md"
$ShipReport = Join-Path $OutputDir "ship-report.md"

Write-Host "[Step 1/7] Office Hours - 需求理解..." -ForegroundColor Green
$OfficePrompt = @"
请使用 /office-hours 命令开始。

## PRD文件
$PrdFile

## 项目
AI Coding可落地性评估系统

## 核心需求
- 评估维度: 上下文完备性(25%)、逻辑原子性(25%)、边界明确性(20%)、可验证性(15%)、技术约束清晰度(15%)
- 评分算法: AI就绪分 = Σ(维度得分×权重) × 复杂度惩罚系数

## 流程
将提出6个强制问题来重新审视产品，挑战前提，生成多种实现方案

## 输出
请将设计文档保存到: $DesignFile
"@
opencode run -m "$Model" $OfficePrompt

Write-Host ""
Write-Host "[Step 2/7] Plan CEO Review - CEO级审查..." -ForegroundColor Green
$CeoPrompt = @"
请使用 /plan-ceo-review 命令进行CEO级审查。

## 设计文档
$DesignFile

## 审查要求
重新思考问题，找到10星产品。4种模式: Expansion, Selective Expansion, Hold Scope, Reduction。10章节审查

## 输出
请将审查结果保存到: $CeoReview
"@
opencode run -m "$Model" $CeoPrompt

Write-Host ""
Write-Host "[Step 3/7] Plan Eng Review - 工程审查..." -ForegroundColor Green
$EngPrompt = @"
请使用 /plan-eng-review 命令进行工程审查。

## CEO审查
$CeoReview

## 审查要求
锁定架构、数据流、图表、边缘情况和测试。ASCII图、状态机、错误路径、测试矩阵、故障模式、安全问题

## 输出
请将工程审查结果保存到: $EngReview
"@
opencode run -m "$Model" $EngPrompt

Write-Host ""
Write-Host "[Step 4/7] Implement - 实现..." -ForegroundColor Green
$ImplementPrompt = "使用 gstack 的实现模式执行。

## 工程审查
$EngReview

## 实现内容
- 评估维度模型(上下文完备性25%、逻辑原子性25%、边界明确性20%、可验证性15%、技术约束清晰度15%)
- 评分算法(S/A/B/C级)
- 报告生成器
- 风险热力图"
opencode run -m "$Model" $ImplementPrompt

Write-Host ""
Write-Host "[Step 5/7] Review - 代码审查..." -ForegroundColor Green
$ReviewPrompt = @"
请使用 /review 命令进行代码审查。

## 实现产出
请审查Step 4的产出

## 审查要求
找到通过CI但在生产中爆发的bug。AUTO-FIXED 明显问题，FLAGS 完整性差距

## 输出
请将审查报告保存到: $ReviewReport
"@
opencode run -m "$Model" $ReviewPrompt

Write-Host ""
Write-Host "[Step 6/7] QA & Ship - 测试与发布..." -ForegroundColor Green
$ShipPrompt = @"
请使用 /qa 命令测试你的应用，找到bug并修复。然后使用 /ship 命令同步main、运行测试、审计覆盖率、推送、打开PR。

## 代码审查报告
$ReviewReport

## 要求
自动生成回归测试

## 输出
请将发布报告保存到: $ShipReport
"@
opencode run -m "$Model" $ShipPrompt

Write-Host ""
Write-Host "[Step 7/7] Retro - 回顾..." -ForegroundColor Green
$RetroPrompt = @"
请使用 /retro 命令进行回顾。

## 发布报告
$ShipReport

## 要求
每周总结: 人员分解、航运 streaks、测试健康趋势、增长机会
"@
opencode run -m "$Model" $RetroPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "GStack workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
