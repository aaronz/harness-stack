import { v4 as uuidv4 } from 'uuid';
import { runQuery, getOne, getAll } from '../db/schema';

export type RiskDimension = 'context' | 'atomicity' | 'boundary' | 'verifiability' | 'technical';

export interface RiskHighlight {
  id: string; scoreId: string; dimension: RiskDimension; textSpan: string; reason: string;
}

export class RiskHighlightRepository {
  create(input: { scoreId: string; dimension: RiskDimension; textSpan: string; reason: string }): RiskHighlight {
    const id = uuidv4();
    runQuery('INSERT INTO risk_highlights (id, score_id, dimension, text_span, reason) VALUES (?, ?, ?, ?, ?)', [id, input.scoreId, input.dimension, input.textSpan, input.reason]);
    return { id, ...input };
  }
  findByScoreId(scoreId: string): RiskHighlight[] {
    return getAll('SELECT * FROM risk_highlights WHERE score_id = ?', [scoreId])
      .map((row) => ({ id: row.id, scoreId: row.score_id, dimension: row.dimension as RiskDimension, textSpan: row.text_span, reason: row.reason }));
  }
  deleteByScoreId(scoreId: string): void { runQuery('DELETE FROM risk_highlights WHERE score_id = ?', [scoreId]); }
}

export const riskHighlightRepository = new RiskHighlightRepository();