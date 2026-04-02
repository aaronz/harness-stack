import { useState } from 'react';

interface DimensionScores {
  context: number;
  atomicity: number;
  boundary: number;
  verifiability: number;
  technical: number;
}

interface ScoreBreakdownProps {
  dimensionScores: DimensionScores;
  complexityPenalty: number;
  totalScore: number;
  showDetails?: boolean;
}

export default function ScoreBreakdown({
  dimensionScores,
  complexityPenalty,
  totalScore,
  showDetails = false,
}: ScoreBreakdownProps) {
  const [expanded, setExpanded] = useState(showDetails);

  const maxScores = {
    context: 25,
    atomicity: 25,
    boundary: 20,
    verifiability: 15,
    technical: 15,
  };

  const dimensionWeights = {
    context: 0.25,
    atomicity: 0.25,
    boundary: 0.2,
    verifiability: 0.15,
    technical: 0.15,
  };

  const getScoreColor = (score: number, max: number) => {
    const ratio = score / max;
    if (ratio >= 0.8) return 'bg-green-500';
    if (ratio >= 0.5) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  const getDimensionLabel = (key: string) => {
    const labels: Record<string, string> = {
      context: 'Context Sufficiency',
      atomicity: 'Logic Atomicity',
      boundary: 'Boundary Definiteness',
      verifiability: 'Verifiability',
      technical: 'Technical Clarity',
    };
    return labels[key] || key;
  };

  const calculateWeightedScore = (dimension: keyof typeof dimensionScores, max: number) => {
    const ratio = dimensionScores[dimension] / max;
    return ratio * dimensionWeights[dimension] * 100;
  };

  return (
    <div className="mt-6 p-4 bg-white rounded-lg border border-gray-200">
      <button
        onClick={() => setExpanded(!expanded)}
        className="flex items-center justify-between w-full text-left"
      >
        <h3 className="text-lg font-semibold">Score Breakdown</h3>
        <span className="text-gray-500">{expanded ? '▲' : '▼'}</span>
      </button>

      {expanded && (
        <div className="mt-4 space-y-4">
          {Object.entries(dimensionScores).map(([dimension, score]) => {
            const max = maxScores[dimension as keyof typeof maxScores];
            const weighted = calculateWeightedScore(dimension as keyof typeof dimensionScores, max);
            return (
              <div key={dimension}>
                <div className="flex items-center justify-between mb-1">
                  <span className="text-sm font-medium">{getDimensionLabel(dimension)}</span>
                  <span className="text-sm text-gray-600">
                    {score}/{max}
                  </span>
                </div>
                <div className="h-3 bg-gray-200 rounded-full overflow-hidden">
                  <div
                    className={`h-full ${getScoreColor(score, max)} transition-all`}
                    style={{ width: `${(score / max) * 100}%` }}
                  />
                </div>
                <div className="text-xs text-gray-500 mt-1">
                  Weight: {dimensionWeights[dimension as keyof typeof dimensionWeights] * 100}% | 
                  Weighted: {weighted.toFixed(1)} pts
                </div>
              </div>
            );
          })}

          <div className="pt-4 border-t border-gray-200">
            <div className="flex items-center justify-between">
              <span className="font-medium">Complexity Penalty:</span>
              <span className="text-lg font-bold">{complexityPenalty}</span>
            </div>
            <div className="text-sm text-gray-500 mt-1">
              {complexityPenalty === 1.0 && 'Simple (CRUD) - No penalty'}
              {complexityPenalty === 0.9 && 'Medium (Business Logic) - 10% reduction'}
              {complexityPenalty === 0.7 && 'Complex (Algorithm/Architecture) - 30% reduction'}
            </div>
          </div>

          <div className="pt-4 border-t border-gray-200">
            <div className="flex items-center justify-between">
              <span className="font-medium text-lg">Final Score:</span>
              <span className="text-2xl font-bold text-blue-600">{totalScore}/100</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
