import { Request, Response, NextFunction } from 'express';
import { LLMProviderName } from '../config/providers';

declare global {
  namespace Express {
    interface Request {
      providerOverride?: LLMProviderName;
    }
  }
}

const ALLOWED_PROVIDERS: LLMProviderName[] = ['openai', 'anthropic', 'gemini', 'ollama'];

export function providerSelector(req: Request, res: Response, next: NextFunction): void {
  const providerHeader = req.headers['x-llm-provider'] as string;
  const providerBody = req.body?.provider as string;

  const providerName = providerHeader || providerBody;

  if (providerName && ALLOWED_PROVIDERS.includes(providerName as LLMProviderName)) {
    req.providerOverride = providerName as LLMProviderName;
  }

  next();
}

export function getProviderFromRequest(req: Request, defaultProvider: LLMProviderName): LLMProviderName {
  return req.providerOverride || defaultProvider;
}

export { ALLOWED_PROVIDERS };
