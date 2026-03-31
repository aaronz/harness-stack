$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$WorkspaceDir = Split-Path -Parent $ScriptDir
$PrdFile = Join-Path $WorkspaceDir "..\PRD.md"

$Model = if ($args.Count -gt 0) { $args[0] } else { "opencode/minimax-m2.5-free" }

if (-not (Test-Path $PrdFile)) {
    Write-Host "Error: PRD.md not found at $PrdFile" -ForegroundColor Red
    exit 1
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Spec Kit Workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: /speckit.constitution -> /speckit.specify -> /speckit.plan -> /speckit.tasks -> /speckit.implement" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

Write-Host "[Step 1/5] Constitution - 建立项目原则..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /speckit.constitution 命令创建项目开发原则。原则应聚焦于: 代码质量标准、测试覆盖率要求(80%+)、用户体验一致性、性能要求。确保这些原则指导所有后续技术决策。"

Write-Host ""
Write-Host "[Step 2/5] Specify - 定义需求规范..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /speckit.specify 命令基于 PRD.md 定义需求规范。PRD内容: AI Coding可落地性评估系统。核心要点: 五大评估维度(上下文完备性25%、逻辑原子性25%、边界明确性20%、可验证性15%、技术约束清晰度15%)、评分算法、AI就绪等级(S/A/B/C级)"

Write-Host ""
Write-Host "[Step 3/5] Plan - 创建技术实现计划..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /speckit.plan 命令创建技术实现计划。技术栈: 根据现有项目选择。架构: 模块化设计。计划应包含: 评估引擎模块、评分计算器、报告生成器、数据模型设计"

Write-Host ""
Write-Host "[Step 4/5] Tasks - 生成任务清单..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /speckit.tasks 命令生成可执行的任务清单。从技术计划中分解出具体的开发任务，每个任务应有明确的验收标准"

Write-Host ""
Write-Host "[Step 5/5] Implement - 执行实现..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /speckit.implement 命令执行所有任务。实现内容: 评估维度模型、评分算法、S/A/B/C等级报告生成器、风险热力图功能"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Spec Kit Workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan