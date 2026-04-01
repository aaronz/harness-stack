$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PrdFile = Join-Path $ScriptDir "..\..\PRD.md"

$Model = if ($args.Count -gt 0) { $args[0] } else { "opencode/minimax-m2.5-free" }

if (-not (Test-Path $PrdFile)) {
    Write-Host "Error: PRD.md not found at $PrdFile" -ForegroundColor Red
    exit 1
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Spec Kit workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: /speckit.constitution -> /speckit.specify -> /speckit.plan -> /speckit.tasks -> /speckit.implement" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

$OutputDir = Join-Path $ScriptDir "outputs"
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

$ConstitutionFile = Join-Path $OutputDir "constitution.md"
$SpecFile = Join-Path $OutputDir "spec.md"
$PlanFile = Join-Path $OutputDir "plan.md"
$TasksFile = Join-Path $OutputDir "tasks.md"

Write-Host "[Step 1/5] Constitution - 建立项目原则..." -ForegroundColor Green
$ConstitutionPrompt = @"
请使用 /speckit.constitution 命令创建项目开发原则。

## PRD内容
$PrdFile

## 核心需求
- AI Coding可落地性评估系统
- 评估维度: 上下文完备性(25%)、逻辑原子性(25%)、边界明确性(20%)、可验证性(15%)、技术约束清晰度(15%)
- 评分算法: AI就绪分 = Σ(维度得分×权重) × 复杂度惩罚系数

## 原则要求
原则应聚焦于:
- 代码质量标准
- 测试覆盖率要求(80%+)
- 用户体验一致性
- 性能要求

## 输出
请将原则保存到: $ConstitutionFile
"@
opencode run -m "$Model" $ConstitutionPrompt

Write-Host ""
Write-Host "[Step 2/5] Specify - 定义需求规范..." -ForegroundColor Green
$SpecifyPrompt = @"
请使用 /speckit.specify 命令基于 PRD.md 定义需求规范。

## 项目原则
$ConstitutionFile

## PRD内容
$PrdFile

## 核心要点
- 五大评估维度(上下文完备性25%、逻辑原子性25%、边界明确性20%、可验证性15%、技术约束清晰度15%)
- 评分算法
- AI就绪等级(S/A/B/C级)

## 输出
请将规范保存到: $SpecFile
"@
opencode run -m "$Model" $SpecifyPrompt

Write-Host ""
Write-Host "[Step 3/5] Plan - 创建技术实现计划..." -ForegroundColor Green
$PlanPrompt = @"
请使用 /speckit.plan 命令创建技术实现计划。

## 需求规范
$SpecFile

## 技术栈
根据现有项目选择(Next.js + Express + SQLite)

## 架构
模块化设计

## 计划内容
- 评估引擎模块
- 评分计算器
- 报告生成器
- 数据模型设计

## 输出
请将计划保存到: $PlanFile
"@
opencode run -m "$Model" $PlanPrompt

Write-Host ""
Write-Host "[Step 4/5] Tasks - 生成任务清单..." -ForegroundColor Green
$TasksPrompt = @"
请使用 /speckit.tasks 命令生成可执行的任务清单。

## 技术计划
$PlanFile

## 任务要求
从技术计划中分解出具体的开发任务，每个任务应有明确的验收标准

## 输出
请将任务清单保存到: $TasksFile
"@
opencode run -m "$Model" $TasksPrompt

Write-Host ""
Write-Host "[Step 5/5] Implement - 执行实现..." -ForegroundColor Green
$ImplementPrompt = @"
请使用 /speckit.implement 命令执行所有任务。

## 任务清单
$TasksFile

## 实现内容
- 评估维度模型
- 评分算法
- S/A/B/C等级报告生成器
- 风险热力图功能

## 输出
确保满足PRD成功指标: AI代码采纳率>80%、评估效率<2分钟
"@
opencode run -m "$Model" $ImplementPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Spec Kit workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
