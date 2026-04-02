import { EvaluationResult } from '../types/evaluation';

/**
 * LLM Provider配置
 */
export interface ProviderConfig {
  name: string;
  apiKey?: string;
  baseUrl?: string;
  model: string;
  temperature?: number;
  maxTokens?: number;
}

/**
 * 评估请求
 */
export interface EvaluationRequest {
  requirementText: string;
  config?: Partial<ProviderConfig>;
}

/**
 * Provider响应
 */
export interface ProviderResponse {
  success: boolean;
  data?: EvaluationResult;
  error?: string;
  provider: string;
  model: string;
  duration: number; // 毫秒
}

/**
 * LLM Provider基类接口
 */
export interface ILLMProvider {
  /**
   * Provider名称
   */
  readonly name: string;

  /**
   * 支持的模型列表
   */
  readonly models: string[];

  /**
   * 执行评估
   */
  evaluate(request: EvaluationRequest): Promise<ProviderResponse>;

  /**
   * 健康检查
   */
  healthCheck(): Promise<boolean>;

  /**
   * 配置Provider
   */
  configure(config: ProviderConfig): void;

  /**
   * 获取当前配置
   */
  getConfig(): ProviderConfig;
}
