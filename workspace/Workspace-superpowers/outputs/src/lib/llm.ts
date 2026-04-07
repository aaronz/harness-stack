import OpenAI from 'openai'
import Anthropic from '@anthropic-ai/sdk'
import { GoogleGenerativeAI } from '@google/generative-ai'
import { decryptApiKey } from './encryption'

export type LLMProvider = 'openai' | 'anthropic' | 'google' | 'ollama'

export interface LLMConfig {
  provider: LLMProvider
  model: string
  apiKey: string
  baseUrl?: string
}

export interface EvaluationResult {
  scores: {
    context: number
    atomicity: number
    boundary: number
    verifiability: number
    tech: number
  }
  overall: number
  grade: 'S' | 'A' | 'B' | 'C'
  complexity: 'simple' | 'medium' | 'complex'
  risks: string[]
  suggestions: Array<{ type: string; content: string }>
}

const EVALUATION_PROMPT = `
你是一个 AI 需求评估专家。请分析以下需求，评估其 AI Coding 可实现性。

## 五大评估维度

1. 上下文完备性（25%）：业务背景、术语定义、用户旅程
2. 逻辑原子性（25%）：需求颗粒度、功能内聚、依赖解耦
3. 边界明确性（20%）：异常流程、输入校验、状态枚举
4. 可验证性（15%）：验收标准量化、测试场景完备性
5. 技术约束清晰度（15%）：架构约束、技术栈限定

## 需求内容
---
{CONTENT}
---

## 输出要求

请返回 JSON 格式（不要有其他内容）:
{
  "scores": {
    "context": 0-100,
    "atomicity": 0-100,
    "boundary": 0-100,
    "verifiability": 0-100,
    "tech": 0-100
  },
  "overall": 0-100,
  "grade": "S|A|B|C",
  "complexity": "simple|medium|complex",
  "risks": ["风险点1", "风险点2"],
  "suggestions": [
    {"type": "context|boundary|atomicity|verifiability", "content": "具体建议"}
  ]
}
`

export async function callLLM(config: LLMConfig, content: string): Promise<EvaluationResult> {
  const prompt = EVALUATION_PROMPT.replace('{CONTENT}', content)
  
  switch (config.provider) {
    case 'openai':
      return withRetry(() => callOpenAI(config, prompt))
    case 'anthropic':
      return withRetry(() => callAnthropic(config, prompt))
    case 'google':
      return withRetry(() => callGoogle(config, prompt))
    case 'ollama':
      return withRetry(() => callOllama(config, prompt))
    default:
      throw new Error(`Unknown provider: ${config.provider}`)
  }
}

async function withRetry<T>(fn: () => Promise<T>, maxRetries: number = 3, baseDelayMs: number = 1000): Promise<T> {
  let lastError: Error | null = null
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await fn()
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error))
      if (lastError.message.includes('Invalid model ID') || lastError.message.includes('api key')) throw lastError
      if (attempt < maxRetries) {
        const delay = baseDelayMs * Math.pow(2, attempt) + Math.random() * 1000
        await new Promise(resolve => setTimeout(resolve, delay))
      }
    }
  }
  throw lastError || new Error('Max retries exceeded')
}

async function callOpenAI(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const client = new OpenAI({
    apiKey: config.apiKey,
    baseURL: config.baseUrl,
  })
  
  const response = await client.chat.completions.create({
    model: config.model,
    messages: [{ role: 'user', content: prompt }],
    temperature: 0.3,
  })
  
  const content = response.choices[0]?.message?.content || ''
  return parseResponse(content)
}

async function callAnthropic(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const client = new Anthropic({
    apiKey: config.apiKey,
    baseURL: config.baseUrl,
  })
  
  const response = await client.messages.create({
    model: config.model,
    max_tokens: 4096,
    messages: [{ role: 'user', content: prompt }],
  })
  
  const content = response.content[0]?.type === 'text' ? response.content[0].text : ''
  return parseResponse(content)
}

async function callGoogle(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const client = new GoogleGenerativeAI(config.apiKey)
  const model = client.getGenerativeModel({ model: config.model })
  
  const result = await model.generateContent(prompt)
  const content = result.response.text()
  return parseResponse(content)
}

async function callOllama(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const baseUrl = config.baseUrl || 'http://localhost:11434'
  
  const response = await fetch(`${baseUrl}/api/generate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      model: config.model,
      prompt,
      stream: false,
    }),
  })
  
  const data = await response.json() as { response: string }
  return parseResponse(data.response)
}

export function parseResponse(content: string): EvaluationResult {
  const jsonMatch = content.match(/\{[\s\S]*\}/)
  if (!jsonMatch) {
    throw new Error('Failed to parse LLM response')
  }
  
  const parsed = JSON.parse(jsonMatch[0])
  
  return {
    scores: {
      context: Math.min(100, Math.max(0, parsed.scores?.context || 0)),
      atomicity: Math.min(100, Math.max(0, parsed.scores?.atomicity || 0)),
      boundary: Math.min(100, Math.max(0, parsed.scores?.boundary || 0)),
      verifiability: Math.min(100, Math.max(0, parsed.scores?.verifiability || 0)),
      tech: Math.min(100, Math.max(0, parsed.scores?.tech || 0)),
    },
    overall: Math.min(100, Math.max(0, parsed.overall || 0)),
    grade: parsed.grade || 'C',
    complexity: parsed.complexity || 'medium',
    risks: Array.isArray(parsed.risks) ? parsed.risks : [],
    suggestions: Array.isArray(parsed.suggestions) ? parsed.suggestions : [],
  }
}
