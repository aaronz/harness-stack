#!/usr/bin/env bash
set -e

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

needs_cli_init() {
    local method="$1"
    for m in "${CLI_INIT_METHODOLOGIES[@]}"; do
        if [[ "$m" == "$method" ]]; then
            return 0
        fi
    done
    return 1
}

install_methodology() {
    local method="$1"
    local target="$2"
    local clean="$3"
    
    local source_dir="$SKILL_SOURCE_DIR/$method"
    
    if [[ ! -d "$source_dir" ]]; then
        echo -e "${RED}Error: Source directory not found: $source_dir${NC}"
        exit 1
    fi
    
    local target_abs
    target_abs="$(cd "$target" && pwd)"
    
    if [[ ! -d "$target_abs" ]]; then
        echo -e "${YELLOW}Target directory does not exist. Creating: $target_abs${NC}"
        mkdir -p "$target_abs"
    fi
    
    echo -e "${BLUE}Installing methodology: ${method}${NC}"
    echo -e "  Source: $source_dir"
    echo -e "  Target: $target_abs"
    echo ""
    
    if needs_cli_init "$method"; then
        if [[ "$clean" == true ]]; then
            mkdir -p "$target_abs/.opencode/skills"
            mkdir -p "$target_abs/.opencode/commands"
            echo "Cleaning existing .opencode/skills, .opencode/commands, scripts"
            rm -rf "$target_abs/.opencode/skills"/* 2>/dev/null || true
            rm -rf "$target_abs/.opencode/commands"/* 2>/dev/null || true
            for script in "${SCRIPT_FILES[@]}"; do
                rm -f "$target_abs/$script" 2>/dev/null || true
            done
        fi
        
        case "$method" in
            openspec)
                if command -v openspec &> /dev/null; then
                    cd "$target_abs" && openspec init . --tools opencode --force 2>/dev/null || true
                    echo -e "${GREEN}openspec init completed${NC}"
                else
                    echo -e "${YELLOW}openspec CLI not found, installing...${NC}"
                    npm install -g @fission-ai/openspec 2>/dev/null || true
                    if command -v openspec &> /dev/null; then
                        cd "$target_abs" && openspec init . --tools opencode --force 2>/dev/null || true
                        echo -e "${GREEN}openspec init completed${NC}"
                    else
                        echo -e "${YELLOW}Failed to install openspec, skipping init${NC}"
                    fi
                fi
                ;;
            speckit)
                if command -v specify &> /dev/null; then
                    cd "$target_abs" && specify init . --ai opencode --here --force 2>/dev/null || true
                    echo -e "${GREEN}specify init completed${NC}"
                else
                    echo -e "${YELLOW}specify CLI not found, installing...${NC}"
                    installed=false
                    if command -v uv &> /dev/null; then
                        uv tool install specify-cli --from git+https://github.com/github/spec-kit.git 2>/dev/null && installed=true
                    fi
                    if [[ "$installed" == false ]] && command -v pip &> /dev/null; then
                        pip install specify-cli 2>/dev/null && installed=true
                    fi
                    if [[ "$installed" == false ]] && command -v pip3 &> /dev/null; then
                        pip3 install specify-cli 2>/dev/null && installed=true
                    fi
                    if command -v specify &> /dev/null; then
                        cd "$target_abs" && specify init . --ai opencode --here --force 2>/dev/null || true
                        echo -e "${GREEN}specify init completed${NC}"
                    else
                        echo -e "${YELLOW}Failed to install specify, skipping init${NC}"
                    fi
                fi
                ;;
        esac
        
        if [[ -d "$WORKSPACE_DIR/Workspace-$method" ]]; then
            echo "Copying scripts from workspace"
            for script in "${SCRIPT_FILES[@]}"; do
                if [[ -f "$WORKSPACE_DIR/Workspace-$method/$script" ]]; then
                    cp "$WORKSPACE_DIR/Workspace-$method/$script" "$target_abs/"
                fi
            done
        fi
    else
        mkdir -p "$target_abs/.opencode/skills"
        mkdir -p "$target_abs/.opencode/commands"
        
        if [[ "$CLEAN" == true ]]; then
            echo "Cleaning existing .opencode/skills and .opencode/commands"
            rm -rf "$target_abs/.opencode/skills"/* 2>/dev/null || true
            rm -rf "$target_abs/.opencode/commands"/* 2>/dev/null || true
            for script in "${SCRIPT_FILES[@]}"; do
                rm -f "$target_abs/$script" 2>/dev/null || true
            done
        fi
        
        found_assets=false
        
        if [[ -d "$source_dir/.opencode/skills" ]]; then
            echo "Copying skills from $source_dir/.opencode/skills"
            cp -r "$source_dir/.opencode/skills"/* "$target_abs/.opencode/skills/" 2>/dev/null || true
            found_assets=true
        fi
        
        if [[ -d "$source_dir/skills" ]]; then
            echo "Copying skills from $source_dir/skills"
            cp -r "$source_dir/skills"/* "$target_abs/.opencode/skills/" 2>/dev/null || true
            found_assets=true
        fi
        
        if [[ "$method" == "gstack" ]]; then
            echo "Copying gstack skills from repo root"
            for dir in "$source_dir"/*/; do
                if [[ -f "$dir/SKILL.md" ]]; then
                    dir_name=$(basename "$dir")
                    mkdir -p "$target_abs/.opencode/skills/$dir_name"
                    cp -r "$dir"* "$target_abs/.opencode/skills/$dir_name/" 2>/dev/null || true
                    found_assets=true
                fi
            done
        fi
        
        if [[ -d "$source_dir/.opencode/commands" ]]; then
            echo "Copying commands from $source_dir/.opencode/commands"
            cp -r "$source_dir/.opencode/commands"/* "$target_abs/.opencode/commands/" 2>/dev/null || true
            found_assets=true
        fi
        
        if [[ -d "$source_dir/commands" ]]; then
            echo "Copying commands from $source_dir/commands"
            cp -r "$source_dir/commands"/* "$target_abs/.opencode/commands/" 2>/dev/null || true
            found_assets=true
        fi
        
        if [[ "$found_assets" == false ]]; then
            echo -e "${YELLOW}No skills/commands found in repo${NC}"
        fi
        
        if [[ -d "$WORKSPACE_DIR/Workspace-$method" ]]; then
            echo "Copying scripts from workspace"
            for script in "${SCRIPT_FILES[@]}"; do
                if [[ -f "$WORKSPACE_DIR/Workspace-$method/$script" ]]; then
                    cp "$WORKSPACE_DIR/Workspace-$method/$script" "$target_abs/"
                fi
            done
        fi
    fi
    
    echo -e "${GREEN}Successfully installed $method to $target_abs${NC}"
}

CLEAN=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        -c|--clean)
            CLEAN=true
            shift
            ;;
        -*)
            echo -e "${RED}Error: Unknown option: $1${NC}"
            print_usage
            exit 1
            ;;
        *)
            break
            ;;
    esac
done

if [[ $# -lt 2 ]]; then
    print_usage
    exit 1
fi

METHODOLOGY="$1"
TARGET="$2"

valid=false
for m in "${METHODOLOGIES[@]}"; do
    if [[ "$m" == "$METHODOLOGY" ]]; then
        valid=true
        break
    fi
done

if [[ "$valid" == false ]]; then
    echo -e "${RED}Error: Unknown methodology: $METHODOLOGY${NC}"
    echo ""
    print_usage
    exit 1
fi

install_methodology "$METHODOLOGY" "$TARGET" "$CLEAN"
