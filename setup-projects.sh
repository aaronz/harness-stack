#!/bin/bash

set -e

PROJECTS=(
    "speckit:https://github.com/github/spec-kit.git"
    "openspec:https://github.com/Fission-AI/OpenSpec.git"
    "superpowers:https://github.com/obra/superpowers.git"
    "everything-claude-code:https://github.com/ysyecust/everything-claude-code.git"
    "planning-with-files:https://github.com/OthmanAdi/planning-with-files.git"
    "gstack:https://github.com/garrytan/gstack.git"
)

CLI_INIT_PROJECTS=("openspec" "speckit")

needs_cli_init() {
    local repo_name="$1"
    for cli_proj in "${CLI_INIT_PROJECTS[@]}"; do
        if [ "$repo_name" = "$cli_proj" ]; then
            return 0
        fi
    done
    return 1
}

SKILLS_SOURCE_DIR="$HOME/.cache/opencode/node_modules/superpowers/skills"
OPENCODE_CONFIG_SKILLS="$HOME/.config/opencode/skills"
SKILL_SOURCE_DIR="skill-source"
WORKSPACE_DIR="workspace"
SETUP_SCRIPT="$0"

WORKSPACE="$(cd "$(dirname "$0")" && pwd)"

fix_yaml_tools() {
    local file="$1"
    if [ ! -f "$file" ]; then
        return
    fi
    if grep -q "^tools: [A-Z]" "$file" 2>/dev/null; then
        sed -i '' 's/^tools: \(.*\)$/tools: [\1]/' "$file"
        sed -i '' 's/\[/["/g; s/\]/"]/g; s/, /", "/g' "$file"
    fi
}

echo "=== Initializing git repository in $WORKSPACE ==="
cd "$WORKSPACE"

if [ ! -d ".git" ]; then
    git init
    git checkout -b main
else
    echo "Git repo already exists"
    if ! git show-ref --verify --quiet refs/heads/main; then
        git checkout -b main
    fi
fi

echo ""
echo "=== Cloning repos into $SKILL_SOURCE_DIR ==="

for project in "${PROJECTS[@]}"; do
    IFS=':' read -r repo_name repo_url <<< "$project"
    target_dir="$WORKSPACE/$SKILL_SOURCE_DIR/$repo_name"
    
    echo ""
    echo "--- Cloning $repo_name ---"
    
    mkdir -p "$(dirname "$target_dir")"
    mkdir -p "$target_dir"
    
    temp_clone="/tmp/${repo_name}_clone"
    rm -rf "$temp_clone"
    git clone --depth 1 "$repo_url" "$temp_clone"
    
    rsync -av "$temp_clone/" "$target_dir/" 2>/dev/null || cp -r "$temp_clone"/* "$target_dir/" 2>/dev/null || true
    rm -rf "$temp_clone"
    
    file_count=$(ls -1 "$target_dir" | wc -l)
    echo "$repo_name ready at $target_dir ($file_count files)"
done

echo ""
echo "=== Creating workspaces with skills and commands ==="

for project in "${PROJECTS[@]}"; do
    IFS=':' read -r repo_name repo_url <<< "$project"
    
    workspace_name="workspace-$repo_name"
    workspace_path="$WORKSPACE/$WORKSPACE_DIR/$workspace_name"
    repo_path="$WORKSPACE/$SKILL_SOURCE_DIR/$repo_name"
    
    echo ""
    echo "--- Setting up $workspace_name ---"
    
    mkdir -p "$workspace_path/.opencode/skills"
    mkdir -p "$workspace_path/.opencode/commands"
    
    found_assets=false
    
    if [ -d "$repo_path/.opencode/skills" ]; then
        echo "Found skills in $repo_path/.opencode/skills"
        cp -r "$repo_path/.opencode/skills"/* "$workspace_path/.opencode/skills/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [ -d "$repo_path/skills" ]; then
        echo "Found skills in $repo_path/skills"
        cp -r "$repo_path/skills"/* "$workspace_path/.opencode/skills/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [ -d "$repo_path/.opencode/commands" ]; then
        echo "Found commands in $repo_path/.opencode/commands"
        cp -r "$repo_path/.opencode/commands"/* "$workspace_path/.opencode/commands/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [ -d "$repo_path/commands" ]; then
        echo "Found commands in $repo_path/commands"
        cp -r "$repo_path/commands"/* "$workspace_path/.opencode/commands/" 2>/dev/null || true
        found_assets=true
    fi
    
    if [ "$found_assets" = false ]; then
        echo "No skills/commands found in repo, using default skill sources"
        
        if [ -d "$SKILLS_SOURCE_DIR" ]; then
            cp -r "$SKILLS_SOURCE_DIR"/* "$workspace_path/.opencode/skills/" 2>/dev/null || true
        fi
        
        if [ -d "$OPENCODE_CONFIG_SKILLS" ]; then
            cp -r "$OPENCODE_CONFIG_SKILLS"/* "$workspace_path/.opencode/skills/" 2>/dev/null || true
        fi
    fi
    
    echo "$workspace_name ready at $workspace_path"
    
    if needs_cli_init "$repo_name"; then
        echo "Running CLI init for $repo_name..."
        case "$repo_name" in
            openspec)
                if command -v openspec &> /dev/null; then
                    cd "$workspace_path" && openspec init . --tools opencode --force 2>/dev/null || true
                else
                    echo "openspec CLI not found, skipping init"
                fi
                ;;
            speckit)
                if command -v specify &> /dev/null; then
                    cd "$workspace_path" && specify init . --ai opencode --here --force 2>/dev/null || true
                else
                    echo "specify CLI not found, skipping init"
                fi
                ;;
        esac
    fi
done

echo ""
echo "=== Committing all ==="

git add -A
git commit -m "Add skill-source and workspace with skills" 2>/dev/null || echo "No changes to commit"

echo ""
echo "=== Setup complete! ==="
echo ""
echo "Structure:"
echo "  $SKILL_SOURCE_DIR/          - Cloned repositories"
echo "  $WORKSPACE_DIR/            - workspaces with skills and commands"
echo "    workspace-<name>/"
echo "      .opencode/skills/      - Skills for each project"
echo "      .opencode/commands/    - Commands for each project"
echo ""
echo "To work on a project:"
echo "  cd $SKILL_SOURCE_DIR/<project>    # View source"
echo "  cd $WORKSPACE_DIR/workspace-<project>  # Work with skills"