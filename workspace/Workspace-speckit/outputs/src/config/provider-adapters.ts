import { BaseLLMProvider, LLMProviderName, LLMConfig, LLMResponse } from './providers';

export class OpenAIProvider extends BaseLLMProvider {
  name: LLMProviderName = 'openai';

  async generate(prompt: string): Promise<LLMResponse> {
    if (!this.config.apiKey) {
      throw new Error('OpenAI API key not configured');
    }

    const response = await fetch('https://api.openai.com/v1/chat/completions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${this.config.apiKey}`,
      },
      body: JSON.stringify({
        model: this.config.model || 'gpt-4o',
        messages: [{ role: 'user', content: prompt }],
        temperature: this.config.temperature || 0.7,
        max_tokens: this.config.maxTokens || 2000,
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`OpenAI API error: ${error}`);
    }

    const data = await response.json();
    return {
      content: data.choices[0]?.message?.content || '',
      usage: data.usage,
    };
  }
}

export class AnthropicProvider extends BaseLLMProvider {
  name: LLMProviderName = 'anthropic';

  async generate(prompt: string): Promise<LLMResponse> {
    if (!this.config.apiKey) {
      throw new Error('Anthropic API key not configured');
    }

    const response = await fetch('https://api.anthropic.com/v1/messages', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': this.config.apiKey,
        'anthropic-version': '2023-06-01',
      },
      body: JSON.stringify({
        model: this.config.model || 'claude-3-5-sonnet-20241022',
        messages: [{ role: 'user', content: prompt }],
        temperature: this.config.temperature || 0.7,
        max_tokens: this.config.maxTokens || 2000,
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Anthropic API error: ${error}`);
    }

    const data = await response.json();
    return {
      content: data.content[0]?.text || '',
      usage: {
        promptTokens: data.usage.input_tokens,
        completionTokens: data.usage.output_tokens,
        totalTokens: data.usage.input_tokens + data.usage.output_tokens,
      },
    };
  }
}

export class GeminiProvider extends BaseLLMProvider {
  name: LLMProviderName = 'gemini';

  async generate(prompt: string): Promise<LLMResponse> {
    if (!this.config.apiKey) {
      throw new Error('Gemini API key not configured');
    }

    const model = this.config.model || 'gemini-1.5-flash';
    const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models/${model}:generateContent?key=${this.config.apiKey}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        contents: [{ parts: [{ text: prompt }] }],
        generationConfig: {
          temperature: this.config.temperature || 0.7,
          maxOutputTokens: this.config.maxTokens || 2000,
        },
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Gemini API error: ${error}`);
    }

    const data = await response.json();
    return {
      content: data.candidates[0]?.content?.parts[0]?.text || '',
    };
  }
}

export class OllamaProvider extends BaseLLMProvider {
  name: LLMProviderName = 'ollama';

  async generate(prompt: string): Promise<LLMResponse> {
    const baseUrl = this.config.baseUrl || 'http://localhost:11434';
    const model = this.config.model || 'llama2';

    const response = await fetch(`${baseUrl}/api/generate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        model,
        prompt,
        temperature: this.config.temperature || 0.7,
        stream: false,
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Ollama API error: ${error}`);
    }

    const data = await response.json();
    return {
      content: data.response || '',
    };
  }

  isConfigured(): boolean {
    return !!this.config.baseUrl || !!process.env.OLLAMA_BASE_URL;
  }
}

export const providerClasses = {
  openai: OpenAIProvider,
  anthropic: AnthropicProvider,
  gemini: GeminiProvider,
  ollama: OllamaProvider,
};
