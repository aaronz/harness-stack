import { LLMConfig, DimensionScore, Dimension } from '../types';

interface LLMClient {
  generate(prompt: string): Promise<string>;
}

export class LLMGateway {
  private config: LLMConfig;
  private client: LLMClient;

  constructor(config: LLMConfig) {
    this.config = config;
    this.client = this.createClient();
  }

  private createClient(): LLMClient {
    return {
      generate: async (_prompt: string): Promise<string> => {
        return JSON.stringify({
          context: { score: 80, analysis: 'Mock response', riskPoints: [] },
          atomicity: { score: 80, analysis: 'Mock response', riskPoints: [] },
          boundary: { score: 80, analysis: 'Mock response', riskPoints: [] },
          verifiability: { score: 80, analysis: 'Mock response', riskPoints: [] },
          technical: { score: 80, analysis: 'Mock response', riskPoints: [] }
        });
      }
    };
  }

  async evaluateDimensions(requirement: string): Promise<Record<Dimension, DimensionScore>> {
    const prompt = this.buildPrompt(requirement);
    const response = await this.client.generate(prompt);
    return this.parseResponse(response);
  }

  private buildPrompt(requirement: string): string {
    return `请分析以下需求的AI可落地性，从5个维度评估（上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束）：

需求内容：${requirement}

请按以下JSON格式返回评估结果：
{
  "context": {"score": 0-100, "analysis": "分析说明", "riskPoints": ["风险点"]},
  "atomicity": {"score": 0-100, "analysis": "分析说明", "riskPoints": ["风险点"]},
  "boundary": {"score": 0-100, "analysis": "分析说明", "riskPoints": ["风险点"]},
  "verifiability": {"score": 0-100, "analysis": "分析说明", "riskPoints": ["风险点"]},
  "technical": {"score": 0-100, "analysis": "分析说明", "riskPoints": ["风险点"]}
}`;
  }

  private parseResponse(response: string): Record<Dimension, DimensionScore> {
    try {
      const parsed = JSON.parse(response);
      const dimensions = ['context', 'atomicity', 'boundary', 'verifiability', 'technical'] as Dimension[];
      
      const result: Record<Dimension, DimensionScore> = {} as Record<Dimension, DimensionScore>;
      
      for (const dim of dimensions) {
        const data = parsed[dim];
        if (!data || typeof data.score !== 'number') {
          throw new Error(`Invalid dimension data for ${dim}`);
        }
        
        if (!this.validateScore(data.score)) {
          throw new Error(`Invalid score ${data.score} for ${dim}`);
        }
        
        result[dim] = {
          dimension: dim,
          score: data.score,
          analysis: data.analysis || '',
          riskPoints: data.riskPoints || []
        };
      }
      
      return result;
    } catch (e) {
      throw new Error(`Failed to parse LLM response: ${e instanceof Error ? e.message : 'Unknown error'}`);
    }
  }

  private validateScore(score: number): boolean {
    return Number.isInteger(score) && score >= 0 && score <= 100;
  }

  getConfig(): LLMConfig {
    return this.config;
  }
}