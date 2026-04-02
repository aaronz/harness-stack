#!/bin/bash

# AI Ready Evaluator - One-Click Startup Script
# ==============================================
# This script starts both the backend and frontend services
# for local development.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed.${NC}"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm is not installed.${NC}"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Function to check if dependencies are installed
check_deps() {
    local dir=$1
    local name=$2
    if [ ! -d "$dir/node_modules" ]; then
        echo -e "${YELLOW}Installing $name dependencies...${NC}"
        cd "$dir"
        npm install
        cd "$SCRIPT_DIR"
    else
        echo -e "${GREEN}$name dependencies already installed.${NC}"
    fi
}

# Function to cleanup background processes on exit
cleanup() {
    echo -e "\n${YELLOW}Shutting down services...${NC}"
    kill $BACKEND_PID $FRONTEND_PID 2>/dev/null
    wait $BACKEND_PID $FRONTEND_PID 2>/dev/null
    echo -e "${GREEN}All services stopped.${NC}"
    exit 0
}

trap cleanup SIGINT SIGTERM

# Check and install dependencies
check_deps "$SCRIPT_DIR/backend" "Backend"
check_deps "$SCRIPT_DIR/frontend" "Frontend"

# Copy .env.example to .env if not exists
if [ ! -f "$SCRIPT_DIR/backend/.env" ]; then
    echo -e "${YELLOW}Creating backend .env file...${NC}"
    cp "$SCRIPT_DIR/backend/.env.example" "$SCRIPT_DIR/backend/.env"
    echo -e "${GREEN}.env file created. Please update with your settings.${NC}"
fi

# Start backend
echo -e "${GREEN}Starting backend server...${NC}"
cd "$SCRIPT_DIR/backend"
npm run dev &
BACKEND_PID=$!

# Wait a moment for backend to start
sleep 2

# Start frontend
echo -e "${GREEN}Starting frontend server...${NC}"
cd "$SCRIPT_DIR/frontend"
npm run dev &
FRONTEND_PID=$!

# Wait a moment for services to start
sleep 3

# Print service URLs
echo ""
echo "============================================"
echo -e "${GREEN}🚀 AI Ready Evaluator is running!${NC}"
echo "============================================"
echo ""
echo -e "${YELLOW}Backend:${NC}  http://localhost:3001"
echo -e "${YELLOW}Frontend:${NC} http://localhost:3000"
echo ""
echo "Press Ctrl+C to stop all services"
echo "============================================"
echo ""

# Wait for both processes
wait $BACKEND_PID $FRONTEND_PID
