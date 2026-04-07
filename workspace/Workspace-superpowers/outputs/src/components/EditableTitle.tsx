'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'

interface EditableTitleProps {
  title: string
  evaluationId: string
}

export default function EditableTitle({ title, evaluationId }: EditableTitleProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [editValue, setEditValue] = useState(title)
  const [saving, setSaving] = useState(false)
  const router = useRouter()

  const handleSave = async () => {
    if (editValue.trim() === title) { setIsEditing(false); return }
    setSaving(true)
    try {
      await fetch(`/api/evaluations/${evaluationId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ title: editValue.trim() })
      })
      setIsEditing(false)
      router.refresh()
    } catch (error) {
      console.error('Failed to update title:', error)
    } finally {
      setSaving(false)
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') handleSave()
    else if (e.key === 'Escape') { setEditValue(title); setIsEditing(false) }
  }

  if (isEditing) {
    return (
      <div className="flex items-center gap-2">
        <input
          type="text"
          value={editValue}
          onChange={(e) => setEditValue(e.target.value)}
          onKeyDown={handleKeyDown}
          onBlur={handleSave}
          autoFocus
          className="text-2xl font-bold text-gray-900 border border-blue-300 rounded px-2 py-1"
        />
        <button onClick={handleSave} disabled={saving} className="text-blue-600 hover:text-blue-800 text-sm">
          {saving ? 'Saving...' : 'Save'}
        </button>
      </div>
    )
  }

  return (
    <div className="flex items-center gap-2">
      <h1 className="text-2xl font-bold text-gray-900">{title}</h1>
      <button onClick={() => setIsEditing(true)} className="text-gray-400 hover:text-gray-600 text-sm" title="Edit title">
        ✏️
      </button>
    </div>
  )
}
