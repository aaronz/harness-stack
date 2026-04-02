'use client'

import { useState } from 'react'

interface LLMConfig {
  id: string
  name: string
  provider: string
  model: string
  baseUrl?: string | null
  isDefault: boolean
}

interface LLMConfigFormProps {
  configs: LLMConfig[]
  onRefresh: () => void
}

export default function LLMConfigForm({ configs, onRefresh }: LLMConfigFormProps) {
  const [showForm, setShowForm] = useState(false)
  const [formData, setFormData] = useState({
    name: '',
    provider: 'openai',
    model: 'gpt-4o',
    apiKey: '',
    baseUrl: '',
    isDefault: false,
  })
  const [saving, setSaving] = useState(false)
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    
    try {
      await fetch('/api/config/llm', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      })
      setShowForm(false)
      setFormData({
        name: '',
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: '',
        baseUrl: '',
        isDefault: false,
      })
      onRefresh()
    } catch (error) {
      console.error('Failed to save config:', error)
    } finally {
      setSaving(false)
    }
  }
  
  const handleDelete = async (id: string) => {
    if (!confirm('Delete this configuration?')) return
    
    await fetch(`/api/config/llm/${id}`, { method: 'DELETE' })
    onRefresh()
  }
  
  const providerModels: Record<string, string[]> = {
    openai: ['gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo', 'gpt-4'],
    anthropic: ['claude-3-5-sonnet-20241022', 'claude-3-opus-20240229', 'claude-3-sonnet-20240229'],
    google: ['gemini-1.5-pro', 'gemini-1.5-flash', 'gemini-1.5-flash-8b'],
    ollama: ['llama3', 'mistral', 'codellama', 'phi3'],
  }
  
  return (
    <div className="space-y-6">
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-medium">LLM Configurations</h2>
          <button
            onClick={() => setShowForm(!showForm)}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            {showForm ? 'Cancel' : '+ Add Model'}
          </button>
        </div>
        
        {showForm && (
          <form onSubmit={handleSubmit} className="space-y-4 mb-6 border-b pb-6">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Name
                </label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="My GPT-4o"
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Provider
                </label>
                <select
                  value={formData.provider}
                  onChange={(e) => setFormData({ ...formData, provider: e.target.value, model: providerModels[e.target.value as keyof typeof providerModels][0] })}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                >
                  <option value="openai">OpenAI</option>
                  <option value="anthropic">Anthropic</option>
                  <option value="google">Google Gemini</option>
                  <option value="ollama">Ollama (Local)</option>
                </select>
              </div>
            </div>
            
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Model
                </label>
                <select
                  value={formData.model}
                  onChange={(e) => setFormData({ ...formData, model: e.target.value })}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                >
                  {providerModels[formData.provider as keyof typeof providerModels]?.map(m => (
                    <option key={m} value={m}>{m}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Base URL (optional)
                </label>
                <input
                  type="text"
                  value={formData.baseUrl}
                  onChange={(e) => setFormData({ ...formData, baseUrl: e.target.value })}
                  placeholder="For proxy/custom endpoints"
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                />
              </div>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                API Key
              </label>
              <input
                type="password"
                value={formData.apiKey}
                onChange={(e) => setFormData({ ...formData, apiKey: e.target.value })}
                placeholder="sk-..."
                className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                required
              />
            </div>
            
            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                id="isDefault"
                checked={formData.isDefault}
                onChange={(e) => setFormData({ ...formData, isDefault: e.target.checked })}
                className="rounded border-gray-300"
              />
              <label htmlFor="isDefault" className="text-sm text-gray-700">
                Set as default model
              </label>
            </div>
            
            <button
              type="submit"
              disabled={saving}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50"
            >
              {saving ? 'Saving...' : 'Save Configuration'}
            </button>
          </form>
        )}
        
        {configs.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            No LLM configurations. Add one to get started.
          </div>
        ) : (
          <div className="space-y-2">
            {configs.map((config) => (
              <div
                key={config.id}
                className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
              >
                <div className="flex items-center gap-3">
                  <span className="text-2xl">
                    {config.provider === 'openai' && '🔵'}
                    {config.provider === 'anthropic' && '🟣'}
                    {config.provider === 'google' && '🟠'}
                    {config.provider === 'ollama' && '🟢'}
                  </span>
                  <div>
                    <div className="font-medium">
                      {config.name}
                      {config.isDefault && (
                        <span className="ml-2 px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded">
                          Default
                        </span>
                      )}
                    </div>
                    <div className="text-sm text-gray-500">
                      {config.provider} / {config.model}
                    </div>
                  </div>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => handleDelete(config.id)}
                    className="text-red-600 hover:text-red-900 text-sm"
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
