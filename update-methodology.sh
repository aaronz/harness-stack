#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILL_SOURCE_DIR="$SCRIPT_DIR/skill-source"
WORKSPACE_DIR="$SCRIPT_DIR/workspace"

source "$SCRIPT_DIR/lib/common.sh"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_usage() {
    echo "Usage: $0 <methodology>"
    echo ""
    echo "Updates a methodology from its source repository."
    echo ""
    echo "Available methodologies:"
    for m in "${METHODOLOGIES[@]}"; do
        echo "  - $m"
    done
    echo ""
    echo "Examples:"
    echo "  $0 superpowers"
    echo "  $0 speckit"
    echo "  $0 openspec"
}

update_methodology() {
    local method="$1"
    local source_dir="$SKILL_SOURCE_DIR/$method"
    local workspace_path="$WORKSPACE_DIR/Workspace-$method"
    
    if [[ ! -d "$source_dir" ]]; then
        echo -e "${RED}Error: Source directory not found: $source_dir${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}Updating methodology: ${method}${NC}"
    echo ""
    
    if [[ -d "$source_dir/.git" ]]; then
        echo "Pulling latest from skill-source/$method"
        cd "$source_dir"
        git fetch origin
        git pull origin main 2>/dev/null || git pull origin master 2>/dev/null || true
        echo -e "${GREEN}Updated skill-source/$method${NC}"
    else
        echo -e "${YELLOW}Not a git repository, skipping pull${NC}"
    fi
    
    if [[ -d "$workspace_path" ]]; then
        if [[ -d "$workspace_path/.git" ]]; then
            echo ""
            echo "Pulling latest from workspace/$method"
            cd "$workspace_path"
            git fetch origin
            git pull origin main 2>/dev/null || git pull origin master 2>/dev/null || true
            echo -e "${GREEN}Updated workspace/$method${NC}"
        else
            echo -e "${YELLOW}Workspace not a git repository, skipping pull${NC}"
        fi
    fi
    
    echo ""
    echo -e "${GREEN}Successfully updated $method${NC}"
}

update_all() {
    echo -e "${BLUE}Updating all methodologies...${NC}"
    echo ""
    
    for method in "${METHODOLOGIES[@]}"; do
        update_methodology "$method"
    done
    
    echo -e "${GREEN}All methodologies updated${NC}"
}

if [[ $# -lt 1 ]]; then
    print_usage
    exit 1
fi

if [[ "$1" == "--all" ]]; then
    update_all
    exit 0
fi

METHODOLOGY="$1"

if ! validate_methodology "$METHODOLOGY"; then
    echo -e "${RED}Error: Unknown methodology: $METHODOLOGY${NC}"
    echo ""
    print_usage
    exit 1
fi

update_methodology "$METHODOLOGY"
