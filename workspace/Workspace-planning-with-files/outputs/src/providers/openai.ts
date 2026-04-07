import { EvaluationRequest, ILLMProvider, ProviderConfig, ProviderResponse } from './base';

export class OpenAIProvider implements ILLMProvider {
  readonly name = 'OpenAI';
  readonly models = ['gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo'];

  private config: ProviderConfig = {
    name: 'openai',
    model: 'gpt-4o',
    temperature: 0.3,
    maxTokens: 4000
  };

  async evaluate(request: EvaluationRequest): Promise<ProviderResponse> {
    const startTime = Date.now();
    const mergedConfig = { ...this.config, ...request.config };

    return {
      success: false,
      error: 'OpenAI provider not fully implemented - API key required',
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
