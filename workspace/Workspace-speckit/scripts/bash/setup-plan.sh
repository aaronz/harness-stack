#!/bin/bash
# Setup script for speckit.plan - generates JSON for feature planning
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Parse arguments
SHORT_NAME=""
JSON_OUTPUT=false
TIMESTAMP=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --json)
      JSON_OUTPUT=true
      shift
      ;;
    --short-name)
      SHORT_NAME="$2"
      shift 2
      ;;
    --timestamp)
      TIMESTAMP=true
      shift
      ;;
    *)
      shift
      ;;
  esac
done

# Get feature spec path
SPEC_DIR="$REPO_ROOT/.specify/specs"
if [ -n "$SHORT_NAME" ]; then
  FEATURE_DIR="$SPEC_DIR/feature-$SHORT_NAME"
else
  # Find the latest feature directory
  FEATURE_DIR=$(ls -td "$SPEC_DIR"/feature-* 2>/dev/null | head -1)
fi

FEATURE_SPEC="$FEATURE_DIR/spec.md"
IMPL_PLAN="$FEATURE_DIR/plan.md"
SPECS_DIR="$FEATURE_DIR"
BRANCH="feature/$SHORT_NAME"

# Get git branch
if [ -d "$REPO_ROOT/.git" ]; then
  CURRENT_BRANCH=$(cd "$REPO_ROOT" && git branch --show-current 2>/dev/null || echo "")
  if [ -n "$CURRENT_BRANCH" ]; then
    BRANCH="$CURRENT_BRANCH"
  fi
fi

if [ "$JSON_OUTPUT" = true ]; then
  cat << EOF
{
  "FEATURE_SPEC": "$FEATURE_SPEC",
  "IMPL_PLAN": "$IMPL_PLAN",
  "SPECS_DIR": "$SPECS_DIR",
  "BRANCH": "$BRANCH"
}
EOF
else
  echo "FEATURE_SPEC=$FEATURE_SPEC"
  echo "IMPL_PLAN=$IMPL_PLAN"
  echo "SPECS_DIR=$SPECS_DIR"
  echo "BRANCH=$BRANCH"
fi
