import { v4 as uuidv4 } from 'uuid';
import { runQuery, getOne, getAll } from '../db/schema';

export type SuggestionCategory = 'context' | 'boundary' | 'atomicity' | 'verifiability';
export type SuggestionPriority = 'high' | 'medium' | 'low';

export interface OptimizationSuggestion {
  id: string;
  scoreId: string;
  category: SuggestionCategory;
  description: string;
  priority: SuggestionPriority;
}

export interface CreateOptimizationSuggestionInput {
  scoreId: string;
  category: SuggestionCategory;
  description: string;
  priority: SuggestionPriority;
}

export class OptimizationSuggestionRepository {
  create(input: CreateOptimizationSuggestionInput): OptimizationSuggestion {
    const id = uuidv4();

    runQuery(
      'INSERT INTO optimization_suggestions (id, score_id, category, description, priority) VALUES (?, ?, ?, ?, ?)',
      [id, input.scoreId, input.category, input.description, input.priority]
    );

    return {
      id,
      scoreId: input.scoreId,
      category: input.category,
      description: input.description,
      priority: input.priority,
    };
  }

  findByScoreId(scoreId: string): OptimizationSuggestion[] {
    const rows = getAll('SELECT * FROM optimization_suggestions WHERE score_id = ?', [scoreId]);

    return rows.map((row) => ({
      id: row.id,
      scoreId: row.score_id,
      category: row.category as SuggestionCategory,
      description: row.description,
      priority: row.priority as SuggestionPriority,
    }));
  }

  createBatch(inputs: CreateOptimizationSuggestionInput[]): OptimizationSuggestion[] {
    return inputs.map((input) => this.create(input));
  }

  deleteByScoreId(scoreId: string): void {
    runQuery('DELETE FROM optimization_suggestions WHERE score_id = ?', [scoreId]);
  }
}

export const optimizationSuggestionRepository = new OptimizationSuggestionRepository();
