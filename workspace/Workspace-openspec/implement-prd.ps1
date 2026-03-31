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
Write-Host "OpenSpec workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: /opsx:propose -> /opsx:apply -> /opsx:archive" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

Write-Host "[Step 1/3] Propose - 创建需求提案..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /opsx:propose ai-ready-evaluator 命令基于 PRD.md 创建需求提案。PRD内容: AI Coding可落地性评估系统(AI-Ready Evaluator)。文件位置: $PrdFile。输出: proposal.md, specs/, design.md, tasks.md"

Write-Host ""
Write-Host "[Step 2/3] Apply - 执行实现..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /opsx:apply 命令执行 ai-ready-evaluator 的实现。实现内容: 评估维度模型(上下文完备性25%、逻辑原子性25%、边界明确性20%、可验证性15%、技术约束清晰度15%)、评分算法(权重x维度+复杂度惩罚)、S/A/B/C等级报告生成器"

Write-Host ""
Write-Host "[Step 3/3] Archive - 归档..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /opsx:archive 命令归档完成的功能"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "OpenSpec workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan