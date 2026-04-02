import { v4 as uuidv4 } from 'uuid';
import { runQuery, getOne, getAll } from '../db/schema';

export type RiskDimension = 'context' | 'atomicity' | 'boundary' | 'verifiability' | 'technical';

export interface RiskHighlight {
  id: string;
  scoreId: string;
  dimension: RiskDimension;
  textSpan: string;
  reason: string;
}

export interface CreateRiskHighlightInput {
  scoreId: string;
  dimension: RiskDimension;
  textSpan: string;
  reason: string;
}

export class RiskHighlightRepository {
  create(input: CreateRiskHighlightInput): RiskHighlight {
    const id = uuidv4();

    runQuery(
      'INSERT INTO risk_highlights (id, score_id, dimension, text_span, reason) VALUES (?, ?, ?, ?, ?)',
      [id, input.scoreId, input.dimension, input.textSpan, input.reason]
    );

    return {
      id,
      scoreId: input.scoreId,
      dimension: input.dimension,
      textSpan: input.textSpan,
      reason: input.reason,
    };
  }

  findByScoreId(scoreId: string): RiskHighlight[] {
    const rows = getAll('SELECT * FROM risk_highlights WHERE score_id = ?', [scoreId]);

    return rows.map((row) => ({
      id: row.id,
      scoreId: row.score_id,
      dimension: row.dimension as RiskDimension,
      textSpan: row.text_span,
      reason: row.reason,
    }));
  }

  deleteByScoreId(scoreId: string): void {
    runQuery('DELETE FROM risk_highlights WHERE score_id = ?', [scoreId]);
  }
}

export const riskHighlightRepository = new RiskHighlightRepository();
