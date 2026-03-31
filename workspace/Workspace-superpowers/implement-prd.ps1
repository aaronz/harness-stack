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
Write-Host "Superpowers Workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: brainstorming -> writing-plans -> subagent-driven-development -> verification -> finishing" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

Write-Host "[Step 1/5] Brainstorming - 需求理解与设计..." -ForegroundColor Green
opencode run -m "$Model" "请使用 brainstorming skill 分析 PRD.md 中的 AI Coding可落地性评估系统需求。文件位置: $PrdFile。流程: 1)探索项目上下文 2)提出视觉辅助(如有UI问题) 3)提出澄清问题 4)提出2-3个方案及权衡 5)展示设计sections获取批准。输出: design doc 保存到 docs/superpowers/specs/"

Write-Host ""
Write-Host "[Step 2/5] Writing Plans - 创建实现计划..." -ForegroundColor Green
opencode run -m "$Model" "请使用 writing-plans skill 基于已批准的设计创建详细实现计划。计划需分解为2-5分钟可完成的原子任务，每个任务有精确文件路径、完整代码、验证步骤。使用 subagent-driven-development skill 进行任务分解"

Write-Host ""
Write-Host "[Step 3/5] Subagent-Driven Development - 执行实现..." -ForegroundColor Green
opencode run -m "$Model" "请使用 subagent-driven-development skill 执行实现计划。每个任务由fresh subagent执行，两阶段review(规范合规性->代码质量)。实现内容: 评估维度模型、评分算法、S/A/B/C报告生成器、风险热力图"

Write-Host ""
Write-Host "[Step 4/5] Verification - 验证..." -ForegroundColor Green
opencode run -m "$Model" "请使用 verification-before-completion skill 进行最终验证。验证要点: 功能完整性、代码质量、测试覆盖(80%+)确保PRD中的成功指标满足: AI代码采纳率>80%、返工率降低50%、评估效率<2分钟"

Write-Host ""
Write-Host "[Step 5/5] Finishing - 完成开发..." -ForegroundColor Green
opencode run -m "$Model" "请使用 finishing-a-development-branch skill 完成开发。验证测试通过，展示选项(merge/PR/keep/discard)，清理worktree"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Superpowers Workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan