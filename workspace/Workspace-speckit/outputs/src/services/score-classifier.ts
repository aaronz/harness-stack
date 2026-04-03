import { ScoreLevel } from '../entities/score';

const LEVELS = { S: { min: 90, max: 100 }, A: { min: 75, max: 89 }, B: { min: 60, max: 74 }, C: { min: 0, max: 59 } };

export class ScoreClassifier {
  classify(score: number): ScoreLevel {
    if (score >= LEVELS.S.min) return 'S';
    if (score >= LEVELS.A.min) return 'A';
    if (score >= LEVELS.B.min) return 'B';
    return 'C';
  }
  getDescription(level: ScoreLevel): string {
    return { S: 'AI can implement independently', A: 'AI can implement with human review', B: 'AI generates code requiring modification', C: 'AI implementation not recommended' }[level];
  }
  getColor(level: ScoreLevel): string {
    return { S: '#22c55e', A: '#3b82f6', B: '#f59e0b', C: '#ef4444' }[level];
  }
}

export const scoreClassifier = new ScoreClassifier();