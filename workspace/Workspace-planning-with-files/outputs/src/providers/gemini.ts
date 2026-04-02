import { EvaluationRequest, ILLMProvider, ProviderConfig, ProviderResponse } from './base';

export class GeminiProvider implements ILLMProvider {
  readonly name = 'Gemini';
  readonly models = ['gemini-1.5-pro', 'gemini-1.5-flash', 'gemini-2.0-flash'];

  private config: ProviderConfig = {
    name: 'gemini',
    model: 'gemini-1.5-pro',
    temperature: 0.3,
    maxTokens: 4000
  };

  async evaluate(request: EvaluationRequest): Promise<ProviderResponse> {
    const startTime = Date.now();
    const mergedConfig = { ...this.config, ...request.config };

    return {
      success: false,
      error: 'Gemini provider not fully implemented - API key required',
      provider: this.name,
      model: mergedConfig.model,
      duration: Date.now() - startTime
    };
  }

  async healthCheck(): Promise<boolean> {
    return false;
  }

  configure(config: ProviderConfig): void {
    this.config = { ...this.config, ...config };
  }

  getConfig(): ProviderConfig {
    return { ...this.config };
  }
}
