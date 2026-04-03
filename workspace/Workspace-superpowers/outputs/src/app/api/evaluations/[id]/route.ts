import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { evaluateRequirement } from '@/lib/evaluator'
import { decryptApiKey } from '@/lib/encryption'
import { LLMConfig } from '@/lib/llm'

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

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json()
    const { title, content, modelId } = body
    
    const existing = await prisma.evaluation.findUnique({
      where: { id: params.id },
    })
    
    if (!existing) {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }
    
    let result
    let modelUsed = existing.modelUsed
    if (content && content !== existing.content) {
      let llmConfig: LLMConfig
      if (modelId) {
        const config = await prisma.lLMConfig.findUnique({ where: { id: modelId } })
        if (!config) {
          return NextResponse.json({ error: 'Invalid model ID' }, { status: 400 })
        }
        llmConfig = {
          provider: config.provider as LLMConfig['provider'],
          model: config.model,
          apiKey: decryptApiKey(config.apiKey),
          baseUrl: config.baseUrl || undefined,
        }
      } else {
        const defaultConfig = await prisma.lLMConfig.findFirst({ where: { isDefault: true } })
        if (!defaultConfig) {
          return NextResponse.json({ error: 'No default LLM configured' }, { status: 400 })
        }
        llmConfig = {
          provider: defaultConfig.provider as LLMConfig['provider'],
          model: defaultConfig.model,
          apiKey: decryptApiKey(defaultConfig.apiKey),
          baseUrl: defaultConfig.baseUrl || undefined,
        }
      }
      
      result = await evaluateRequirement(content, llmConfig)
      modelUsed = llmConfig.model
    }
    
    const updateData: any = {
      title: title || existing.title,
    }
    
    if (result) {
      updateData.content = content
      updateData.overallScore = result.overallScore
      updateData.grade = result.grade
      updateData.complexity = result.complexity
      updateData.contextScore = result.scores.context
      updateData.atomicityScore = result.scores.atomicity
      updateData.boundaryScore = result.scores.boundary
      updateData.verifiabilityScore = result.scores.verifiability
      updateData.techScore = result.scores.tech
      updateData.rawResponse = JSON.stringify(result)
      updateData.suggestions = JSON.stringify(result.suggestions)
      updateData.modelUsed = modelUsed
    }
    
    const evaluation = await prisma.evaluation.update({
      where: { id: params.id },
      data: updateData,
    })
    
    return NextResponse.json({
      id: evaluation.id,
      title: evaluation.title,
      overallScore: evaluation.overallScore,
      grade: evaluation.grade,
      contextScore: evaluation.contextScore,
      atomicityScore: evaluation.atomicityScore,
      boundaryScore: evaluation.boundaryScore,
      verifiabilityScore: evaluation.verifiabilityScore,
      techScore: evaluation.techScore,
      suggestions: result ? result.suggestions : JSON.parse(evaluation.suggestions),
      createdAt: evaluation.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Update evaluation error:', error)
    return NextResponse.json({ error: 'Failed to update evaluation' }, { status: 500 })
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
