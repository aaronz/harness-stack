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
Write-Host "GStack workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "Methodology: Gap Analysis -> Office Hours -> CEO Review -> Eng Review -> Implement -> Review -> QA -> Retro" -ForegroundColor Cyan
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

$designFile = "$outputDir\design.md"
$ceoReview = "$outputDir\ceo-review.md"
$engReview = "$outputDir\eng-review.md"
$reviewReport = "$outputDir\code-review-report.md"
$shipReport = "$outputDir\ship-report.md"
$gapAnalysis = "$outputDir\gap-analysis.md"

Write-Host ""
Write-Host "[1/8] PRD Gap Analysis..." -ForegroundColor Yellow
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
Write-Host "[2/8] Office Hours..." -ForegroundColor Yellow
$officePrompt = "Use /office-hours to start.

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save design to: $designFile"
opencode run -m $Model $officePrompt
Start-RerunMissing $designFile $officePrompt

Write-Host ""
Write-Host "[3/8] Plan CEO Review..." -ForegroundColor Yellow
$ceoPrompt = "Use /plan-ceo-review for CEO review.

## Design Document
$designFile

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save CEO review to: $ceoReview"
opencode run -m $Model $ceoPrompt
Start-RerunMissing $ceoReview $ceoPrompt

Write-Host ""
Write-Host "[4/8] Plan Eng Review..." -ForegroundColor Yellow
$engPrompt = "Use /plan-eng-review for engineering review.

## CEO Review
$ceoReview

## Output
Save engineering review to: $engReview"
opencode run -m $Model $engPrompt
Start-RerunMissing $engReview $engPrompt

Write-Host ""
Write-Host "[5/8] Implement..." -ForegroundColor Yellow
$implPrompt = "Use gstack implementation mode.

## Engineering Review
$engReview

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Requirements
Build must pass"
opencode run -m $Model $implPrompt

Write-Host ""
Write-Host "[6/8] Review..." -ForegroundColor Yellow
$reviewPrompt = "Use /review for code review.

## Implementation Output
Review Step 5 output

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save review to: $reviewReport"
opencode run -m $Model $reviewPrompt
Start-RerunMissing $reviewReport $reviewPrompt

Write-Host ""
Write-Host "[7/8] QA & Ship..." -ForegroundColor Yellow
$shipPrompt = "Use /qa to test, find bugs. Then use /ship to sync main, run tests, audit coverage, push, open PR.

## Code Review Report
$reviewReport

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Requirements
- Generate regression tests
- Build must pass

## Output
Save ship report to: $shipReport
Report must include:
1. P0 issue status
2. PRD completeness
3. Remaining issues
4. Next steps"
opencode run -m $Model $shipPrompt
Start-RerunMissing $shipReport $shipPrompt

Write-Host ""
Write-Host "[8/8] Retro..." -ForegroundColor Yellow
$retroPrompt = "Use /retro for retrospective.

## Ship Report
$shipReport

## Requirements
Weekly summary:人员分解、航运 streaks、测试健康趋势、增长机会"
opencode run -m $Model $retroPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "GStack workspace implementation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Output files:" -ForegroundColor Cyan
Write-Host "  - Gap Analysis: $gapAnalysis"
Write-Host "  - Design: $designFile"
Write-Host "  - CEO Review: $ceoReview"
Write-Host "  - Eng Review: $engReview"
Write-Host "  - Code Review: $reviewReport"
Write-Host "  - Ship Report: $shipReport"
Write-Host "  - Output Dir: $outputDir"
