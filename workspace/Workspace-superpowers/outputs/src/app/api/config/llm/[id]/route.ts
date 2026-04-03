import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { encryptApiKey, decryptApiKey } from '@/lib/encryption'

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const config = await prisma.lLMConfig.findUnique({
      where: { id: params.id },
    })
    
    if (!config) {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }
    
    return NextResponse.json({
      id: config.id,
      name: config.name,
      provider: config.provider,
      model: config.model,
      baseUrl: config.baseUrl,
      isDefault: config.isDefault,
      createdAt: config.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Get LLM config error:', error)
    return NextResponse.json({ error: 'Failed to get config' }, { status: 500 })
  }
}

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json()
    const { name, provider, model, apiKey, baseUrl, isDefault } = body
    
    const updateData: Record<string, unknown> = {}
    if (name) updateData.name = name
    if (provider) updateData.provider = provider
    if (model) updateData.model = model
    if (apiKey) updateData.apiKey = encryptApiKey(apiKey)
    if (baseUrl !== undefined) updateData.baseUrl = baseUrl
    
    if (isDefault) {
      await prisma.lLMConfig.updateMany({
        where: { isDefault: true },
        data: { isDefault: false },
      })
      updateData.isDefault = true
    }
    
    const config = await prisma.lLMConfig.update({
      where: { id: params.id },
      data: updateData,
    })
    
    return NextResponse.json({
      id: config.id,
      name: config.name,
      provider: config.provider,
      model: config.model,
      isDefault: config.isDefault,
      updatedAt: config.updatedAt.toISOString(),
    })
  } catch (error) {
    console.error('Update LLM config error:', error)
    return NextResponse.json({ error: 'Failed to update config' }, { status: 500 })
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    await prisma.lLMConfig.delete({
      where: { id: params.id },
    })
    
    return NextResponse.json({ success: true })
  } catch (error) {
    console.error('Delete LLM config error:', error)
    return NextResponse.json({ error: 'Failed to delete config' }, { status: 500 })
  }
}
