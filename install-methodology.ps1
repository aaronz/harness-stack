#!/usr/bin/env pwsh

param(
    [switch]$Clean,
    [Parameter(Position=0)]
    [string]$Methodology,
    [Parameter(Position=1)]
    [string]$Target
)

$ErrorActionPreference = "Stop"

$ScriptDir = $PSScriptRoot
if (-not $ScriptDir) {
    $ScriptDir = (Get-Location).Path
}
$SkillSourceDir = Join-Path $ScriptDir "skill-source"
$WorkspaceDir = Join-Path $ScriptDir "workspace"

$Methodologies = @("openspec", "gstack", "planning-with-files", "speckit", "superpowers", "everything-claude-code")
$CliInitMethods = @("openspec", "speckit")

$ScriptFiles = @(
    "iterate-prd.sh"
    "iterate-prd.ps1"
    "prompts.md"
)

function Print-Usage {
    Write-Host "Usage: $($MyInvocation.ScriptName) [options] <methodology> <target-folder>"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -c, --clean    Clean .opencode/skills and .opencode/commands before install"
    Write-Host ""
    Write-Host "Available methodologies:"
    foreach ($m in $Methodologies) {
        Write-Host "  - $m"
    }
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  $($MyInvocation.ScriptName) superpowers ~/projects/my-app"
    Write-Host "  $($MyInvocation.ScriptName) speckit ./my-workspace"
    Write-Host "  $($MyInvocation.ScriptName) --clean planning-with-files ../other-project"
}

function Test-CliInit {
    param([string]$Method)
    return $CliInitMethods -contains $Method
}

