'use client'

import { useState, useEffect } from 'react'
import ScoreCard from '@/components/ScoreCard'
import { useRouter } from 'next/navigation'

interface EvaluationDetail {
  id: string
  title: string
  content: string
  fileName: string | null
  overallScore: number
  grade: string
  complexity: string
  contextScore: number
  atomicityScore: number
  boundaryScore: number
  verifiabilityScore: number
  techScore: number
  suggestions: Array<{ type: string; content: string }>
  modelUsed: string
  createdAt: string
}

export default function EvaluationPage({ params }: { params: { id: string } }) {
  const [evaluation, setEvaluation] = useState<EvaluationDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const router = useRouter()
  
  useEffect(() => {
    fetch(`/api/evaluations/${params.id}`)
      .then(res => res.json())
      .then(data => setEvaluation(data))
      .catch(console.error)
      .finally(() => setLoading(false))
  }, [params.id])
  
  if (loading) {
    return <div className="text-center py-8">Loading...</div>
  }
  
  if (!evaluation) {
    return <div className="text-center py-8">Evaluation not found</div>
  }
  
  const gradeColors: Record<string, string> = {
    S: 'bg-green-100 text-green-800',
    A: 'bg-blue-100 text-blue-800',
    B: 'bg-yellow-100 text-yellow-800',
    C: 'bg-red-100 text-red-800',
  }
  
  return (
    <div className="space-y-6">
      <button
        onClick={() => router.back()}
        className="text-gray-600 hover:text-gray-900 flex items-center gap-2"
      >
        ← Back
      </button>
      
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex items-center justify-between mb-4">
          <h1 className="text-2xl font-bold text-gray-900">{evaluation.title}</h1>
          <span className={`px-3 py-1 text-lg font-semibold rounded-full ${gradeColors[evaluation.grade]}`}>
            {evaluation.grade} ({evaluation.overallScore})
          </span>
        </div>
        
        <div className="w-full bg-gray-200 rounded-full h-4 mb-6">
          <div
            className="h-4 rounded-full bg-blue-600"
            style={{ width: `${evaluation.overallScore}%` }}
          />
        </div>
        
        <div className="text-sm text-gray-500 mb-6">
          Assessed with {evaluation.modelUsed} • {new Date(evaluation.createdAt).toLocaleString()}
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
        <ScoreCard label="Context Score" score={evaluation.contextScore} />
        <ScoreCard label="Atomicity Score" score={evaluation.atomicityScore} />
        <ScoreCard label="Boundary Score" score={evaluation.boundaryScore} />
        <ScoreCard label="Verifiability" score={evaluation.verifiabilityScore} />
        <ScoreCard label="Tech Score" score={evaluation.techScore} />
      </div>
      
      {evaluation.suggestions.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-medium mb-4">Suggestions</h2>
          <ul className="space-y-2">
            {evaluation.suggestions.map((suggestion, index) => (
              <li key={index} className="flex items-start gap-2">
                <span className="px-2 py-0.5 text-xs font-medium bg-gray-100 rounded">
                  {suggestion.type}
                </span>
                <span className="text-gray-700">{suggestion.content}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
      
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-medium mb-4">Original Content</h2>
        <pre className="whitespace-pre-wrap text-sm text-gray-700 bg-gray-50 p-4 rounded-lg overflow-auto">
          {evaluation.content}
        </pre>
      </div>
    </div>
  )
}
