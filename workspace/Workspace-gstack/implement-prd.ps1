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

Write-Host "[Step 1/7] Office Hours - 需求理解..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /office-hours 命令开始。这将重新定义你的产品理念并生成设计文档。PRD文件: $PrdFile。AI Coding可落地性评估系统。将提出6个强制问题来重新审视产品，挑战前提，生成多种实现方案"

Write-Host ""
Write-Host "[Step 2/7] Plan CEO Review - CEO级审查..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /plan-ceo-review 命令进行CEO级审查。重新思考问题，找到10星产品。4种模式: Expansion, Selective Expansion, Hold Scope, Reduction。10章节审查"

Write-Host ""
Write-Host "[Step 3/7] Plan Eng Review - 工程审查..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /plan-eng-review 命令进行工程审查。锁定架构、数据流、图表、边缘情况和测试。ASCII图、状态机、错误路径、测试矩阵、故障模式、安全问题"

Write-Host ""
Write-Host "[Step 4/7] Implement - 实现..." -ForegroundColor Green
opencode run -m "$Model" "使用 gstack 的实现模式执行。实现内容: 评估维度模型(上下文完备性25%、逻辑原子性25%、边界明确性20%、可验证性15%、技术约束清晰度15%)、评分算法(S/A/B/C级)、报告生成器、风险热力图"

Write-Host ""
Write-Host "[Step 5/7] Review - 代码审查..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /review 命令进行代码审查。找到通过CI但在生产中爆发的bug。AUTO-FIXED 明显问题，FLAGS 完整性差距"

Write-Host ""
Write-Host "[Step 6/7] QA & Ship - 测试与发布..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /qa 命令测试你的应用，找到bug并修复。然后使用 /ship 命令同步main、运行测试、审计覆盖率、推送、打开PR。自动生成回归测试"

Write-Host ""
Write-Host "[Step 7/7] Retro - 回顾..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /retro 命令进行回顾。每周总结: 人员分解、航运 streaks、测试健康趋势、增长机会"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "GStack workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan