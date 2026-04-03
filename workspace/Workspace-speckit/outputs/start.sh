#!/bin/bash

echo "🚀 Starting AI-Readiness Evaluator..."

if ! command -v npm &> /dev/null; then
    echo "Error: npm not found. Please install Node.js first."
    exit 1
fi

echo "🧹 Stopping any existing services..."
pkill -f "tsx src/server" 2>/dev/null
pkill -f "next dev" 2>/dev/null
sleep 1

if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
    if [ $? -ne 0 ]; then
        echo "Error: npm install failed"
        exit 1
    fi
fi

if [ ! -d "data" ]; then
    echo "📁 Creating data directory..."
    mkdir -p data
fi

cleanup() {
    echo "🛑 Shutting down..."
    if [ ! -z "$SERVER_PID" ]; then kill $SERVER_PID 2>/dev/null; fi
    if [ ! -z "$NEXT_PID" ]; then kill $NEXT_PID 2>/dev/null; fi
    exit 0
}

trap cleanup SIGINT SIGTERM

echo "🔧 Starting backend server on port 3001..."
npm run server &
SERVER_PID=$!

sleep 3

echo "🌐 Starting frontend on port 3000..."
npm run dev &
NEXT_PID=$!

echo ""
echo "✅ AI-Readiness Evaluator is running!"
echo "   Frontend: http://localhost:3000"
echo "   Backend:  http://localhost:3001"
echo ""
echo "Press Ctrl+C to stop both servers"

wait