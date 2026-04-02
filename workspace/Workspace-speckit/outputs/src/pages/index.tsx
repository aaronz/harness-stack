import { useState } from 'react';
import RiskHeatmap from '../components/RiskHeatmap';
import OptimizationSuggestions from '../components/OptimizationSuggestions';
import ScoreBreakdown from '../components/ScoreBreakdown';
import ProviderSelector from '../components/ProviderSelector';

interface ScoreResult {
  totalScore: number;
  level: 'S' | 'A' | 'B' | 'C';
  dimensionScores: {
    context: number;
    atomicity: number;
    boundary: number;
    verifiability: number;
    technical: number;
  };
  complexityPenalty: number;
}

interface RequirementData {
  id: string;
  text: string;
  submittedAt: string;
}

interface RiskHighlight {
  dimension: string;
  textSpan: string;
  reason: string;
}

interface Suggestion {
  category: string;
  description: string;
  priority: string;
}

interface ApiResponse {
  requirement: RequirementData;
  score: ScoreResult;
  riskAnalysis?: {
    highlights: RiskHighlight[];
    suggestions: Suggestion[];
  };
}

export default function Home() {
  const [requirementText, setRequirementText] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<ApiResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!requirementText.trim()) return;

    setIsLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/requirements', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: requirementText }),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.error?.message || 'Failed to evaluate requirement');
      }

      setResult(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred');
    } finally {
      setIsLoading(false);
    }
  };

  const getLevelColor = (level: string) => {
    const colors: Record<string, string> = {
      S: 'bg-green-500',
      A: 'bg-blue-500',
      B: 'bg-yellow-500',
      C: 'bg-red-500',
    };
    return colors[level] || 'bg-gray-500';
  };

  const getLevelDescription = (level: string) => {
    const descriptions: Record<string, string> = {
      S: 'AI can implement independently without human intervention',
      A: 'AI can implement but requires human review of critical boundaries',
      B: 'AI generates code requiring significant human modification',
      C: 'AI implementation not recommended - risk too high',
    };
    return descriptions[level] || '';
  };

  return (
    <main className="min-h-screen p-8 bg-gray-50">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold mb-2">AI-Readiness Evaluator</h1>
        <p className="text-gray-600 mb-8">
          Evaluate development requirements for AI Coding implementability
        </p>

        <form onSubmit={handleSubmit} className="mb-8">
          <div className="mb-4">
            <label htmlFor="requirement" className="block text-sm font-medium mb-2">
              Enter your requirement text:
            </label>
            <textarea
              id="requirement"
              value={requirementText}
              onChange={(e) => setRequirementText(e.target.value)}
              className="w-full h-48 p-4 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Paste your requirement here...&#10;&#10;Example: 实现用户注册功能。输入：用户名(6-20位正则)，密码(8-32位)。输出：用户对象或错误码。使用Spring Boot 2.7，密码BCrypt加密。"
            />
          </div>
          <button
            type="submit"
            disabled={isLoading || !requirementText.trim()}
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? 'Evaluating...' : 'Evaluate'}
          </button>
        </form>

        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
            {error}
          </div>
        )}

        {result && (
          <div className="bg-white rounded-lg shadow-md p-6">
            <div className="flex items-center justify-between mb-6">
              <div>
                <h2 className="text-xl font-semibold mb-1">Evaluation Result</h2>
                <p className="text-sm text-gray-500">
                  Submitted at {new Date(result.requirement.submittedAt).toLocaleString()}
                </p>
              </div>
              <div className="text-right">
                <div className={`text-4xl font-bold text-white rounded-lg px-4 py-2 ${getLevelColor(result.score.level)}`}>
                  {result.score.level}
                </div>
                <div className="text-2xl font-semibold mt-1">{result.score.totalScore}/100</div>
              </div>
            </div>

            <p className="text-gray-700 mb-6">{getLevelDescription(result.score.level)}</p>

            <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
              {Object.entries(result.score.dimensionScores).map(([dimension, score]) => (
                <div key={dimension} className="p-3 bg-gray-50 rounded-lg text-center">
                  <div className="text-sm text-gray-500 capitalize">{dimension}</div>
                  <div className="text-xl font-semibold">{score}</div>
                </div>
              ))}
            </div>

            <div className="mt-4 p-3 bg-gray-50 rounded-lg text-center">
              <span className="text-sm text-gray-500">Complexity Penalty: </span>
              <span className="font-semibold">{result.score.complexityPenalty}</span>
            </div>

            {result.riskAnalysis && (
              <>
                <RiskHeatmap highlights={result.riskAnalysis.highlights} />
                <OptimizationSuggestions suggestions={result.riskAnalysis.suggestions} />
              </>
            )}

            <ScoreBreakdown
              dimensionScores={result.score.dimensionScores}
              complexityPenalty={result.score.complexityPenalty}
              totalScore={result.score.totalScore}
              showDetails={true}
            />
          </div>
        )}
      </div>
    </main>
  );
}
