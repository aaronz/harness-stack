import { EvaluationRequest, ILLMProvider, ProviderConfig, ProviderResponse } from './base';

export class AnthropicProvider implements ILLMProvider {
  readonly name = 'Anthropic';
  readonly models = ['claude-3-5-sonnet', 'claude-3-5-haiku', 'claude-3-opus'];

  private config: ProviderConfig = {
    name: 'anthropic',
    model: 'claude-3-5-sonnet',
    temperature: 0.3,
    maxTokens: 4000
  };

  async evaluate(request: EvaluationRequest): Promise<ProviderResponse> {
    const startTime = Date.now();
    const mergedConfig = { ...this.config, ...request.config };

    return {
      success: false,
      error: 'Anthropic provider not fully implemented - API key required',
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
