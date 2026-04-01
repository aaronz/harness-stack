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

$OutputDir = Join-Path $workspaceDir "outputs"
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

$ProposalDir = Join-Path $OutputDir "proposal"
New-Item -ItemType Directory -Path $ProposalDir -Force | Out-Null

Write-Host "[Step 1/3] Propose - 创建需求提案..." -ForegroundColor Green
$ProposePrompt = @"
请使用 /opsx:propose ai-ready-evaluator 命令基于 PRD.md 创建需求提案。

## PRD内容
$PrdFile

## 核心需求
- AI Coding可落地性评估系统
- 评估维度: 上下文完备性(25%)、逻辑原子性(25%)、边界明确性(20%)、可验证性(15%)、技术约束清晰度(15%)
- 评分算法: AI就绪分 = Σ(维度得分×权重) × 复杂度惩罚系数
- AI就绪等级: S级(90-100)、A级(75-89)、B级(60-74)、C级(<60)

## 输出要求
请将产出保存到: $ProposalDir
- proposal.md: 需求提案
- specs/: 详细规格
- design.md: 设计文档
- tasks.md: 任务清单
"@
opencode run -m "$Model" $ProposePrompt

Write-Host ""
Write-Host "[Step 2/3] Apply - 执行实现..." -ForegroundColor Green
$ApplyPrompt = @"
请使用 /opsx:apply 命令执行 ai-ready-evaluator 的实现。

## 提案产出参考
$ProposalDir

## PRD核心要求
- 评估维度模型: 上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束清晰度
- 评分算法: 加权求和 + 复杂度惩罚
- 报告生成器: S/A/B/C等级、风险热力图

## 输出要求
将实现代码保存到项目目录，确保与proposal中的设计一致
"@
opencode run -m "$Model" $ApplyPrompt

Write-Host ""
Write-Host "[Step 3/3] Archive - 归档..." -ForegroundColor Green
$ArchivePrompt = @"
请使用 /opsx:archive 命令归档完成的功能。

## 实现产出参考
请基于Step 2的实现产出进行归档

## 输出
完成归档，更新相关文档
"@
opencode run -m "$Model" $ArchivePrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "OpenSpec workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
