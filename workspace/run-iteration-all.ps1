# 批量运行所有Workspace迭代脚本
param(
    [string]$Model = "opencode/minimax-m2.5-free"
)

Write-Host "==============================================" -ForegroundColor Cyan
Write-Host "批量执行所有Workspace迭代开发" -ForegroundColor Cyan
Write-Host "==============================================" -ForegroundColor Cyan
Write-Host "使用模型: $Model"
Write-Host ""

$IterationOrder = @(
    "workspace-openspec",
    "workspace-gstack",
    "workspace-planning-with-files",
    "workspace-speckit",
    "workspace-superpowers",
    "workspace-everything-claude-code"
)

foreach ($workspace in $IterationOrder) {
    $workspacePath = Join-Path $PSScriptRoot $workspace
    $scriptPath = Join-Path $workspacePath "iterate-prd.ps1"
    
    if (Test-Path $scriptPath) {
        Write-Host ""
        Write-Host "##############################################" -ForegroundColor Magenta
        Write-Host "# 迭代: $workspace" -ForegroundColor Magenta
        Write-Host "##############################################" -ForegroundColor Magenta
        
        Push-Location $workspacePath
        try {
            & $scriptPath -Model $Model
        }
        finally {
            Pop-Location
        }
        
        Write-Host ""
        Write-Host "[完成] $workspace 迭代" -ForegroundColor Green
    }
    else {
        Write-Host "[跳过] $workspace - 迭代脚本不存在" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "==============================================" -ForegroundColor Green
Write-Host "全部Workspace迭代完成!" -ForegroundColor Green
Write-Host "==============================================" -ForegroundColor Green
