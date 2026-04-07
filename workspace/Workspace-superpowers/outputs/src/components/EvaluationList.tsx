'use client'

interface Evaluation {
  id: string
  title: string
  fileName: string | null
  overallScore: number
  grade: string
  complexity: string
  createdAt: string
}

interface EvaluationListProps {
  evaluations: Evaluation[]
  onDelete: (id: string) => void
}

export default function EvaluationList({ evaluations, onDelete }: EvaluationListProps) {
  const gradeColors: Record<string, string> = {
    S: 'bg-green-100 text-green-800',
    A: 'bg-blue-100 text-blue-800',
    B: 'bg-yellow-100 text-yellow-800',
    C: 'bg-red-100 text-red-800',
  }
  
  if (evaluations.length === 0) {
    return (
      <div className="text-center py-8 text-gray-500">
        No evaluations yet. Upload a requirement document to get started.
      </div>
    )
  }
  
  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Grade
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Title
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Score
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Date
            </th>
            <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
              Actions
            </th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {evaluations.map((eval_) => (
            <tr key={eval_.id}>
              <td className="px-6 py-4 whitespace-nowrap">
                <span className={`px-2 py-1 text-xs font-semibold rounded-full ${gradeColors[eval_.grade]}`}>
                  {eval_.grade}
                </span>
              </td>
              <td className="px-6 py-4">
                <div className="text-sm font-medium text-gray-900">{eval_.title}</div>
                {eval_.fileName && (
                  <div className="text-sm text-gray-500">{eval_.fileName}</div>
                )}
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <div className="text-sm text-gray-900">{eval_.overallScore}/100</div>
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <div className="text-sm text-gray-500">
                  {new Date(eval_.createdAt).toLocaleDateString()}
                </div>
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <a
                  href={`/evaluations/${eval_.id}`}
                  className="text-blue-600 hover:text-blue-900 mr-4"
                >
                  View
                </a>
                <button
                  onClick={() => onDelete(eval_.id)}
                  className="text-red-600 hover:text-red-900"
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
