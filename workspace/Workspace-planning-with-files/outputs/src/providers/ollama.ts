import { EvaluationRequest, ILLMProvider, ProviderConfig, ProviderResponse } from './base';

export class OllamaProvider implements ILLMProvider {
  readonly name = 'Ollama';
  readonly models = ['llama3.1:8b', 'qwen2.5:7b', 'mistral:7b'];

  private config: ProviderConfig = {
    name: 'ollama',
    baseUrl: 'http://localhost:11434',
    model: 'llama3.1:8b',
    temperature: 0.3,
    maxTokens: 4000
  };

  async evaluate(request: EvaluationRequest): Promise<ProviderResponse> {
    const startTime = Date.now();
    const mergedConfig = { ...this.config, ...request.config };

    return {
      success: false,
      error: 'Ollama provider not fully implemented - local Ollama service required',
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
