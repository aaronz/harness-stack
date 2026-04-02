'use client'

import { useState, useEffect } from 'react'
import LLMConfigForm from '@/components/LLMConfigForm'
import { useRouter } from 'next/navigation'

interface LLMConfig {
  id: string
  name: string
  provider: string
  model: string
  baseUrl?: string | null
  isDefault: boolean
}

export default function SettingsPage() {
  const [configs, setConfigs] = useState<LLMConfig[]>([])
  const [loading, setLoading] = useState(true)
  const router = useRouter()
  
  const fetchConfigs = async () => {
    try {
      const res = await fetch('/api/config/llm')
      if (res.ok) {
        const data = await res.json()
        setConfigs(data)
      }
    } catch (error) {
      console.error('Failed to fetch configs:', error)
    } finally {
      setLoading(false)
    }
  }
  
  useEffect(() => {
    fetchConfigs()
  }, [])
  
  return (
    <div>
      <button
        onClick={() => router.back()}
        className="text-gray-600 hover:text-gray-900 flex items-center gap-2 mb-6"
      >
        ← Back
      </button>
      
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      
      {loading ? (
        <div className="text-center py-8">Loading...</div>
      ) : (
        <LLMConfigForm configs={configs} onRefresh={fetchConfigs} />
      )}
    </div>
  )
}
