'use client'

import { useState, useEffect } from 'react'

interface ScoreWeights {
  id: string
  name: string
  context: number
  atomicity: number
  boundary: number
  verifiability: number
  tech: number
  isDefault: boolean
  createdAt: string
}

const DEFAULT_VALUES = {
  context: 0.25,
  atomicity: 0.25,
  boundary: 0.20,
  verifiability: 0.15,
  tech: 0.15,
}

type WeightKey = 'context' | 'atomicity' | 'boundary' | 'verifiability' | 'tech'

export default function WeightsForm() {
  const [weights, setWeights] = useState<ScoreWeights[]>([])
  const [loading, setLoading] = useState(true)
  const [showForm, setShowForm] = useState(false)
  const [editingId, setEditingId] = useState<string | null>(null)
  const [formData, setFormData] = useState({ name: '', ...DEFAULT_VALUES, isDefault: false })
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  const fetchWeights = async () => {
    try {
      const res = await fetch('/api/config/weights')
      if (res.ok) setWeights(await res.json())
    } catch (err) {
      console.error('Failed to fetch weights:', err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => { fetchWeights() }, [])

  const total = formData.context + formData.atomicity + formData.boundary +
                 formData.verifiability + formData.tech

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')
    if (Math.abs(total - 1.0) > 0.01) {
      setError(`Weights must sum to 1.0 (current: ${total.toFixed(2)})`)
      return
    }
    setSaving(true)
    try {
      const url = editingId ? `/api/config/weights/${editingId}` : '/api/config/weights'
      const method = editingId ? 'PUT' : 'POST'
      await fetch(url, { method, headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(formData) })
      setShowForm(false)
      setEditingId(null)
      setFormData({ name: '', ...DEFAULT_VALUES, isDefault: false })
      fetchWeights()
    } catch { setError('Failed to save weights') } finally { setSaving(false) }
  }

  const handleEdit = (w: ScoreWeights) => {
    setEditingId(w.id)
    setFormData({ name: w.name, context: w.context, atomicity: w.atomicity, boundary: w.boundary, verifiability: w.verifiability, tech: w.tech, isDefault: w.isDefault })
    setShowForm(true)
  }

  const handleDelete = async (id: string) => {
    if (!confirm('Delete this weights preset?')) return
    await fetch(`/api/config/weights/${id}`, { method: 'DELETE' })
    fetchWeights()
  }

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-medium">Score Weights Configuration</h2>
        <button onClick={() => setShowForm(!showForm)} className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
          {showForm ? 'Cancel' : '+ Add Preset'}
        </button>
      </div>

      {error && <div className="text-red-600 text-sm mb-4">{error}</div>}

      {showForm && (
        <form onSubmit={handleSubmit} className="space-y-4 mb-6 border-b pb-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Preset Name</label>
            <input type="text" value={formData.name} onChange={(e) => setFormData({ ...formData, name: e.target.value })} placeholder="e.g., Strict Evaluation" className="block w-full px-3 py-2 border border-gray-300 rounded-md" required />
          </div>
          <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
            {(['context', 'atomicity', 'boundary', 'verifiability', 'tech'] as WeightKey[]).map((key) => (
              <div key={key}>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  {key === 'context' && 'Context (25%)'}
                  {key === 'atomicity' && 'Atomicity (25%)'}
                  {key === 'boundary' && 'Boundary (20%)'}
                  {key === 'verifiability' && 'Verifiability (15%)'}
                  {key === 'tech' && 'Tech (15%)'}
                </label>
                <input
                  type="number"
                  step="0.01"
                  min="0"
                  max="1"
                  value={formData[key]}
                  onChange={(e) => setFormData({ ...formData, [key]: parseFloat(e.target.value) || 0 })}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                />
              </div>
            ))}
          </div>
          <div className="text-sm text-gray-500">Total: {total.toFixed(2)} {Math.abs(total - 1.0) > 0.01 && <span className="text-red-600">(must be 1.0)</span>}</div>
          <div className="flex items-center gap-2">
            <input type="checkbox" id="isDefault" checked={formData.isDefault} onChange={(e) => setFormData({ ...formData, isDefault: e.target.checked })} className="rounded border-gray-300" />
            <label htmlFor="isDefault" className="text-sm text-gray-700">Set as default preset</label>
          </div>
          <div className="flex gap-2">
            <button type="submit" disabled={saving} className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50">
              {saving ? 'Saving...' : editingId ? 'Update Preset' : 'Save Preset'}
            </button>
            {showForm && <button type="button" onClick={() => { setShowForm(false); setEditingId(null); setFormData({ name: '', ...DEFAULT_VALUES, isDefault: false }); setError('') }} className="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400">Cancel</button>}
          </div>
        </form>
      )}

      {loading ? <div className="text-center py-8 text-gray-500">Loading...</div> : weights.length === 0 ? <div className="text-center py-8 text-gray-500">No weight presets configured.</div> : (
        <div className="space-y-2">
          {weights.map((w) => (
            <div key={w.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <div>
                <div className="font-medium">{w.name}{w.isDefault && <span className="ml-2 px-2 py-0.5 text-xs bg-green-100 text-green-800 rounded">Default</span>}</div>
                <div className="text-sm text-gray-500">C:{w.context} | A:{w.atomicity} | B:{w.boundary} | V:{w.verifiability} | T:{w.tech}</div>
              </div>
              <div className="flex gap-2">
                <button onClick={() => handleEdit(w)} className="text-blue-600 hover:text-blue-900 text-sm">Edit</button>
                <button onClick={() => handleDelete(w.id)} className="text-red-600 hover:text-red-900 text-sm">Delete</button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
