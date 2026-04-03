'use client';

import { useState } from 'react';

interface ScoreResult {
  totalScore: number;
  level: 'S' | 'A' | 'B' | 'C';
  dimensionScores: { context: number; atomicity: number; boundary: number; verifiability: number; technical: number };
  complexityPenalty: number;
}

interface ApiResponse {
  requirement: { id: string; text: string; submittedAt: string };
  score: ScoreResult;
  riskAnalysis?: { highlights: any[]; suggestions: any[] };
}

export default function Home() {
  const [text, setText] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<ApiResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!text.trim()) return;
    setLoading(true);
    setError(null);
    setResult(null);
    try {
      const res = await fetch('/api/requirements', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ text }) });
      const data = await res.json();
      if (!res.ok) throw new Error(data.error?.message || 'Failed');
      setResult(data);
    } catch (err) { setError(err instanceof Error ? err.message : 'Error'); }
    finally { setLoading(false); }
  };

  const getLevelColor = (l: string) => ({ S: 'bg-green-500', A: 'bg-blue-500', B: 'bg-yellow-500', C: 'bg-red-500' }[l] || 'bg-gray-500');
  const getLevelDesc = (l: string) => ({ S: 'AI can implement independently', A: 'AI can implement with human review', B: 'AI generates code requiring modification', C: 'AI implementation not recommended' }[l] || '');

  return (
    <main className="min-h-screen p-8 bg-gray-50">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold mb-2">AI-Readiness Evaluator</h1>
        <p className="text-gray-600 mb-8">Evaluate development requirements for AI Coding implementability</p>
        
        <form onSubmit={submit} className="mb-8">
          <textarea value={text} onChange={(e) => setText(e.target.value)} className="w-full h-48 p-4 border rounded-lg" placeholder="Enter requirement text..." />
          <button type="submit" disabled={loading || !text.trim()} className="mt-2 px-6 py-2 bg-blue-600 text-white rounded-lg disabled:opacity-50">
            {loading ? 'Evaluating...' : 'Evaluate'}
          </button>
        </form>

        {error && <div className="p-4 bg-red-50 text-red-700 rounded-lg mb-6">{error}</div>}

        {result && (
          <div className="bg-white rounded-lg shadow-md p-6">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-xl font-semibold">Result</h2>
              <div className="text-right">
                <div className={`text-4xl font-bold text-white rounded-lg px-4 py-2 ${getLevelColor(result.score.level)}`}>{result.score.level}</div>
                <div className="text-2xl font-semibold mt-1">{result.score.totalScore}/100</div>
              </div>
            </div>
            <p className="text-gray-700 mb-4">{getLevelDesc(result.score.level)}</p>
            <div className="grid grid-cols-5 gap-2">
              {Object.entries(result.score.dimensionScores).map(([k, v]) => (
                <div key={k} className="p-2 bg-gray-50 rounded text-center">
                  <div className="text-xs text-gray-500 capitalize">{k}</div>
                  <div className="font-semibold">{v}</div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </main>
  );
}