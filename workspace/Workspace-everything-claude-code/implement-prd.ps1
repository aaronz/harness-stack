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
Write-Host "Everything Claude Code workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "方法论: /plan -> /tdd -> /code-review -> /verify -> /security-scan" -ForegroundColor Yellow
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "PRD: $PrdFile"
Write-Host ""

$OutputDir = Join-Path $ScriptDir "outputs"
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

$TddOutputDir = Join-Path $OutputDir "tdd"
New-Item -ItemType Directory -Path $TddOutputDir -Force | Out-Null

$PlanFile = Join-Path $OutputDir "implementation-plan.md"
$ReviewReport = Join-Path $OutputDir "code-review-report.md"
$VerifyReport = Join-Path $OutputDir "verification-report.md"
$SecurityReport = Join-Path $OutputDir "security-report.md"

Write-Host "[Step 1/5] Plan - 创建实现计划..." -ForegroundColor Green
$PlanPrompt = @"
请使用 /plan 命令创建实现计划。

## Requirements Document
$PrdContent

## 输出要求
请将完整的实施计划保存到: $PlanFile
确保计划包含:
- 模块划分及依赖关系
- 数据模型设计
- API接口定义
- 前端页面规划
"@
opencode run -m "$Model" $PlanPrompt

Write-Host ""
Write-Host "[Step 2/5] TDD - 测试驱动开发..." -ForegroundColor Green
$TddPrompt = @"
请使用 /tdd 命令执行测试驱动开发。

## 实施计划参考
$PlanFile

## Requirements Document
$PrdContent

## 输出要求
1. 先创建测试文件（RED阶段），测试必须失败
2. 实现最小代码使测试通过（GREEN阶段）
3. 重构并确保测试覆盖（REFACTOR阶段）
4. 覆盖率需达到80%+

请将TDD产出保存到: $TddOutputDir
"@
opencode run -m "$Model" $TddPrompt

Write-Host ""
Write-Host "[Step 3/5] Review - 代码审查与验证..." -ForegroundColor Green
$ReviewPrompt = @"
请使用 /code-review 命令审查代码质量。

## 实施计划
$PlanFile

## TDD产出
$TddOutputDir

## Requirements Document
$PrdContent

## 审查标准
结合需求文档检查代码质量、可维护性、可测试性与性能表现

## 输出
请将审查报告保存到: $ReviewReport
"@
opencode run -m "$Model" $ReviewPrompt

Write-Host ""
Write-Host "[Step 4/5] Verify - 验证循环..." -ForegroundColor Green
$VerifyPrompt = @"
请使用 /verify 命令运行验证循环。

## 审查报告
$ReviewReport

## Requirements Document
$PrdContent

## 验证重点
根据需求文档验证功能正确性、质量标准与性能要求

## 输出
请将验证报告保存到: $VerifyReport
"@
opencode run -m "$Model" $VerifyPrompt

Write-Host ""
Write-Host "[Step 5/5] Security Scan - 安全审计..." -ForegroundColor Green
$SecurityPrompt = @"
请使用 /security-scan 命令进行安全审计。

## 验证报告
$VerifyReport

## Requirements Document
$PrdContent

## 安全检查清单
- 无硬编码 secrets
- SQL注入防护
- XSS防护
- CSRF保护
- 认证授权验证
- 速率限制

## 输出
请将安全审计报告保存到: $SecurityReport
"@
opencode run -m "$Model" $SecurityPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Everything Claude Code workspace 实现完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "产出文件:" -ForegroundColor Cyan
Write-Host "  - $PlanFile"
Write-Host "  - $TddOutputDir"
Write-Host "  - $ReviewReport"
Write-Host "  - $VerifyReport"
Write-Host "  - $SecurityReport"
