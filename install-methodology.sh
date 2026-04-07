#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/common.sh"

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
            clean_target "$target_abs"
        fi
        
        case "$method" in
            openspec)
                install_openspec "$target_abs"
                ;;
            speckit)
                install_speckit "$target_abs"
                ;;
        esac
        
        copy_scripts "$method" "$target_abs"
    else
        if [[ "$clean" == true ]]; then
            clean_target "$target_abs"
        fi
        
        copy_skills_commands "$method" "$target_abs"
        copy_scripts "$method" "$target_abs"
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

if ! validate_methodology "$METHODOLOGY"; then
    echo -e "${RED}Error: Unknown methodology: $METHODOLOGY${NC}"
    echo ""
    print_usage
    exit 1
fi

install_methodology "$METHODOLOGY" "$TARGET" "$CLEAN"
