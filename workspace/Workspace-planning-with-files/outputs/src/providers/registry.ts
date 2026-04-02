import { ILLMProvider, ProviderConfig } from './base';
import { OpenAIProvider } from './openai';
import { AnthropicProvider } from './anthropic';
import { GeminiProvider } from './gemini';
import { OllamaProvider } from './ollama';

export type ProviderType = 'openai' | 'anthropic' | 'gemini' | 'ollama';

export class ProviderRegistry {
  private providers: Map<ProviderType, ILLMProvider> = new Map();
  private activeProvider: ProviderType = 'openai';

  constructor() {
    this.registerDefaultProviders();
  }

  private registerDefaultProviders(): void {
    this.providers.set('openai', new OpenAIProvider());
    this.providers.set('anthropic', new AnthropicProvider());
    this.providers.set('gemini', new GeminiProvider());
    this.providers.set('ollama', new OllamaProvider());
  }

  getProvider(type?: ProviderType): ILLMProvider {
    const providerType = type ?? this.activeProvider;
    const provider = this.providers.get(providerType);
    if (!provider) {
      throw new Error(`Provider ${providerType} not registered`);
    }
    return provider;
  }

  setActiveProvider(type: ProviderType): void {
    if (!this.providers.has(type)) {
      throw new Error(`Provider ${type} not available`);
    }
    this.activeProvider = type;
  }

  getActiveProvider(): ILLMProvider {
    return this.getProvider(this.activeProvider);
  }

  getAvailableProviders(): Array<{ type: ProviderType; name: string; models: string[] }> {
    return Array.from(this.providers.entries()).map(([type, provider]) => ({
      type,
      name: provider.name,
      models: provider.models
    }));
  }

  configureProvider(type: ProviderType, config: ProviderConfig): void {
    const provider = this.getProvider(type);
    provider.configure(config);
  }
}

export const providerRegistry = new ProviderRegistry();
