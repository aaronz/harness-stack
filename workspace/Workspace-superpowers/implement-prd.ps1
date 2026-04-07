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
Write-Host "Superpowers workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "Methodology: Gap Analysis → Brainstorming → Plans → SDD → Verification" -ForegroundColor Cyan
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

$gapAnalysis = "$outputDir\gap-analysis.md"
$designFile = "$outputDir\design.md"
$planFile = "$outputDir\plan.md"
$verifyReport = "$outputDir\verification-report.md"

Write-Host ""
Write-Host "[1/5] PRD Gap Analysis..." -ForegroundColor Yellow
$gapPrompt = "Analyze the gap between current implementation and PRD.

## Requirements Document
$PRDContent

## Gap Analysis Dimensions
1. Feature completeness
2. API completeness
3. Frontend completeness
4. Data model
5. Configuration
6. Test coverage

## Output
Write gap analysis to: $gapAnalysis
Report must include:
1. Gap list (table format)
2. P0/P1/P2 classification
3. Technical debt list
4. Implementation progress summary"
opencode run -m $Model $gapPrompt
Start-RerunMissing $gapAnalysis $gapPrompt

Write-Host ""
Write-Host "[2/5] Brainstorming..." -ForegroundColor Yellow
$brainstormPrompt = "Use brainstorming skill to analyze PRD requirements.

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save design document to: $designFile"
opencode run -m $Model $brainstormPrompt
Start-RerunMissing $designFile $brainstormPrompt

Write-Host ""
Write-Host "[3/5] Writing Plans..." -ForegroundColor Yellow
$plansPrompt = "Use writing-plans skill to create implementation plan.

## Design Document
$(Get-Content $designFile -Raw)

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save plan to: $planFile"
opencode run -m $Model $plansPrompt
Start-RerunMissing $planFile $plansPrompt

Write-Host ""
Write-Host "[4/5] Subagent-Driven Development..." -ForegroundColor Yellow
$sddPrompt = "Use subagent-driven-development skill to execute plan.

## Implementation Plan
$(Get-Content $planFile -Raw)

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Requirements
- Build must pass"
opencode run -m $Model $sddPrompt

Write-Host ""
Write-Host "[5/5] Verification..." -ForegroundColor Yellow
$verifyPrompt = "Use verification-before-completion skill for final verification.

## Implementation Output
Verify Step 4 output

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Verification Points
- Functionality completeness
- Code quality
- Test coverage (80%+)
- Build must pass

## Output
Save verification report to: $verifyReport
Report must include:
1. P0 issue status
2. PRD completeness
3. Remaining issues
4. Next steps"
opencode run -m $Model $verifyPrompt
Start-RerunMissing $verifyReport $verifyPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Superpowers workspace implementation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Output files:" -ForegroundColor Cyan
Write-Host "  - Gap Analysis: $gapAnalysis"
Write-Host "  - Design: $designFile"
Write-Host "  - Plan: $planFile"
Write-Host "  - Verification: $verifyReport"
Write-Host "  - Output Dir: $outputDir"