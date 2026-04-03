import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function GET() {
  try {
    const weights = await prisma.scoreWeights.findMany({
      orderBy: { createdAt: 'desc' },
    })
    
    return NextResponse.json(weights.map(w => ({
      id: w.id,
      name: w.name,
      context: w.context,
      atomicity: w.atomicity,
      boundary: w.boundary,
      verifiability: w.verifiability,
      tech: w.tech,
      isDefault: w.isDefault,
      createdAt: w.createdAt.toISOString(),
    })))
  } catch (error) {
    console.error('List weights error:', error)
    return NextResponse.json({ error: 'Failed to list weights' }, { status: 500 })
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json()
    const { name, context, atomicity, boundary, verifiability, tech, isDefault } = body
    
    if (!name) {
      return NextResponse.json({ error: 'Missing required field: name' }, { status: 400 })
    }
    
    const total = (context || 0.25) + (atomicity || 0.25) + (boundary || 0.20) + 
                  (verifiability || 0.15) + (tech || 0.15)
    if (Math.abs(total - 1.0) > 0.01) {
      return NextResponse.json({ error: 'Weights must sum to 1.0' }, { status: 400 })
    }
    
    if (isDefault) {
      await prisma.scoreWeights.updateMany({
        where: { isDefault: true },
        data: { isDefault: false },
      })
    }
    
    const weights = await prisma.scoreWeights.create({
      data: {
        name,
        context: context || 0.25,
        atomicity: atomicity || 0.25,
        boundary: boundary || 0.20,
        verifiability: verifiability || 0.15,
        tech: tech || 0.15,
        isDefault: isDefault || false,
      },
    })
    
    return NextResponse.json({
      id: weights.id,
      name: weights.name,
      context: weights.context,
      atomicity: weights.atomicity,
      boundary: weights.boundary,
      verifiability: weights.verifiability,
      tech: weights.tech,
      isDefault: weights.isDefault,
      createdAt: weights.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Create weights error:', error)
    return NextResponse.json({ error: 'Failed to create weights' }, { status: 500 })
  }
}