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
Write-Host "Everything Claude Code Workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: /plan -> /tdd -> /code-review -> /verify -> /security-scan" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

Write-Host "[Step 1/4] Plan - 创建实现计划..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /plan 命令创建 AI Coding可落地性评估系统的实现计划。PRD文件: $PrdFile。使用 planner agent 分解功能: 评估维度模型(上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束清晰度)、评分算法、报告生成器。输出: implementation blueprint"

Write-Host ""
Write-Host "[Step 2/4] TDD - 测试驱动开发..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /tdd 命令执行测试驱动开发。使用 tdd-guide agent 强制 RED-GREEN-REFACTOR 循环。实现内容: 评估引擎模块、评分计算器、报告生成器。每个功能先写失败测试，再写最小实现。覆盖率需达到80%+"

Write-Host ""
Write-Host "[Step 3/4] Review & Verify - 代码审查与验证..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /code-review 命令审查代码质量。然后使用 /verify 命令运行验证循环(build、test、lint、typecheck、security)。确保满足PRD成功指标: AI代码采纳率>80%、评估效率<2分钟"

Write-Host ""
Write-Host "[Step 4/4] Security Scan - 安全审计..." -ForegroundColor Green
opencode run -m "$Model" "请使用 /security-scan 命令进行安全审计。检查: 无硬编码 secrets、SQL注入防护、XSS防护、CSRF保护、认证授权验证、速率限制。确保生产就绪"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Everything Claude Code Workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan