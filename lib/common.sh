#!/usr/bin/env bash
# Common functions for methodology scripts

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILL_SOURCE_DIR="$SCRIPT_DIR/skill-source"
WORKSPACE_DIR="$SCRIPT_DIR/workspace"

METHODOLOGIES=("openspec" "gstack" "planning-with-files" "speckit" "superpowers" "everything-claude-code")
CLI_INIT_METHODOLOGIES=("openspec" "speckit")

SCRIPT_FILES=(
    "implement-prd.sh"
    "implement-prd.ps1"
    "iterate-prd.sh"
    "iterate-prd.ps1"
    "prompts.md"
)

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

needs_cli_init() {
    local method="$1"
    for m in "${CLI_INIT_METHODOLOGIES[@]}"; do
        if [[ "$m" == "$method" ]]; then
            return 0
        fi
    done
    return 1
}

print_usage() {
    echo "Usage: $0 [options] <methodology> <target-folder>"
    echo ""
    echo "Options:"
    echo "  -c, --clean    Clean .opencode/skills and .opencode/commands before install"
    echo ""
    echo "Available methodologies:"
    for m in "${METHODOLOGIES[@]}"; do
        echo "  - $m"
    done
    echo ""
    echo "Examples:"
    echo "  $0 superpowers ~/projects/my-app"
    echo "  $0 speckit ./my-workspace"
    echo "  $0 --clean planning-with-files ../other-project"
}

clean_target() {
    local target="$1"
    local skills_dir="$target/.opencode/skills"
    local commands_dir="$target/.opencode/commands"
    
    mkdir -p "$skills_dir"
    mkdir -p "$commands_dir"
    
    echo "Cleaning existing .opencode/skills, .opencode/commands, scripts"
    rm -rf "$skills_dir"/* 2>/dev/null || true
    rm -rf "$commands_dir"/* 2>/dev/null || true
    for script in "${SCRIPT_FILES[@]}"; do
        rm -f "$target/$script" 2>/dev/null || true
    done
}

copy_scripts() {
    local method="$1"
    local target="$2"
    local workspace_path="$WORKSPACE_DIR/Workspace-$method"
    
    if [[ -d "$workspace_path" ]]; then
        echo "Copying scripts from workspace"
        for script in "${SCRIPT_FILES[@]}"; do
            if [[ -f "$workspace_path/$script" ]]; then
                cp "$workspace_path/$script" "$target/"
            fi
        done
    fi
}

copy_skills_commands() {
    local method="$1"
    local source_dir="$SKILL_SOURCE_DIR/$method"
    local target="$2"
    
    local skills_dir="$target/.opencode/skills"
    local commands_dir="$target/.opencode/commands"
    
    mkdir -p "$skills_dir"
    mkdir -p "$commands_dir"
    
    local found_assets=false
    
    if [[ -d "$source_dir/.opencode/skills" ]]; then
        echo "Copying skills from $source_dir/.opencode/skills"
        cp -r "$source_dir/.opencode/skills"/* "$skills_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ -d "$source_dir/skills" ]]; then
        echo "Copying skills from $source_dir/skills"
        cp -r "$source_dir/skills"/* "$skills_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ "$method" == "gstack" ]]; then
        echo "Copying gstack skills from repo root"
        for dir in "$source_dir"/*/; do
            if [[ -f "$dir/SKILL.md" ]]; then
                dir_name=$(basename "$dir")
                mkdir -p "$skills_dir/$dir_name"
                cp -r "$dir"* "$skills_dir/$dir_name/" 2>/dev/null || true
                found_assets=true
            fi
        done
    fi
    
    if [[ -d "$source_dir/.opencode/commands" ]]; then
        echo "Copying commands from $source_dir/.opencode/commands"
        cp -r "$source_dir/.opencode/commands"/* "$commands_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ -d "$source_dir/commands" ]]; then
        echo "Copying commands from $source_dir/commands"
        cp -r "$source_dir/commands"/* "$commands_dir/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [[ "$found_assets" == false ]]; then
        echo -e "${YELLOW}No skills/commands found in repo${NC}"
    fi
}

install_openspec() {
    local target="$1"
    
    if command -v openspec &> /dev/null; then
        cd "$target" && openspec init . --tools opencode --force 2>/dev/null || true
        echo -e "${GREEN}openspec init completed${NC}"
    else
        echo -e "${YELLOW}openspec CLI not found, installing...${NC}"
        npm install -g @fission-ai/openspec 2>/dev/null || true
        if command -v openspec &> /dev/null; then
            cd "$target" && openspec init . --tools opencode --force 2>/dev/null || true
            echo -e "${GREEN}openspec init completed${NC}"
        else
            echo -e "${YELLOW}Failed to install openspec, skipping init${NC}"
        fi
    fi
}

install_speckit() {
    local target="$1"
    
    if command -v specify &> /dev/null; then
        cd "$target" && specify init . --ai opencode --here --force 2>/dev/null || true
        echo -e "${GREEN}specify init completed${NC}"
    else
        echo -e "${YELLOW}specify CLI not found, installing...${NC}"
        installed=false
        
        if command -v uv &> /dev/null; then
            uv tool install specify-cli --from git+https://github.com/github/spec-kit.git 2>/dev/null && installed=true
        fi
        
        if [[ "$installed" == false ]] && command -v pipx &> /dev/null; then
            pipx install specify-cli 2>/dev/null && installed=true
        fi
        
        if [[ "$installed" == false ]] && command -v pip &> /dev/null; then
            pip install specify-cli 2>/dev/null && installed=true
        fi
        
        if [[ "$installed" == false ]] && command -v pip3 &> /dev/null; then
            pip3 install specify-cli 2>/dev/null && installed=true
        fi
        
        if command -v specify &> /dev/null; then
            cd "$target" && specify init . --ai opencode --here --force 2>/dev/null || true
            echo -e "${GREEN}specify init completed${NC}"
        else
            echo -e "${YELLOW}Failed to install specify, skipping init${NC}"
        fi
    fi
}

validate_methodology() {
    local method="$1"
    for m in "${METHODOLOGIES[@]}"; do
        if [[ "$m" == "$method" ]]; then
            return 0
        fi
    done
    return 1
}
