'use client'

import { useState, useEffect } from 'react'
import FileUpload from '@/components/FileUpload'
import EvaluationList from '@/components/EvaluationList'
import { useRouter } from 'next/navigation'

interface Evaluation {
  id: string
  title: string
  fileName: string | null
  overallScore: number
  grade: string
  complexity: string
  createdAt: string
}

interface Model {
  id: string
  name: string
  model: string
  isDefault: boolean
}

export default function Home() {
  const [evaluations, setEvaluations] = useState<Evaluation[]>([])
  const [models, setModels] = useState<Model[]>([])
  const [selectedModelId, setSelectedModelId] = useState<string>('')
  const [loading, setLoading] = useState(true)
  const router = useRouter()
  
  const fetchEvaluations = async () => {
    try {
      const res = await fetch('/api/evaluations')
      if (res.ok) {
        const data = await res.json()
        setEvaluations(data)
      }
    } catch (error) {
      console.error('Failed to fetch evaluations:', error)
    } finally {
      setLoading(false)
    }
  }

  const fetchModels = async () => {
    try {
      const res = await fetch('/api/config/llm')
      if (res.ok) {
        const data = await res.json()
        setModels(data)
        const defaultModel = data.find((m: Model) => m.isDefault)
        if (defaultModel) setSelectedModelId(defaultModel.id)
      }
    } catch (error) {
      console.error('Failed to fetch models:', error)
    }
  }
  
  useEffect(() => {
    fetchEvaluations()
    fetchModels()
  }, [])
  
  const handleUpload = async (file: File, title?: string, modelId?: string) => {
    const formData = new FormData()
    formData.append('file', file)
    if (title) formData.append('title', title)
    if (modelId) formData.append('modelId', modelId)
    
    const res = await fetch('/api/evaluations', {
      method: 'POST',
      body: formData,
    })
    
    if (!res.ok) {
      const error = await res.json()
      throw new Error(error.error || 'Evaluation failed')
    }
    
    const result = await res.json()
    router.push(`/evaluations/${result.id}`)
  }
  
  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this evaluation?')) return
    
    const res = await fetch(`/api/evaluations/${id}`, {
      method: 'DELETE',
    })
    
    if (res.ok) {
      setEvaluations(evaluations.filter(e => e.id !== id))
    }
  }
  
  return (
    <div className="space-y-8">
      <FileUpload
        onUpload={handleUpload}
        models={models}
        selectedModelId={selectedModelId}
        onModelChange={setSelectedModelId}
      />
      
      <div>
        <h2 className="text-lg font-medium mb-4">Recent Evaluations</h2>
        {loading ? (
          <div className="text-center py-8 text-gray-500">Loading...</div>
        ) : (
          <EvaluationList evaluations={evaluations} onDelete={handleDelete} />
        )}
      </div>
    </div>
  )
}
