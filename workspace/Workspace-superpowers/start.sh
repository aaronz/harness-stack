#!/bin/bash

# AI-Ready Evaluator Startup Script

set -e

echo "🚀 Starting AI-Ready Evaluator..."

cd "$(dirname "$0")/outputs"

# Check if .env exists
if [ ! -f .env ]; then
    echo "⚠️  No .env file found. Creating from example..."
    if [ -f .env.example ]; then
        cp .env.example .env
        echo "📝 Please edit .env and add your API keys"
    else
        echo "❌ No .env.example found. Please create .env manually"
    fi
fi

# Push database schema if needed
echo "📦 Checking database..."
npx prisma db push --skip-generate 2>/dev/null || true

# Start dev server
echo "🎮 Starting dev server on http://localhost:3000"
npm run dev
