# AI Ready Evaluator - One-Click Startup Script (Windows PowerShell)
# ==================================================================
# This script starts both the backend and frontend services
# for local development.

# Check if Node.js is installed
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Node.js is not installed." -ForegroundColor Red
    Write-Host "Please install Node.js from https://nodejs.org/"
    exit 1
}

# Check if npm is installed
if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Write-Host "Error: npm is not installed." -ForegroundColor Red
    Write-Host "Please install Node.js from https://nodejs.org/"
    exit 1
}

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Function to check if dependencies are installed
function Test-Dependencies {
    param(
        [string]$Dir,
        [string]$Name
    )
    
    if (-not (Test-Path "$Dir/node_modules")) {
        Write-Host "Installing $Name dependencies..." -ForegroundColor Yellow
        Set-Location $Dir
        npm install
        Set-Location $ScriptDir
    } else {
        Write-Host "$Name dependencies already installed." -ForegroundColor Green
    }
}

# Check and install dependencies
Test-Dependencies -Dir "$ScriptDir/backend" -Name "Backend"
Test-Dependencies -Dir "$ScriptDir/frontend" -Name "Frontend"

# Copy .env.example to .env if not exists
if (-not (Test-Path "$ScriptDir/backend/.env")) {
    Write-Host "Creating backend .env file..." -ForegroundColor Yellow
    Copy-Item "$ScriptDir/backend/.env.example" "$ScriptDir/backend/.env"
    Write-Host ".env file created. Please update with your settings." -ForegroundColor Green
}

# Create background jobs for backend and frontend
Write-Host "Starting backend server..." -ForegroundColor Green
$BackendJob = Start-Job -ScriptBlock {
    param($Path)
    Set-Location $Path
    npm run dev
} -ArgumentList "$ScriptDir/backend"

Start-Sleep -Seconds 2

Write-Host "Starting frontend server..." -ForegroundColor Green
$FrontendJob = Start-Job -ScriptBlock {
    param($Path)
    Set-Location $Path
    npm run dev
} -ArgumentList "$ScriptDir/frontend"

Start-Sleep -Seconds 3

# Print service URLs
Write-Host ""
Write-Host "============================================"
Write-Host "🚀 AI Ready Evaluator is running!" -ForegroundColor Green
Write-Host "============================================"
Write-Host ""
Write-Host "Backend:  http://localhost:3001" -ForegroundColor Yellow
Write-Host "Frontend: http://localhost:3000" -ForegroundColor Yellow
Write-Host ""
Write-Host "Press Ctrl+C to stop all services"
Write-Host "============================================"
Write-Host ""

# Wait for Ctrl+C
try {
    while ($true) {
        # Display job output
        $BackendJob | Receive-Job
        $FrontendJob | Receive-Job
        Start-Sleep -Seconds 1
    }
} finally {
    # Cleanup
    Write-Host "`nShutting down services..." -ForegroundColor Yellow
    Stop-Job $BackendJob, $FrontendJob -ErrorAction SilentlyContinue
    Remove-Job $BackendJob, $FrontendJob -Force -ErrorAction SilentlyContinue
    Write-Host "All services stopped." -ForegroundColor Green
}
