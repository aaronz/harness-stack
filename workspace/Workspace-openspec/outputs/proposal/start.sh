#!/bin/bash

# AI-Ready Evaluator Start Script

echo "🚀 Starting AI-Ready Evaluator..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if in project root
if [ ! -f "package.json" ]; then
    echo -e "${RED}Error: Please run from project root${NC}"
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed${NC}"
    exit 1
fi

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}Installing dependencies...${NC}"
    npm install --prefer-offline
    cd packages/backend && npm install --prefer-offline && cd ../..
    cd packages/frontend && npm install --prefer-offline && cd ../..
fi

# Check if backend .env exists
if [ ! -f "packages/backend/.env" ]; then
    echo -e "${YELLOW}Warning: packages/backend/.env not found. Creating default...${NC}"
    cp packages/backend/.env.example packages/backend/.env 2>/dev/null || echo "DATABASE_URL=\"file:./dev.db\"\nJWT_SECRET=\"dev-secret-key\"\nPORT=3001" > packages/backend/.env
fi

# Generate Prisma client and push DB
echo -e "${YELLOW}Setting up database...${NC}"
cd packages/backend
npx prisma generate 2>/dev/null
npx prisma db push 2>/dev/null
cd ../..

# Start backend in background
echo -e "${GREEN}Starting backend on port 3001...${NC}"
cd packages/backend
npm run dev &
BACKEND_PID=$!
cd ../..

# Wait for backend to start
sleep 3

# Start frontend in background
echo -e "${GREEN}Starting frontend on port 3000...${NC}"
cd packages/frontend
npm run dev &
FRONTEND_PID=$!
cd ../..

echo ""
echo -e "${GREEN}✅ AI-Ready Evaluator is running!${NC}"
echo "   Frontend: http://localhost:3000"
echo "   Backend:  http://localhost:3001"
echo ""
echo "Press Ctrl+C to stop all services"

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}Stopping services...${NC}"
    kill $BACKEND_PID 2>/dev/null
    kill $FRONTEND_PID 2>/dev/null
    exit 0
}

trap cleanup SIGINT SIGTERM

# Wait for any process to exit
wait