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
Write-Host "OpenSpec workspace - PRD Implementation" -ForegroundColor Cyan
Write-Host "Methodology: Gap Analysis → Propose → Apply" -ForegroundColor Cyan
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

$proposalDir = "$outputDir\proposal"
$gapAnalysis = "$outputDir\gap-analysis.md"
New-Item -ItemType Directory -Path $proposalDir -Force | Out-Null

Write-Host ""
Write-Host "[1/3] PRD Gap Analysis..." -ForegroundColor Yellow
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
Write-Host "[2/3] Propose..." -ForegroundColor Yellow
$proposePrompt = "Use /opsx:propose to create proposal.

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Output
Save proposal to: $proposalDir
- proposal.md
- specs/
- design.md
- tasks.md"
opencode run -m $Model $proposePrompt
Start-RerunMissing "$proposalDir\proposal.md" $proposePrompt

Write-Host ""
Write-Host "[3/3] Apply + Archive..." -ForegroundColor Yellow
$applyPrompt = "Use /opsx:apply to execute.

## Proposal
$proposalDir

## Requirements Document
$PRDContent

## Gap Analysis
$(Get-Content $gapAnalysis -Raw)

## Requirements
- Build must pass"
opencode run -m $Model $applyPrompt

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "OpenSpec workspace implementation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Output files:" -ForegroundColor Cyan
Write-Host "  - Gap Analysis: $gapAnalysis"
Write-Host "  - Proposal: $proposalDir"
Write-Host "  - Output Dir: $outputDir"