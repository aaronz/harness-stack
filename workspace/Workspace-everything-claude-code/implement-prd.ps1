$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$WorkspaceDir = Split-Path -Parent $ScriptDir
$PRDFile = Join-Path $WorkspaceDir "PRD.md"

$Model = if ($args[0]) { $args[0] } else { "opencode/minimax-m2.5-free" }

$lastIteration = Get-ChildItem -Path "$ScriptDir\outputs\iteration-*" -ErrorAction SilentlyContinue | 
    ForEach-Object { [int]($_.Name -replace 'iteration-', '') } | 
    Sort-Object | Select-Object -Last 1
$nextIteration = if ($lastIteration) { $lastIteration + 1 } else { 1 }
$outputDir = "$ScriptDir\outputs\iteration-$nextIteration"
New-Item -ItemType Directory -Path $outputDir -Force | Out-Null

if (-not (Test-Path $PRDFile)) {
    Write-Host "Error: PRD.md not found at $PRDFile" -ForegroundColor Red
    exit 1
}

$PRDContent = Get-Content $PRDFile -Raw

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Everything Claude Code workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "Methodology: Gap Analysis -> Plan -> TDD -> Review -> Verify -> Security Scan" -ForegroundColor Cyan
Write-Host "Iteration: $nextIteration" -ForegroundColor Cyan
Write-Host "Model: $Model" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

function Test-OutputFile {
    param([string]$FilePath)
    if (-not (Test-Path $FilePath)) {
        Write-Host "  ❌ File missing: $FilePath" -ForegroundColor Red
        return $false
    }
    $content = Get-Content $FilePath -Raw -ErrorAction SilentlyContinue
    if (-not $content -or $content.Length -lt 10) {
        Write-Host "  ❌ File invalid (too small): $FilePath" -ForegroundColor Red
        return $false
    }
    Write-Host "  ✅ File exists: $FilePath ($($content.Length) bytes)" -ForegroundColor Green
    return $true
}

function Start-RerunMissing {
    param([string]$File, [string]$Prompt)
    $maxRetries = 2
    $attempt = 0

    while ($attempt -lt $maxRetries) {
        if (Test-OutputFile $File) { return }
        $attempt++
        if ($attempt -lt $maxRetries) {
            Write-Host "  🔄 Regenerating ($attempt/$maxRetries)..." -ForegroundColor Yellow
            opencode run -m $Model $Prompt
        }
    }
    if (-not (Test-OutputFile $File)) {
        Write-Host "  ⚠️  File generation failed: $File" -ForegroundColor Red
    }
}

$planFile = "$outputDir\implementation-plan.md"
$tddDir = "$outputDir\tdd"
$reviewReport = "$outputDir\code-review-report.md"
$verifyReport = "$outputDir\verification-report.md"
$securityReport = "$outputDir\security-report.md"
$gapAnalysis = "$outputDir\gap-analysis.md"
New-Item -ItemType Directory -Path $tddDir -Force | Out-Null

Write-Host ""
Write-Host "[1/6] PRD Gap Analysis..." -ForegroundColor Yellow
$gapPrompt = "Analyze the gap between current implementation and PRD.

## Requirements Document
$PRDContent

## Output
Write gap analysis to: $gapAnalysis
Report must include:
1. Gap list (table format)
2. P0/P1/P2 classification
3. Implementation progress summary"
opencode run -m $Model $gapPrompt
Start-RerunMissing $gapAnalysis $gapPrompt

Write-Host ""
Write-Host "[2/6] Plan..." -ForegroundColor Yellow
$planPrompt = "Use /plan to create implementation plan.

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save plan to: $planFile"
opencode run -m $Model $planPrompt
Start-RerunMissing $planFile $planPrompt

Write-Host ""
Write-Host "[3/6] TDD..." -ForegroundColor Yellow
$tddPrompt = "Use /tdd for test-driven development.

## Implementation Plan
$planFile

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Requirements
1. RED: Tests must fail
2. GREEN: Minimal code to pass
3. REFACTOR: Ensure 80%+ coverage
Build must pass"
opencode run -m $Model $tddPrompt

Write-Host ""
Write-Host "[4/6] Review..." -ForegroundColor Yellow
$reviewPrompt = "Use /code-review for code review.

## Implementation Plan
$planFile

## TDD Output
$tddDir

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save review to: $reviewReport"
opencode run -m $Model $reviewPrompt
Start-RerunMissing $reviewReport $reviewPrompt

Write-Host ""
Write-Host "[5/6] Verify..." -ForegroundColor Yellow
$verifyPrompt = "Use /verify for verification loop.

## Review Report
$reviewReport

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save verification to: $verifyReport
Report must include:
1. P0 issue status
2. PRD completeness
3. Remaining issues
4. Next steps"
opencode run -m $Model $verifyPrompt
Start-RerunMissing $verifyReport $verifyPrompt

Write-Host ""
Write-Host "[6/6] Security Scan..." -ForegroundColor Yellow
$securityPrompt = "Use /security-scan for security audit.

## Verification Report
$verifyReport

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Security Checklist
- No hardcoded secrets
- SQL injection protection
- XSS protection
- CSRF protection
- Auth & auth
- Rate limiting

## Output
Save security report to: $securityReport"
opencode run -m $Model $securityPrompt
Start-RerunMissing $securityReport $securityPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Everything Claude Code workspace implementation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Output files:" -ForegroundColor Cyan
Write-Host "  - Gap Analysis: $gapAnalysis"
Write-Host "  - Plan: $planFile"
Write-Host "  - TDD: $tddDir"
Write-Host "  - Review: $reviewReport"
Write-Host "  - Verify: $verifyReport"
Write-Host "  - Security: $securityReport"
Write-Host "  - Output Dir: $outputDir"
