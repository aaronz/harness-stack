'use client';

import { useState } from 'react';
import RequirementInput from '@/components/RequirementInput';
import EvaluationResult from '@/components/EvaluationResult';

export default function Home() {
  const [requirementText, setRequirementText] = useState('');
  const [result, setResult] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const handleEvaluate = async () => {
    if (!requirementText.trim()) return;

    setLoading(true);
    try {
      const response = await fetch('http://localhost:3001/api/evaluate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ requirementText })
      });
      const data = await response.json();
      setResult(data);
    } catch (error) {
      console.error('Evaluation failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="min-h-screen p-4 sm:p-8 max-w-6xl mx-auto">
      <h1 className="text-2xl sm:text-3xl font-bold mb-8 text-center">
        AI Coding 可落地性评估系统
      </h1>

      <RequirementInput
        value={requirementText}
        onChange={setRequirementText}
        onSubmit={handleEvaluate}
        loading={loading}
      />

      {result && <EvaluationResult result={result} />}
    </main>
  );
}
