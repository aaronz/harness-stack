import OpenAI from 'openai';
import Anthropic from '@anthropic-ai/sdk';
import axios from 'axios';
import { prisma } from '../index.js';

interface LLMConfig {
  provider: string;
  model: string;
  apiKey: string;
  endpoint?: string;
  maxTokens?: number;
  temperature?: number;
}

async function getActiveProvider(): Promise<LLMConfig | null> {
  const provider = await prisma.lLMProvider.findFirst({
    where: { enabled: true },
    orderBy: { priority: 'asc' }
  });

  if (!provider) {
    return {
      provider: 'OPENAI',
      model: 'gpt-4o',
      apiKey: process.env.OPENAI_API_KEY || '',
      temperature: 0.7,
      maxTokens: 4096
    };
  }

  return {
    provider: provider.provider,
    model: provider.model,
    apiKey: provider.apiKey,
    endpoint: provider.endpoint || undefined,
    maxTokens: provider.maxTokens,
    temperature: provider.temperature
  };
}

export async function callLLM(prompt: string, systemPrompt?: string): Promise<string> {
  const config = await getActiveProvider();

  if (!config.apiKey) {
    throw new Error('No LLM API key configured');
  }

  switch (config.provider) {
    case 'OPENAI':
      return callOpenAI(prompt, config, systemPrompt);
    case 'ANTHROPIC':
      return callAnthropic(prompt, config, systemPrompt);
    case 'GOOGLE':
      return callGoogle(prompt, config, systemPrompt);
    case 'LOCAL':
      return callLocal(prompt, config, systemPrompt);
    default:
      return callOpenAI(prompt, config, systemPrompt);
  }
}

async function callOpenAI(prompt: string, config: LLMConfig, systemPrompt?: string): Promise<string> {
  const client = new OpenAI({ apiKey: config.apiKey });

  const response = await client.chat.completions.create({
    model: config.model || 'gpt-4o',
    messages: [
      { role: 'system', content: systemPrompt || 'You are an expert at evaluating software requirements for AI Coding readiness.' },
      { role: 'user', content: prompt }
    ],
    temperature: config.temperature || 0.7,
    max_tokens: config.maxTokens || 4096
  });

  return response.choices[0]?.message?.content || '';
}

async function callAnthropic(prompt: string, config: LLMConfig, systemPrompt?: string): Promise<string> {
  const client = new Anthropic({ apiKey: config.apiKey });

  const response = await client.messages.create({
    model: config.model || 'claude-3-5-sonnet-20241022',
    max_tokens: config.maxTokens || 4096,
    system: systemPrompt || 'You are an expert at evaluating software requirements for AI Coding readiness.',
    messages: [{ role: 'user', content: prompt }]
  });

  return response.content[0]?.type === 'text' ? response.content[0].text : '';
}

async function callGoogle(prompt: string, config: LLMConfig, systemPrompt?: string): Promise<string> {
  const endpoint = config.endpoint || `https://generativelanguage.googleapis.com/v1beta/models/${config.model || 'gemini-pro'}:generateContent`;

  const response = await axios.post(
    endpoint,
    { contents: [{ parts: [{ text: prompt }] }] },
    {
      headers: { Authorization: `Bearer ${config.apiKey}` },
      params: { key: config.apiKey }
    }
  );

  return response.data.candidates?.[0]?.content?.parts?.[0]?.text || '';
}

async function callLocal(prompt: string, config: LLMConfig, systemPrompt?: string): Promise<string> {
  const endpoint = config.endpoint || 'http://localhost:11434/api/generate';

  const response = await axios.post(endpoint, {
    model: config.model || 'llama2',
    prompt: `${systemPrompt || ''}\n\n${prompt}`,
    stream: false
  });

  return response.data.response || '';
}