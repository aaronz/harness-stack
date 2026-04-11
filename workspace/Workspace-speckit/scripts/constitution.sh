#!/bin/bash

DEFAULT_CONSTITUTION="Use default Constitution"

load_constitution() {
    local constitution_path="${1:-$CONSTITUTION_PATH}"
    
    if [ -f "$constitution_path" ]; then
        cat "$constitution_path"
    else
        echo "$DEFAULT_CONSTITUTION"
    fi
}

load_constitution_cached() {
    if [ -z "$_CONSTITUTION_CACHED" ]; then
        _CONSTITUTION_CACHED=$(load_constitution "$CONSTITUTION_PATH")
    fi
    echo "$_CONSTITUTION_CACHED"
}

validate_constitution() {
    local constitution_path="${1:-$CONSTITUTION_PATH}"
    
    if [ ! -f "$constitution_path" ]; then
        echo "Warning: Constitution file not found: $constitution_path"
        return 1
    fi
    
    local size=$(wc -c < "$constitution_path")
    if [ "$size" -lt 10 ]; then
        echo "Warning: Constitution file content too small: $size bytes"
        return 1
    fi
    
    echo "Constitution valid: $constitution_path ($size bytes)"
    return 0
}

get_constitution_summary() {
    local constitution_path="${1:-$CONSTITUTION_PATH}"
    
    if [ ! -f "$constitution_path" ]; then
        echo "No Constitution"
        return
    fi
    
    local lines=$(wc -l < "$constitution_path")
    local size=$(wc -c < "$constitution_path")
    echo "Constitution: ${lines} lines, ${size} bytes"
}
