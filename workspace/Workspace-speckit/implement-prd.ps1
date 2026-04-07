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

New-Item -ItemType Directory -Path "$WorkspaceDir\.specify\memory" -Force | Out-Null
New-Item -ItemType Directory -Path "$WorkspaceDir\.specify\templates" -Force | Out-Null
New-Item -ItemType Directory -Path "$WorkspaceDir\.specify\specs" -Force | Out-Null

if (-not (Test-Path $PRDFile)) {
    Write-Host "Error: PRD.md not found at $PRDFile" -ForegroundColor Red
    exit 1
}

$PRDContent = Get-Content $PRDFile -Raw

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Spec Kit workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "Methodology: Gap Analysis → Constitution → Specify → Plan → Tasks → Implement" -ForegroundColor Cyan
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

$constitutionFile = "$outputDir\constitution.md"
$specFile = "$outputDir\spec.md"
$planFile = "$outputDir\plan.md"
$tasksFile = "$outputDir\tasks.md"
$gapAnalysis = "$outputDir\gap-analysis.md"
$constitutionPath = "$WorkspaceDir\.specify\memory\constitution.md"

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
3. Technical debt list
4. Implementation progress summary"
opencode run -m $Model $gapPrompt
Start-RerunMissing $gapAnalysis $gapPrompt

Write-Host ""
Write-Host "[2/6] Constitution..." -ForegroundColor Yellow
$constPrompt = "Update project constitution.

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save constitution to: $constitutionFile"
opencode run -m $Model $constPrompt
Start-RerunMissing $constitutionFile $constPrompt

Write-Host ""
Write-Host "[3/6] Specify..." -ForegroundColor Yellow
$specPrompt = "Create detailed specification.

## Requirements Document
$PRDContent

## Constitution
$(Get-Content $constitutionPath -Raw)

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save specification to: $specFile"
opencode run -m $Model $specPrompt
Start-RerunMissing $specFile $specPrompt

Write-Host ""
Write-Host "[4/6] Plan..." -ForegroundColor Yellow
$planPrompt = "Create implementation plan.

## Specification
$(Get-Content $specFile -Raw)

## Constitution
$(Get-Content $constitutionPath -Raw)

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save plan to: $planFile"
opencode run -m $Model $planPrompt
Start-RerunMissing $planFile $planPrompt

Write-Host ""
Write-Host "[5/6] Tasks..." -ForegroundColor Yellow
$tasksPrompt = "Generate actionable task list.

## Implementation Plan
$(Get-Content $planFile -Raw)

## Output
Save tasks to: $tasksFile"
opencode run -m $Model $tasksPrompt
Start-RerunMissing $tasksFile $tasksPrompt

Write-Host ""
Write-Host "[6/6] Implement..." -ForegroundColor Yellow
$implPrompt = "Execute all tasks from task list.

## Task List
$(Get-Content $tasksFile -Raw)

## Implementation Plan
$(Get-Content $planFile -Raw)

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Requirements
- Build must pass"
opencode run -m $Model $implPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Spec Kit workspace implementation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Output files:" -ForegroundColor Cyan
Write-Host "  - Gap Analysis: $gapAnalysis"
Write-Host "  - Constitution: $constitutionFile"
Write-Host "  - Specification: $specFile"
Write-Host "  - Plan: $planFile"
Write-Host "  - Tasks: $tasksFile"
Write-Host "  - Output Dir: $outputDir"