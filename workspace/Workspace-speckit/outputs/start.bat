@echo off
echo Starting AI-Readiness Evaluator...

cd /d "%~dp0"

if not exist node_modules (
    echo Installing dependencies...
    call npm install
    if errorlevel 1 (
        echo Error: npm install failed
        exit /b 1
    )
)

if not exist data mkdir data

echo Starting backend server on port 3001...
start "Backend" cmd /c "npm run server"

timeout /t 3 /nobreak >nul

echo Starting frontend on port 3000...
start "Frontend" cmd /c "npm run dev"

echo.
echo AI-Readiness Evaluator is running!
echo    Frontend: http://localhost:3000
echo    Backend:  http://localhost:3001
echo.
echo Press any key to stop servers
pause >nul

taskkill /f /im node.exe >nul 2>&1
