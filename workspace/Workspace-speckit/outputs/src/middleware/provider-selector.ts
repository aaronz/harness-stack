import { Request, Response, NextFunction } from 'express';

export type LLMProviderName = 'openai' | 'anthropic' | 'gemini' | 'ollama';

declare global { namespace Express { interface Request { providerOverride?: LLMProviderName; } } }
const ALLOWED: LLMProviderName[] = ['openai', 'anthropic', 'gemini', 'ollama'];

export function providerSelector(req: Request, res: Response, next: NextFunction): void {
  const provider = (req.headers['x-llm-provider'] as string) || req.body?.provider;
  if (provider && ALLOWED.includes(provider as LLMProviderName)) req.providerOverride = provider as LLMProviderName;
  next();
}

export function getProvider(req: Request, defaultProvider: LLMProviderName): LLMProviderName {
  return req.providerOverride || defaultProvider;
}