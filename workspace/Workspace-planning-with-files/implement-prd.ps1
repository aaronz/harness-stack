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
Write-Host "Planning With Files workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "Methodology: Gap Analysis -> 3-File Init -> Research -> Implement -> Verify" -ForegroundColor Cyan
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

$taskPlan = "$outputDir\task_plan.md"
$findings = "$outputDir\findings.md"
$progress = "$outputDir\progress.md"
$gapAnalysis = "$outputDir\gap-analysis.md"

Write-Host ""
Write-Host "[1/5] PRD Gap Analysis..." -ForegroundColor Yellow
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
Write-Host "[2/5] Initialize 3-File Pattern..." -ForegroundColor Yellow
$initPrompt = "Use /planning-with-files:plan to start.

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## 3-File Pattern
- $taskPlan: Task progress tracking
- $findings: Research findings
- $progress: Session logs

## Output
Save to these files"
opencode run -m $Model $initPrompt
Start-RerunMissing $taskPlan $initPrompt

Write-Host ""
Write-Host "[3/5] Research & Plan..." -ForegroundColor Yellow
$researchPrompt = "Continue with 3-file pattern.

## 3 Files
- $taskPlan
- $findings
- $progress

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Task Requirements
Create detailed task breakdown in $taskPlan
Store research in $findings"
opencode run -m $Model $researchPrompt

Write-Host ""
Write-Host "[4/5] Implement..." -ForegroundColor Yellow
$implPrompt = "Continue with 3-file pattern implementation.

## 3 Files
- $taskPlan
- $findings
- $progress

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Requirements
- Update progress in $taskPlan
- Build must pass"
opencode run -m $Model $implPrompt

Write-Host ""
Write-Host "[5/5] Verify..." -ForegroundColor Yellow
$verifyPrompt = "Complete verification with 3-file pattern.

## 3 Files
- $taskPlan
- $findings
- $progress

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Update $progress with test results
Report must include:
1. P0 issue status
2. PRD completeness
3. Remaining issues
4. Next steps"
opencode run -m $Model $verifyPrompt
Start-RerunMissing $progress $verifyPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Planning With Files workspace implementation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Output files:" -ForegroundColor Cyan
Write-Host "  - Gap Analysis: $gapAnalysis"
Write-Host "  - Task Plan: $taskPlan"
Write-Host "  - Findings: $findings"
Write-Host "  - Progress: $progress"
Write-Host "  - Output Dir: $outputDir"
