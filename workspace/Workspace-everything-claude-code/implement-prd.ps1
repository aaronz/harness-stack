$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PrdFile = Join-Path $ScriptDir "..\..\PRD.md"

$Model = if ($args.Count -gt 0) { $args[0] } else { "opencode/minimax-m2.5-free" }

if (-not (Test-Path $PrdFile)) {
    Write-Host "Error: PRD.md not found at $PrdFile" -ForegroundColor Red
    exit 1
}

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
请使用 /plan 命令创建 AI Coding可落地性评估系统的实现计划。

## PRD文件参考
$PrdFile

## 系统要求（从PRD提取的核心要求）
1. **评估维度模型**: 上下文完备性(25%)、逻辑原子性(25%)、边界明确性(20%)、可验证性(15%)、技术约束清晰度(15%)
2. **评分算法**: AI就绪分 = Σ(维度得分 × 维度权重) × 复杂度惩罚系数
3. **核心模块: 评估引擎、评分计算器、报告生成器**
4. **成功指标**:
   - AI代码采纳率 > 80%
   - 评估效率 < 2分钟

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

## PRD核心要求
- 评估维度: 上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束清晰度
- 评分计算: 加权求和 + 复杂度惩罚
- 成功指标: AI代码采纳率>80%、评估效率<2分钟

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

## 审查标准（来自PRD成功指标）
- AI代码采纳率 > 80%（代码需符合可测试性、可维护性）
- 评估效率 < 2分钟（性能优化检查）

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

## PRD成功指标验证
1. **AI代码采纳率>80%**: 代码质量是否足够清晰、无歧义
2. **评估效率<2分钟**: 性能是否满足要求

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

## PRD要求的安全检查
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