interface RiskHighlight {
  dimension: string;
  textSpan: string;
  reason: string;
}

interface RiskHeatmapProps {
  highlights: RiskHighlight[];
}

export default function RiskHeatmap({ highlights }: RiskHeatmapProps) {
  if (!highlights || highlights.length === 0) {
    return null;
  }

  const getDimensionColor = (dimension: string) => {
    const colors: Record<string, string> = {
      context: 'bg-blue-100 border-blue-300 text-blue-800',
      atomicity: 'bg-purple-100 border-purple-300 text-purple-800',
      boundary: 'bg-orange-100 border-orange-300 text-orange-800',
      verifiability: 'bg-green-100 border-green-300 text-green-800',
      technical: 'bg-red-100 border-red-300 text-red-800',
    };
    return colors[dimension] || 'bg-gray-100 border-gray-300 text-gray-800';
  };

  const getPriorityBadge = (reason: string) => {
    if (reason.includes('未') || reason.includes('无')) {
      return <span className="text-xs bg-red-500 text-white px-2 py-0.5 rounded">高风险</span>;
    }
    return <span className="text-xs bg-yellow-500 text-white px-2 py-0.5 rounded">中风险</span>;
  };

  return (
    <div className="mt-6">
      <h3 className="text-lg font-semibold mb-3">Risk Heatmap</h3>
      <div className="space-y-2">
        {highlights.map((highlight, index) => (
          <div
            key={index}
            className={`p-3 rounded-lg border-2 ${getDimensionColor(highlight.dimension)}`}
          >
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-xs font-medium uppercase">{highlight.dimension}</span>
                  {getPriorityBadge(highlight.reason)}
                </div>
                <div className="text-sm font-mono bg-white/50 px-2 py-1 rounded">
                  "{highlight.textSpan}"
                </div>
                <div className="text-sm mt-1">{highlight.reason}</div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
