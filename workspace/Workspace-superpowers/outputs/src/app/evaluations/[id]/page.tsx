'use client'

import { useState, useEffect } from 'react'
import ScoreCard from '@/components/ScoreCard'
import RiskHighlight from '@/components/RiskHighlight'
import EditableTitle from '@/components/EditableTitle'
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
  rawResponse?: string
  risks?: string[]
}

export default function EvaluationPage({ params }: { params: { id: string } }) {
  const [evaluation, setEvaluation] = useState<EvaluationDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const [activeDimension, setActiveDimension] = useState<string>('context')
  const router = useRouter()
  
  useEffect(() => {
    fetch(`/api/evaluations/${params.id}`)
      .then(res => res.json())
      .then(data => {
        if (data.rawResponse) {
          try {
            const raw = JSON.parse(data.rawResponse)
            data.risks = raw.risks || []
          } catch {
            data.risks = []
          }
        }
        setEvaluation(data)
      })
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
  
  const groupedSuggestions = evaluation.suggestions.reduce((acc, s) => {
    const type = s.type || 'other'
    if (!acc[type]) acc[type] = []
    acc[type].push(s.content)
    return acc
  }, {} as Record<string, string[]>)

  const dimensionLabels: Record<string, string> = {
    context: 'Context Completeness',
    atomicity: 'Logic Atomicity',
    boundary: 'Boundary Clarity',
    verifiability: 'Verifiability',
    tech: 'Technical Constraints',
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
          <EditableTitle title={evaluation.title} evaluationId={evaluation.id} />
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
          
          <div className="flex flex-wrap gap-2 mb-4">
            {Object.keys(groupedSuggestions).map(type => (
              <button
                key={type}
                onClick={() => setActiveDimension(type)}
                className={`px-3 py-1 text-sm rounded-full ${
                  activeDimension === type 
                    ? 'bg-blue-600 text-white' 
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                {dimensionLabels[type] || type} ({groupedSuggestions[type].length})
              </button>
            ))}
          </div>
          
          <ul className="space-y-2">
            {(groupedSuggestions[activeDimension] || []).map((content, index) => (
              <li key={index} className="flex items-start gap-2 text-gray-700">
                <span className="mt-1.5 w-2 h-2 rounded-full bg-blue-500 flex-shrink-0" />
                <span>{content}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {evaluation.risks && evaluation.risks.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-medium mb-4">Context Gaps & Risks</h2>
          <p className="text-sm text-gray-600 mb-4">These areas may cause AI to generate incorrect or incomplete code:</p>
          <ul className="space-y-2">
            {evaluation.risks.map((risk, index) => (
              <li key={index} className="flex items-start gap-2 text-gray-700">
                <span className="mt-1.5 w-2 h-2 rounded-full bg-red-500 flex-shrink-0" />
                <span>{risk}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-medium mb-4">Original Content</h2>
        <RiskHighlight content={evaluation.content} risks={evaluation.risks || []} />
      </div>
    </div>
  )
}
