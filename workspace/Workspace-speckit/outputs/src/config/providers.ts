export type LLMProviderName = 'openai' | 'anthropic' | 'gemini' | 'ollama';

export interface LLMConfig {
  apiKey?: string;
  baseUrl?: string;
  model: string;
  temperature?: number;
  maxTokens?: number;
}

export interface LLMResponse {
  content: string;
  usage?: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
}

export interface ILLMProvider {
  name: LLMProviderName;
  config: LLMConfig;
  generate(prompt: string): Promise<LLMResponse>;
  isConfigured(): boolean;
}

export abstract class BaseLLMProvider implements ILLMProvider {
  abstract name: LLMProviderName;
  config: LLMConfig;

  constructor(config: LLMConfig) {
    this.config = config;
  }

  abstract generate(prompt: string): Promise<LLMResponse>;

  isConfigured(): boolean {
    return !!this.config.apiKey || (this.config.baseUrl?.includes('localhost') ?? false);
  }
}

export interface IProviderRegistry {
  providers: Map<LLMProviderName, ILLMProvider>;
  defaultProvider: LLMProviderName;
  getProvider(name?: LLMProviderName): ILLMProvider | null;
  setDefaultProvider(name: LLMProviderName): void;
  listProviders(): LLMProviderName[];
  registerProvider(provider: ILLMProvider): void;
}

let registry: IProviderRegistry | null = null;

export function getProviderRegistry(): IProviderRegistry {
  if (!registry) {
    registry = {
      providers: new Map(),
      defaultProvider: 'openai' as LLMProviderName,
      getProvider(name?: LLMProviderName) {
        const providerName = name || this.defaultProvider;
        return this.providers.get(providerName) || null;
      },
      setDefaultProvider(name: LLMProviderName) {
        this.defaultProvider = name;
      },
      listProviders() {
        return Array.from(this.providers.keys());
      },
      registerProvider(provider: ILLMProvider) {
        this.providers.set(provider.name, provider);
      },
    };
  }
  return registry;
}
