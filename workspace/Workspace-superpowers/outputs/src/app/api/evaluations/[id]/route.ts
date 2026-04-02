import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const evaluation = await prisma.evaluation.findUnique({
      where: { id: params.id },
    })
    
    if (!evaluation) {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }
    
    return NextResponse.json({
      id: evaluation.id,
      title: evaluation.title,
      content: evaluation.content,
      fileName: evaluation.fileName,
      fileType: evaluation.fileType,
      overallScore: evaluation.overallScore,
      grade: evaluation.grade,
      complexity: evaluation.complexity,
      contextScore: evaluation.contextScore,
      atomicityScore: evaluation.atomicityScore,
      boundaryScore: evaluation.boundaryScore,
      verifiabilityScore: evaluation.verifiabilityScore,
      techScore: evaluation.techScore,
      rawResponse: evaluation.rawResponse,
      suggestions: JSON.parse(evaluation.suggestions),
      modelUsed: evaluation.modelUsed,
      createdAt: evaluation.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Get evaluation error:', error)
    return NextResponse.json({ error: 'Failed to get evaluation' }, { status: 500 })
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    await prisma.evaluation.delete({
      where: { id: params.id },
    })
    
    return NextResponse.json({ success: true })
  } catch (error) {
    console.error('Delete evaluation error:', error)
    return NextResponse.json({ error: 'Failed to delete evaluation' }, { status: 500 })
  }
}
