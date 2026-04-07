import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { evaluateRequirement } from '@/lib/evaluator'
import { decryptApiKey } from '@/lib/encryption'
import { LLMConfig } from '@/lib/llm'

export async function POST(request: NextRequest) {
  try {
    const formData = await request.formData()
    const file = formData.get('file') as File | null
    const title = formData.get('title') as string | null
    const modelId = formData.get('modelId') as string | null
    
    if (!file) {
      return NextResponse.json({ error: 'No file provided' }, { status: 400 })
    }
    
    const buffer = await file.arrayBuffer()
    const content = Buffer.from(buffer).toString('utf-8')
    
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
        return NextResponse.json({ 
          error: 'No default LLM configured. Please add a model in Settings.' 
        }, { status: 400 })
      }
      llmConfig = {
        provider: defaultConfig.provider as LLMConfig['provider'],
        model: defaultConfig.model,
        apiKey: decryptApiKey(defaultConfig.apiKey),
        baseUrl: defaultConfig.baseUrl || undefined,
      }
    }
    
    const result = await evaluateRequirement(content, llmConfig)
    
    const evaluation = await prisma.evaluation.create({
      data: {
        title: title || file.name.replace(/\.(md|txt)$/, '') || 'Untitled',
        content,
        fileName: file.name,
        fileType: file.name.endsWith('.md') ? 'markdown' : 'txt',
        overallScore: result.overallScore,
        grade: result.grade,
        complexity: result.complexity,
        contextScore: result.scores.context,
        atomicityScore: result.scores.atomicity,
        boundaryScore: result.scores.boundary,
        verifiabilityScore: result.scores.verifiability,
        techScore: result.scores.tech,
        rawResponse: JSON.stringify(result),
        suggestions: JSON.stringify(result.suggestions),
        modelUsed: llmConfig.model,
      },
    })
    
    return NextResponse.json({
      id: evaluation.id,
      overallScore: evaluation.overallScore,
      grade: evaluation.grade,
      contextScore: evaluation.contextScore,
      atomicityScore: evaluation.atomicityScore,
      boundaryScore: evaluation.boundaryScore,
      verifiabilityScore: evaluation.verifiabilityScore,
      techScore: evaluation.techScore,
      suggestions: result.suggestions,
      createdAt: evaluation.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Evaluation error:', error)
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Evaluation failed' },
      { status: 500 }
    )
  }
}

export async function GET() {
  try {
    const evaluations = await prisma.evaluation.findMany({
      orderBy: { createdAt: 'desc' },
      take: 50,
    })
    
    return NextResponse.json(evaluations.map(e => ({
      id: e.id,
      title: e.title,
      fileName: e.fileName,
      overallScore: e.overallScore,
      grade: e.grade,
      complexity: e.complexity,
      createdAt: e.createdAt.toISOString(),
    })))
  } catch (error) {
    console.error('List evaluations error:', error)
    return NextResponse.json({ error: 'Failed to list evaluations' }, { status: 500 })
  }
}
