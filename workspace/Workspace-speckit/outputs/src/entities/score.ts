import { v4 as uuidv4 } from 'uuid';
import { runQuery, getOne } from '../db/schema';

export type ScoreLevel = 'S' | 'A' | 'B' | 'C';

export interface Score {
  id: string; requirementId: string; totalScore: number; level: ScoreLevel;
  contextScore: number; atomicityScore: number; boundaryScore: number;
  verifiabilityScore: number; technicalScore: number; complexityPenalty: number; calculatedAt: string;
}

export interface CreateScoreInput {
  requirementId: string; totalScore: number; level: ScoreLevel;
  contextScore: number; atomicityScore: number; boundaryScore: number;
  verifiabilityScore: number; technicalScore: number; complexityPenalty: number;
}

export class ScoreRepository {
  create(input: CreateScoreInput): Score {
    const id = uuidv4();
    const calculatedAt = new Date().toISOString();
    runQuery(
      `INSERT INTO scores (id, requirement_id, total_score, level, context_score, atomicity_score, boundary_score, verifiability_score, technical_score, complexity_penalty, calculated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
      [id, input.requirementId, input.totalScore, input.level, input.contextScore, input.atomicityScore, input.boundaryScore, input.verifiabilityScore, input.technicalScore, input.complexityPenalty, calculatedAt]
    );
    return { id, ...input, calculatedAt };
  }

  findByRequirementId(requirementId: string): Score | null {
    const row = getOne('SELECT * FROM scores WHERE requirement_id = ? ORDER BY calculated_at DESC LIMIT 1', [requirementId]);
    if (!row) return null;
    return { id: row.id, requirementId: row.requirement_id, totalScore: row.total_score, level: row.level as ScoreLevel, contextScore: row.context_score, atomicityScore: row.atomicity_score, boundaryScore: row.boundary_score, verifiabilityScore: row.verifiability_score, technicalScore: row.technical_score, complexityPenalty: row.complexity_penalty, calculatedAt: row.calculated_at };
  }
}

export const scoreRepository = new ScoreRepository();