$ErrorActionPreference = "Stop"

$Projects = @(
    "speckit:https://github.com/github/spec-kit.git",
    "openspec:https://github.com/Fission-AI/OpenSpec.git",
    "superpowers:https://github.com/obra/superpowers.git",
    "everything-claude-code:https://github.com/ysyecust/everything-claude-code.git",
    "planning-with-files:https://github.com/OthmanAdi/planning-with-files.git",
    "gstack:https://github.com/garrytan/gstack.git"
)

$SkillsSourceDir = "$env:USERPROFILE\.cache\opencode\node_modules\superpowers\skills"
$OpencodeConfigSkills = "$env:USERPROFILE\.config\opencode\skills"
$SkillSourceDir = "skill-source"
$workspaceDir = "workspace"

$CliInitProjects = @("openspec", "speckit")

function Needs-CliInit {
    param([string]$RepoName)
    return $CliInitProjects -contains $RepoName
}

function Fix-YamlTools {
    param([string]$FilePath)
    
    if (-not (Test-Path $FilePath)) { return }
    
    $content = Get-Content $FilePath -Raw
    if ($content -match '^\s*tools:\s+[A-Z]') {
        $newContent = $content -replace '^\s*tools:\s+(.+)$', {
            $tools = $args[0].Groups[1].Value -split ',\s*' | ForEach-Object { "`"$_`"" }
            "tools: [$($tools -join ', ')]"
        }
        Set-Content -Path $FilePath -Value $newContent -NoNewline
    }
}

$workspace = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $workspace

Write-Host "=== Initializing git repository in $workspace ===" -ForegroundColor Cyan

if (-not (Test-Path ".git")) {
    git init
    git checkout -b main
} else {
    Write-Host "Git repo already exists" -ForegroundColor Yellow
    $mainBranch = git rev-parse --verify main 2>$null
    if (-not $mainBranch) {
        git checkout -b main
    }
}

Write-Host ""
Write-Host "=== Cloning repos into $SkillSourceDir ===" -ForegroundColor Cyan

foreach ($project in $Projects) {
    $parts = $project -split ':'
    $repoName = $parts[0]
    $repoUrl = $parts[1]
    $targetDir = Join-Path $workspace "$SkillSourceDir\$repoName"

    Write-Host ""
    Write-Host "--- Cloning $repoName ---" -ForegroundColor Yellow

    $parentDir = Split-Path -Parent $targetDir
    if (-not (Test-Path $parentDir)) {
        New-Item -ItemType Directory -Path $parentDir -Force | Out-Null
    }
    if (-not (Test-Path $targetDir)) {
        New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
    }

    $tempClone = "$env:TEMP\${repoName}_clone"
    if (Test-Path $tempClone) {
        Remove-Item -Recurse -Force $tempClone
    }

    git clone --depth 1 $repoUrl $tempClone

    Copy-Item -Path "$tempClone\*" -Destination $targetDir -Recurse -Force
    Remove-Item -Recurse -Force $tempClone

    $fileCount = (Get-ChildItem -Path $targetDir -File).Count
    Write-Host "$repoName ready at $targetDir ($fileCount files)" -ForegroundColor Green
}

Write-Host ""
Write-Host "=== Creating workspaces with skills and commands ===" -ForegroundColor Cyan

foreach ($project in $Projects) {
    $parts = $project -split ':'
    $repoName = $parts[0]
    $repoUrl = $parts[1]

    $workspaceName = "workspace-$repoName"
    $workspacePath = Join-Path $workspace "$workspaceDir\$workspaceName"
    $repoPath = Join-Path $workspace "$SkillSourceDir\$repoName"

    Write-Host ""
    Write-Host "--- Setting up $workspaceName ---" -ForegroundColor Yellow

    $skillsDir = Join-Path $workspacePath ".opencode\skills"
    $commandsDir = Join-Path $workspacePath ".opencode\commands"

    New-Item -ItemType Directory -Path $skillsDir -Force | Out-Null
    New-Item -ItemType Directory -Path $commandsDir -Force | Out-Null

    $foundAssets = $false

    # Copy skills
    $opencodeSkills = Join-Path $repoPath ".opencode\skills"
    if (Test-Path $opencodeSkills) {
        Write-Host "Found skills in $opencodeSkills" -ForegroundColor Green
        Copy-Item -Path "$opencodeSkills\*" -Destination $skillsDir -Recurse -Force
        $foundAssets = $true
    }

    if (Test-Path (Join-Path $repoPath "skills")) {
        Write-Host "Found skills in $($repoPath)\skills" -ForegroundColor Green
        Copy-Item -Path "$repoPath\skills\*" -Destination $skillsDir -Recurse -Force
        $foundAssets = $true
    }

    # Copy commands
    $opencodeCommands = Join-Path $repoPath ".opencode\commands"
    if (Test-Path $opencodeCommands) {
        Write-Host "Found commands in $opencodeCommands" -ForegroundColor Green
        Copy-Item -Path "$opencodeCommands\*" -Destination $commandsDir -Recurse -Force
        $foundAssets = $true
    }

    if (Test-Path (Join-Path $repoPath "commands")) {
        Write-Host "Found commands in $($repoPath)\commands" -ForegroundColor Green
        Copy-Item -Path "$repoPath\commands\*" -Destination $commandsDir -Recurse -Force
        $foundAssets = $true
    }

    if (-not $foundAssets) {
        Write-Host "No skills/commands found in repo, using default skill sources" -ForegroundColor Yellow

        if (Test-Path $SkillsSourceDir) {
            Copy-Item -Path "$SkillsSourceDir\*" -Destination $skillsDir -Recurse -Force
        }

        if (Test-Path $OpencodeConfigSkills) {
            Copy-Item -Path "$OpencodeConfigSkills\*" -Destination $skillsDir -Recurse -Force
        }
    }

    Write-Host "$workspaceName ready at $workspacePath" -ForegroundColor Green
    
    if (Needs-CliInit -RepoName $repoName) {
        Write-Host "Running CLI init for $repoName..." -ForegroundColor Yellow
        switch ($repoName) {
            "openspec" {
                if (Get-Command openspec -ErrorAction SilentlyContinue) {
                    Push-Location $workspacePath
                    try { openspec init . --tools opencode --force 2>$null } catch { }
                    Pop-Location
                } else {
                    Write-Host "openspec CLI not found, skipping init" -ForegroundColor Yellow
                }
            }
            "speckit" {
                if (Get-Command specify -ErrorAction SilentlyContinue) {
                    Push-Location $workspacePath
                    try { specify init . --ai opencode --here --force 2>$null } catch { }
                    Pop-Location
                } else {
                    Write-Host "specify CLI not found, skipping init" -ForegroundColor Yellow
                }
            }
        }
    }
}

Write-Host ""
Write-Host "=== Committing all ===" -ForegroundColor Cyan

git add -A
git commit -m "Add skill-source and workspace with skills" 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "No changes to commit" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== Setup complete! ===" -ForegroundColor Green
Write-Host ""
Write-Host "Structure:" -ForegroundColor Cyan
Write-Host "  $SkillSourceDir\          - Cloned repositories"
Write-Host "  $workspaceDir\            - workspaces with skills and commands"
Write-Host "    workspace-<name>\"
Write-Host "      .opencode\skills\     - Skills for each project"
Write-Host "      .opencode\commands\   - Commands for each project"
Write-Host ""
Write-Host "To work on a project:" -ForegroundColor Cyan
Write-Host "  cd $SkillSourceDir\<project>    # View source"
Write-Host "  cd $workspaceDir\workspace-<project>  # Work with skills"