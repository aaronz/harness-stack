$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PrdFile = Join-Path $ScriptDir "..\..\PRD.md"

$Model = if ($args.Count -gt 0) { $args[0] } else { "opencode/minimax-m2.5-free" }

if (-not (Test-Path $PrdFile)) {
    Write-Host "Error: PRD.md not found at $PrdFile" -ForegroundColor Red
    exit 1
}

$PrdContent = Get-Content $PrdFile -Raw

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Planning With Files workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: 3-File Pattern (task_plan.md + findings.md + progress.md)" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

$OutputDir = Join-Path $ScriptDir "outputs"
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

$TaskPlan = Join-Path $OutputDir "task_plan.md"
$Findings = Join-Path $OutputDir "findings.md"
$Progress = Join-Path $OutputDir "progress.md"

Write-Host "[Step 1/4] Initialize - 初始化3文件模式..." -ForegroundColor Green
$InitPrompt = @"
请使用 /planning-with-files:plan 或 /plan 命令启动规划会话。

## Requirements Document
$PrdContent

## 3-File模式
这将自动创建 3 个文件:
- $TaskPlan: 任务和进度跟踪
- $Findings: 研究和发现
- $Progress: 会话日志和测试结果

## 输出
请将 Requirements Document 中的需求写入这些文件作为持久化上下文
"@
opencode run -m "$Model" $InitPrompt

Write-Host ""
Write-Host "[Step 2/4] Research & Plan - 研究与规划..." -ForegroundColor Green
$ResearchPrompt = @"
继续使用 3-file 模式。

## 3个文件
- $TaskPlan
- $Findings
- $Progress

## 任务要求
基于 Requirements Document，在 $TaskPlan 中创建详细的任务分解

## Requirements Document
$PrdContent

## 研究要求
使用 $Findings 存储研究内容。每 2 个操作后保存 findings

## 要求
从 Requirements Document 提取核心需求并写入任务与研究计划
"@
opencode run -m "$Model" $ResearchPrompt

Write-Host ""
Write-Host "[Step 3/4] Implement - 执行实现..." -ForegroundColor Green
$ImplementPrompt = @"
继续使用 3-file 模式实现。

## 3个文件
- $TaskPlan
- $Findings
- $Progress

## Requirements Document
$PrdContent

## 实现内容
基于 Requirements Document 提取实现范围并执行

## 进度更新
在 $TaskPlan 中更新进度(checkbox)

## 错误处理
错误必须记录在 $TaskPlan 以避免重复失败
"@
opencode run -m "$Model" $ImplementPrompt

Write-Host ""
Write-Host "[Step 4/4] Verify - 验证完成..." -ForegroundColor Green
$VerifyPrompt = @"
使用 3-file 模式的完成检查。

## 3个文件
- $TaskPlan
- $Findings
- $Progress

## Requirements Document
$PrdContent

## 验证要求
在 $TaskPlan 中验证所有 phases 完成，并根据 Requirements Document 验证交付结果

## 输出
更新 $Progress 记录测试结果
"@
opencode run -m "$Model" $VerifyPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Planning With Files workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
