$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

$Model = if ($args.Count -gt 0) { $args[0] } else { "opencode/minimax-m2.5-free" }

$Workspaces = @(
    "workspace\Workspace-openspec",
    "workspace\Workspace-speckit",
    "workspace\Workspace-superpowers",
    "workspace\Workspace-everything-claude-code",
    "workspace\Workspace-planning-with-files",
    "workspace\Workspace-gstack"
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Running PRD Implementation for All Workspaces" -ForegroundColor Cyan
Write-Host "模型: $Model" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

foreach ($workspace in $Workspaces) {
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
        Write-Host "Warning: $scriptPath not found, skipping" -ForegroundColor Yellow
    }
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "All Workspaces Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan