'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import DashboardLayout from '@/components/Layout';
import { useAuthStore } from '@/store/auth';
import { evaluationApi } from '@/lib/api';
import { useMutation, useQuery } from '@tanstack/react-query';
import { Radar, AlertTriangle, CheckCircle, FileText, Loader2, Zap } from 'lucide-react';

export default function EvaluatePage() {
  const router = useRouter();
  const { isAuthenticated } = useAuthStore();

  const [requirementText, setRequirementText] = useState('');
  const [format, setFormat] = useState<'text' | 'markdown' | 'json'>('text');
  const [complexity, setComplexity] = useState<'SIMPLE' | 'MEDIUM' | 'COMPLEX'>('MEDIUM');
  const [evaluationId, setEvaluationId] = useState<string | null>(null);

  const createMutation = useMutation({
    mutationFn: ({ text, fmt, comp }: { text: string; fmt: string; comp: string }) =>
      evaluationApi.create(text, fmt, comp),
    onSuccess: (data) => {
      setEvaluationId(data.data.id);
    },
  });

  const resultQuery = useQuery({
    queryKey: ['evaluation', evaluationId],
    queryFn: () => evaluationApi.get(evaluationId!).then(res => res.data),
    enabled: !!evaluationId && createMutation.isSuccess,
    refetchInterval: (query) => {
      const data = query.state.data;
      if (!data || data.status === 'COMPLETED' || data.status === 'FAILED') return false;
      return 2000;
    },
  });

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  if (!isAuthenticated) return null;

  const evaluation = resultQuery.data;
  const isProcessing = evaluation?.status === 'PROCESSING' || evaluation?.status === 'PENDING';

  return (
    <DashboardLayout>
      <div className="max-w-7xl">
        <h1 className="text-2xl font-bold mb-6">Evaluate Requirement</h1>

        <div className="grid grid-cols-2 gap-6">
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h2 className="font-semibold mb-4">Input Requirement</h2>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Requirement Text</label>
              <textarea
                value={requirementText}
                onChange={(e) => setRequirementText(e.target.value)}
                placeholder="Paste your requirement here..."
                className="w-full h-64 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary resize-none font-mono text-sm"
              />
              <div className="text-sm text-gray-500 mt-1">{requirementText.length} characters</div>
            </div>

            <div className="grid grid-cols-2 gap-4 mb-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Format</label>
                <select
                  value={format}
                  onChange={(e) => setFormat(e.target.value as any)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                >
                  <option value="text">Plain Text</option>
                  <option value="markdown">Markdown</option>
                  <option value="json">JSON</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Complexity</label>
                <select
                  value={complexity}
                  onChange={(e) => setComplexity(e.target.value as any)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                >
                  <option value="SIMPLE">Simple (CRUD)</option>
                  <option value="MEDIUM">Medium (Business Logic)</option>
                  <option value="COMPLEX">Complex (Algorithm/Architecture)</option>
                </select>
              </div>
            </div>

            <button
              onClick={() => createMutation.mutate({ text: requirementText, fmt: format, comp: complexity })}
              disabled={!requirementText.trim() || createMutation.isPending}
              className="w-full py-3 px-4 bg-primary text-white rounded-lg hover:bg-primary-dark disabled:opacity-50 transition-colors flex items-center justify-center gap-2"
            >
              {createMutation.isPending ? (
                <><Loader2 className="animate-spin" size={20} /> Processing...</>
              ) : (
                <><Zap size={20} /> Evaluate Requirement</>
              )}
            </button>
          </div>

          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h2 className="font-semibold mb-4">Evaluation Result</h2>

            {!evaluation && !evaluationId && (
              <div className="h-96 flex items-center justify-center text-gray-400">
                <div className="text-center">
                  <Radar size={48} className="mx-auto mb-4 opacity-50" />
                  <p>Submit a requirement to see evaluation results</p>
                </div>
              </div>
            )}

            {evaluationId && isProcessing && (
              <div className="h-96 flex items-center justify-center text-gray-400">
                <div className="text-center">
                  <Loader2 size={48} className="mx-auto mb-4 animate-spin" />
                  <p>Analyzing requirement...</p>
                  <p className="text-sm mt-2">This may take up to 2 minutes</p>
                </div>
              </div>
            )}

            {evaluation && evaluation.status === 'COMPLETED' && evaluation.result && (
              <div className="space-y-6">
                <div className="text-center p-6 bg-gradient-to-br from-primary-light to-blue-50 rounded-lg">
                  <div className={`inline-flex items-center justify-center w-20 h-20 rounded-full text-3xl font-bold mb-3 ${
                    evaluation.result.level === 'S' ? 'bg-yellow-400 text-yellow-900' :
                    evaluation.result.level === 'A' ? 'bg-green-500 text-white' :
                    evaluation.result.level === 'B' ? 'bg-orange-400 text-white' :
                    'bg-gray-400 text-white'
                  }`}>
                    {evaluation.result.level}
                  </div>
                  <div className="text-4xl font-bold text-gray-800">{Math.round(evaluation.result.overallScore)}</div>
                  <div className="text-gray-600">AI Readiness Score</div>
                </div>

                <div>
                  <h3 className="font-medium mb-3">Dimension Scores</h3>
                  <div className="space-y-2">
                    {[
                      { name: 'Context Sufficiency', score: evaluation.result.contextSufficiency },
                      { name: 'Atomicity', score: evaluation.result.atomicity },
                      { name: 'Boundary Definiteness', score: evaluation.result.boundaryDefiniteness },
                      { name: 'Verifiability', score: evaluation.result.verifiability },
                      { name: 'Technical Constraints', score: evaluation.result.technicalConstraints },
                    ].map((dim) => (
                      <div key={dim.name} className="flex items-center gap-3">
                        <div className="w-40 text-sm text-gray-600">{dim.name}</div>
                        <div className="flex-1 bg-gray-100 rounded-full h-2">
                          <div
                            className={`h-2 rounded-full ${
                              dim.score >= 75 ? 'bg-green-500' : dim.score >= 60 ? 'bg-orange-500' : 'bg-red-500'
                            }`}
                            style={{ width: `${dim.score}%` }}
                          />
                        </div>
                        <div className="w-12 text-sm text-right">{Math.round(dim.score)}</div>
                      </div>
                    ))}
                  </div>
                </div>

                {evaluation.result.recommendations && JSON.parse(evaluation.result.recommendations).length > 0 && (
                  <div>
                    <h3 className="font-medium mb-3">Recommendations</h3>
                    <div className="space-y-2">
                      {JSON.parse(evaluation.result.recommendations).slice(0, 3).map((rec: any, i: number) => (
                        <div key={i} className="p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                          <div className="flex items-center gap-2 mb-1">
                            <AlertTriangle size={16} className="text-yellow-600" />
                            <span className="font-medium text-sm">{rec.title}</span>
                          </div>
                          <p className="text-sm text-gray-600">{rec.description}</p>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )}

            {evaluation && evaluation.status === 'FAILED' && (
              <div className="h-96 flex items-center justify-center text-red-500">
                <div className="text-center">
                  <AlertTriangle size={48} className="mx-auto mb-4" />
                  <p>Evaluation failed. Please try again.</p>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </DashboardLayout>
  );
}