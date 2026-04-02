'use client'

interface ScoreCardProps {
  label: string
  score: number
  maxScore?: number
}

export default function ScoreCard({ label, score, maxScore = 100 }: ScoreCardProps) {
  const percentage = (score / maxScore) * 100
  const color = percentage >= 80 ? 'bg-green-500' : percentage >= 60 ? 'bg-yellow-500' : 'bg-red-500'
  
  return (
    <div className="bg-white rounded-lg shadow p-4">
      <div className="flex justify-between items-center mb-2">
        <span className="text-sm font-medium text-gray-700">{label}</span>
        <span className="text-lg font-semibold text-gray-900">{score}</span>
      </div>
      <div className="w-full bg-gray-200 rounded-full h-2">
        <div
          className={`h-2 rounded-full ${color}`}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  )
}
