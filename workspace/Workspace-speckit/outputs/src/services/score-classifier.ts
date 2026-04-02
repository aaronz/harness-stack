import { ScoreLevel } from '../entities/score';

export type { ScoreLevel };

const LEVEL_THRESHOLDS = {
  S: { min: 90, max: 100 },
  A: { min: 75, max: 89 },
  B: { min: 60, max: 74 },
  C: { min: 0, max: 59 },
};

export class ScoreClassifier {
  classify(score: number): ScoreLevel {
    if (score >= LEVEL_THRESHOLDS.S.min && score <= LEVEL_THRESHOLDS.S.max) return 'S';
    if (score >= LEVEL_THRESHOLDS.A.min && score <= LEVEL_THRESHOLDS.A.max) return 'A';
    if (score >= LEVEL_THRESHOLDS.B.min && score <= LEVEL_THRESHOLDS.B.max) return 'B';
    return 'C';
  }

  getLevelDescription(level: ScoreLevel): string {
    const descriptions: Record<ScoreLevel, string> = {
      S: 'AI can implement independently without human intervention',
      A: 'AI can implement but requires human review of critical boundaries',
      B: 'AI generates code requiring significant human modification',
      C: 'AI implementation not recommended - risk too high',
    };
    return descriptions[level];
  }

  getLevelColor(level: ScoreLevel): string {
    const colors: Record<ScoreLevel, string> = {
      S: '#22c55e',
      A: '#3b82f6',
      B: '#f59e0b',
      C: '#ef4444',
    };
    return colors[level];
  }

  getThresholds(): typeof LEVEL_THRESHOLDS {
    return { ...LEVEL_THRESHOLDS };
  }
}

export const scoreClassifier = new ScoreClassifier();
