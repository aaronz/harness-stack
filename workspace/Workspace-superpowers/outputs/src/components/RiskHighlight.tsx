'use client'

interface RiskHighlightProps {
  content: string
  risks: string[]
}

export default function RiskHighlight({ content, risks }: RiskHighlightProps) {
  if (!risks || risks.length === 0) {
    return (
      <pre className="whitespace-pre-wrap text-sm text-gray-700 bg-gray-50 p-4 rounded-lg overflow-auto">
        {content}
      </pre>
    )
  }

  // Sort risks by length (longer matches first) to avoid partial replacements
  const sortedRisks = [...risks].sort((a, b) => b.length - a.length)

  // Escape regex special characters and create highlight pattern
  let highlighted = content

  sortedRisks.forEach((risk, index) => {
    const escapedRisk = risk.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
    const regex = new RegExp(escapedRisk, 'gi')
    highlighted = highlighted.replace(regex, `{{RISK_${index}}}`)
  })

  // Split by risk markers and render with highlighting
  const parts = highlighted.split(/({{RISK_\d+}})/g)

  return (
    <pre className="whitespace-pre-wrap text-sm text-gray-700 bg-gray-50 p-4 rounded-lg overflow-auto">
      {parts.map((part, i) => {
        const match = part.match(/{{RISK_(\d+)}}/)
        if (match) {
          const riskIndex = parseInt(match[1])
          return (
            <mark key={i} className="bg-red-200 text-red-900 px-1 rounded">
              {sortedRisks[riskIndex]}
            </mark>
          )
        }
        return <span key={i}>{part}</span>
      })}
    </pre>
  )
}
