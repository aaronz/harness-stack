'use client'

import { useState } from 'react'

interface Model {
  id: string
  name: string
  model: string
}

interface FileUploadProps {
  onUpload: (file: File, title?: string, modelId?: string) => Promise<void>
  models?: Model[]
  selectedModelId?: string
  onModelChange?: (modelId: string) => void
}

export default function FileUpload({ onUpload, models = [], selectedModelId, onModelChange }: FileUploadProps) {
  const [file, setFile] = useState<File | null>(null)
  const [title, setTitle] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!file) {
      setError('Please select a file')
      return
    }
    
    setLoading(true)
    setError('')
    
    try {
      await onUpload(file, title, selectedModelId)
      setFile(null)
      setTitle('')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Upload failed')
    } finally {
      setLoading(false)
    }
  }
  
  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-lg font-medium mb-4">Upload Requirement Document</h2>
      
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            File (TXT or Markdown)
          </label>
          <input
            type="file"
            accept=".txt,.md"
            onChange={(e) => setFile(e.target.files?.[0] || null)}
            className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
          />
        </div>
        
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Title (optional)
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Auto-extracted from filename"
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        {models.length > 0 && onModelChange && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Model (optional)
            </label>
            <select
              value={selectedModelId || ''}
              onChange={(e) => onModelChange(e.target.value)}
              className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">Use default model</option>
              {models.map(m => (
                <option key={m.id} value={m.id}>{m.name} ({m.model})</option>
              ))}
            </select>
          </div>
        )}
        
        {error && (
          <div className="text-red-600 text-sm">{error}</div>
        )}
        
        <button
          type="submit"
          disabled={!file || loading}
          className="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? 'Evaluating...' : 'Evaluate'}
        </button>
      </form>
    </div>
  )
}
