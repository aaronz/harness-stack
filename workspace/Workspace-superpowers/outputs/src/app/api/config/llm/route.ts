import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { encryptApiKey } from '@/lib/encryption'

export async function GET() {
  try {
    const configs = await prisma.lLMConfig.findMany({
      orderBy: { createdAt: 'desc' },
    })
    
    return NextResponse.json(configs.map(c => ({
      id: c.id,
      name: c.name,
      provider: c.provider,
      model: c.model,
      baseUrl: c.baseUrl,
      isDefault: c.isDefault,
      createdAt: c.createdAt.toISOString(),
    })))
  } catch (error) {
    console.error('List LLM configs error:', error)
    return NextResponse.json({ error: 'Failed to list configs' }, { status: 500 })
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json()
    const { name, provider, model, apiKey, baseUrl, isDefault } = body
    
    if (!name || !provider || !model || !apiKey) {
      return NextResponse.json({ error: 'Missing required fields' }, { status: 400 })
    }
    
    if (isDefault) {
      await prisma.lLMConfig.updateMany({
        where: { isDefault: true },
        data: { isDefault: false },
      })
    }
    
    const config = await prisma.lLMConfig.create({
      data: {
        name,
        provider,
        model,
        apiKey: encryptApiKey(apiKey),
        baseUrl: baseUrl || null,
        isDefault: isDefault || false,
      },
    })
    
    return NextResponse.json({
      id: config.id,
      name: config.name,
      provider: config.provider,
      model: config.model,
      isDefault: config.isDefault,
      createdAt: config.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Create LLM config error:', error)
    return NextResponse.json({ error: 'Failed to create config' }, { status: 500 })
  }
}
