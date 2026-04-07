$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

$Model = if ($args[0]) { $args[0] } else { "opencode/minimax-m2.5-free" }

$lastIteration = Get-ChildItem -Path "$ScriptDir\workspace\*\outputs\iteration-*" -ErrorAction SilentlyContinue | 
    ForEach-Object { [int]($_.Name -replace 'iteration-', '') } | 
    Sort-Object | Select-Object -Last 1
$nextIteration = if ($lastIteration) { $lastIteration + 1 } else { 1 }

$workspaces = @(
    "workspace\workspace-openspec",
    "workspace\workspace-speckit",
    "workspace\workspace-superpowers",
    "workspace\workspace-everything-claude-code",
    "workspace\workspace-planning-with-files",
    "workspace\workspace-gstack"
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Running PRD Implementation for All workspaces" -ForegroundColor Cyan
Write-Host "Iteration: $nextIteration" -ForegroundColor Cyan
Write-Host "Model: $Model" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

foreach ($workspace in $workspaces) {
    $workspacePath = Join-Path $ScriptDir $workspace
    $scriptPath = Join-Path $workspacePath "implement-prd.ps1"
    
    if (Test-Path $scriptPath) {
        Write-Host "========================================" -ForegroundColor Cyan
        Write-Host "Running: $workspace" -ForegroundColor Yellow
        Write-Host "========================================" -ForegroundColor Cyan
        Push-Location $workspacePath
        & $scriptPath $Model
        Pop-Location
        Write-Host ""
    } else {
        Write-Host "⚠️ Warning: $scriptPath not found, skipping" -ForegroundColor Yellow
    }
}

Write-Host "========================================" -ForegroundColor Green
Write-Host "All workspaces Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Output directories:" -ForegroundColor Cyan
foreach ($workspace in $workspaces) {
    $workspaceName = Split-Path $workspace -Leaf
    Write-Host "  - $workspaceName\outputs\iteration-$nextIteration\"
}