function Install-Methodology {
    param(
        [string]$Method,
        [string]$TargetPath,
        [bool]$DoClean
    )
    
    $SourceDir = Join-Path $SkillSourceDir $Method
    
    if (-not (Test-Path $SourceDir)) {
        Write-Host "Error: Source directory not found: $SourceDir" -ForegroundColor Red
        exit 1
    }
    
    $TargetAbs = (Resolve-Path $TargetPath).Path
    
    if (-not (Test-Path $TargetAbs)) {
        Write-Host "Target directory does not exist. Creating: $TargetAbs" -ForegroundColor Yellow
        New-Item -ItemType Directory -Path $TargetAbs -Force | Out-Null
    }
    
    Write-Host "Installing methodology: $Method" -ForegroundColor Cyan
    Write-Host "  Source: $SourceDir"
    Write-Host "  Target: $TargetAbs"
    Write-Host ""
    
    if (Test-CliInit $Method) {
        if ($DoClean) {
            $skillsDir = Join-Path $TargetAbs ".opencode/skills"
            $commandsDir = Join-Path $TargetAbs ".opencode/commands"
            
            New-Item -ItemType Directory -Path $skillsDir -Force | Out-Null
            New-Item -ItemType Directory -Path $commandsDir -Force | Out-Null
            
            Write-Host "Cleaning existing .opencode/skills, .opencode/commands, scripts"
            if (Test-Path $skillsDir) {
                Remove-Item (Join-Path $skillsDir "*") -Recurse -Force -ErrorAction SilentlyContinue
            }
            if (Test-Path $commandsDir) {
                Remove-Item (Join-Path $commandsDir "*") -Recurse -Force -ErrorAction SilentlyContinue
            }
            foreach ($script in $ScriptFiles) {
                $scriptPath = Join-Path $TargetAbs $script
                if (Test-Path $scriptPath) {
                    Remove-Item $scriptPath -Force -ErrorAction SilentlyContinue
                }
            }
        }
        
        switch ($Method) {
            "openspec" {
                $openspecCmd = Get-Command openspec -ErrorAction SilentlyContinue
                if ($openspecCmd) {
                    Push-Location $TargetAbs
                    try {
                        openspec init . --tools opencode --force 2>&1 | Out-Null
                        Write-Host "openspec init completed" -ForegroundColor Green
                    }
                    finally {
                        Pop-Location
                    }
                }
                else {
                    Write-Host "openspec CLI not found, installing..." -ForegroundColor Yellow
                    try {
                        npm install -g @fission-ai/openspec 2>&1 | Out-Null
                        $openspecCmd = Get-Command openspec -ErrorAction SilentlyContinue
                        if ($openspecCmd) {
                            Push-Location $TargetAbs
                            try {
                                openspec init . --tools opencode --force 2>&1 | Out-Null
                                Write-Host "openspec init completed" -ForegroundColor Green
                            }
                            finally {
                                Pop-Location
                            }
                        }
                        else {
                            Write-Host "Failed to install openspec, skipping init" -ForegroundColor Yellow
                        }
                    }
                    catch {
                        Write-Host "Failed to install openspec, skipping init" -ForegroundColor Yellow
                    }
                }
            }
            "speckit" {
                $specifyCmd = Get-Command specify -ErrorAction SilentlyContinue
                if ($specifyCmd) {
                    Push-Location $TargetAbs
                    try {
                        specify init . --ai opencode --here --force 2>&1 | Out-Null
                        Write-Host "specify init completed" -ForegroundColor Green
                    }
                    finally {
                        Pop-Location
                    }
                }
                else {
                    Write-Host "specify CLI not found, installing..." -ForegroundColor Yellow
                    $installed = $false
                    
                    $uvCmd = Get-Command uv -ErrorAction SilentlyContinue
                    if ($uvCmd -and -not $installed) {
                        try {
                            uv tool install specify-cli --from git+https://github.com/github/spec-kit.git 2>&1 | Out-Null
                            $installed = $true
                        }
                        catch { }
                    }
                    
                    $pipCmd = Get-Command pip -ErrorAction SilentlyContinue
                    if (-not $installed -and $pipCmd) {
                        try {
                            pip install specify-cli 2>&1 | Out-Null
                            $installed = $true
                        }
                        catch { }
                    }
                    
                    $pip3Cmd = Get-Command pip3 -ErrorAction SilentlyContinue
                    if (-not $installed -and $pip3Cmd) {
                        try {
                            pip3 install specify-cli 2>&1 | Out-Null
                            $installed = $true
                        }
                        catch { }
                    }
                    
                    $pipxCmd = Get-Command pipx -ErrorAction SilentlyContinue
                    if (-not $installed -and $pipxCmd) {
                        try {
                            pipx install specify-cli 2>&1 | Out-Null
                            $installed = $true
                        }
                        catch { }
                    }
                    
                    if ($installed -or (Get-Command specify -ErrorAction SilentlyContinue)) {
                        Push-Location $TargetAbs
                        try {
                            specify init . --ai opencode --here --force 2>&1 | Out-Null
                            Write-Host "specify init completed" -ForegroundColor Green
                        }
                        finally {
                            Pop-Location
                        }
                    }
                    else {
                        Write-Host "Failed to install specify, skipping init" -ForegroundColor Yellow
                    }
                }
            }
        }
        
        $workspacePath = Join-Path $WorkspaceDir "Workspace-$Method"
        if (Test-Path $workspacePath) {
            Write-Host "Copying scripts from workspace"
            foreach ($script in $ScriptFiles) {
                $srcScript = Join-Path $workspacePath $script
                if (Test-Path $srcScript) {
                    Copy-Item $srcScript -Destination $TargetAbs -Force
                }
            }
        }
    }
    else {
        $skillsDir = Join-Path $TargetAbs ".opencode/skills"
        $commandsDir = Join-Path $TargetAbs ".opencode/commands"
        
        New-Item -ItemType Directory -Path $skillsDir -Force | Out-Null
        New-Item -ItemType Directory -Path $commandsDir -Force | Out-Null
        
        if ($DoClean) {
            Write-Host "Cleaning existing .opencode/skills, .opencode/commands, scripts"
            if (Test-Path $skillsDir) {
                Remove-Item (Join-Path $skillsDir "*") -Recurse -Force -ErrorAction SilentlyContinue
            }
            if (Test-Path $commandsDir) {
                Remove-Item (Join-Path $commandsDir "*") -Recurse -Force -ErrorAction SilentlyContinue
            }
            foreach ($script in $ScriptFiles) {
                $scriptPath = Join-Path $TargetAbs $script
                if (Test-Path $scriptPath) {
                    Remove-Item $scriptPath -Force -ErrorAction SilentlyContinue
                }
            }
        }
        
        $foundAssets = $false
        
        $sourceSkillsPath = Join-Path $SourceDir ".opencode/skills"
        if (Test-Path $sourceSkillsPath) {
            Write-Host "Copying skills from $sourceSkillsPath"
            Copy-Item (Join-Path $sourceSkillsPath "*") -Destination $skillsDir -Recurse -Force
            $foundAssets = $true
        }
        
        $sourceSkillsAlt = Join-Path $SourceDir "skills"
        if (Test-Path $sourceSkillsAlt) {
            Write-Host "Copying skills from $sourceSkillsAlt"
            Copy-Item (Join-Path $sourceSkillsAlt "*") -Destination $skillsDir -Recurse -Force
            $foundAssets = $true
        }
        
        if ($Method -eq "gstack") {
            Write-Host "Copying gstack skills from repo root"
            Get-ChildItem -Path $SourceDir -Directory | ForEach-Object {
                $skillMd = Join-Path $_.FullName "SKILL.md"
                if (Test-Path $skillMd) {
                    $dirName = $_.Name
                    $targetSkillDir = Join-Path $skillsDir $dirName
                    New-Item -ItemType Directory -Path $targetSkillDir -Force | Out-Null
                    Copy-Item (Join-Path $_.FullName "*") -Destination $targetSkillDir -Recurse -Force
                    $foundAssets = $true
                }
            }
        }
        
        $sourceCommandsPath = Join-Path $SourceDir ".opencode/commands"
        if (Test-Path $sourceCommandsPath) {
            Write-Host "Copying commands from $sourceCommandsPath"
            Copy-Item (Join-Path $sourceCommandsPath "*") -Destination $commandsDir -Recurse -Force
            $foundAssets = $true
        }
        
        $sourceCommandsAlt = Join-Path $SourceDir "commands"
        if (Test-Path $sourceCommandsAlt) {
            Write-Host "Copying commands from $sourceCommandsAlt"
            Copy-Item (Join-Path $sourceCommandsAlt "*") -Destination $commandsDir -Recurse -Force
            $foundAssets = $true
        }
        
        if (-not $foundAssets) {
            Write-Host "No skills/commands found in repo" -ForegroundColor Yellow
        }
        
        $workspacePath = Join-Path $WorkspaceDir "Workspace-$Method"
        if (Test-Path $workspacePath) {
            Write-Host "Copying scripts from workspace"
            foreach ($script in $ScriptFiles) {
                $srcScript = Join-Path $workspacePath $script
                if (Test-Path $srcScript) {
                    Copy-Item $srcScript -Destination $TargetAbs -Force
                }
            }
        }
    }
    
    Write-Host "Successfully installed $Method to $TargetAbs" -ForegroundColor Green
}

if (-not $Methodology -or -not $Target) {
    Print-Usage
    exit 1
}

if ($Methodologies -notcontains $Methodology) {
    Write-Host "Error: Unknown methodology: $Methodology" -ForegroundColor Red
    Write-Host ""
    Print-Usage
    exit 1
}

Install-Methodology -Method $Methodology -TargetPath $Target -DoClean $Clean
