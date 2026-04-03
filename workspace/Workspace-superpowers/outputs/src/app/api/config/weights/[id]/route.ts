import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json()
    const { name, context, atomicity, boundary, verifiability, tech, isDefault } = body

    const updateData: Record<string, unknown> = {}
    if (name !== undefined) updateData.name = name
    if (context !== undefined) updateData.context = context
    if (atomicity !== undefined) updateData.atomicity = atomicity
    if (boundary !== undefined) updateData.boundary = boundary
    if (verifiability !== undefined) updateData.verifiability = verifiability
    if (tech !== undefined) updateData.tech = tech

    if (isDefault) {
      await prisma.scoreWeights.updateMany({
        where: { isDefault: true, id: { not: params.id } },
        data: { isDefault: false },
      })
      updateData.isDefault = true
    }

    const weights = await prisma.scoreWeights.update({
      where: { id: params.id },
      data: updateData,
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
    })
  } catch (error) {
    console.error('Update weights error:', error)
    return NextResponse.json({ error: 'Failed to update weights' }, { status: 500 })
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    await prisma.scoreWeights.delete({ where: { id: params.id } })
    return NextResponse.json({ success: true })
  } catch (error) {
    console.error('Delete weights error:', error)
    return NextResponse.json({ error: 'Failed to delete weights' }, { status: 500 })
  }
}
