interface Suggestion {
  category: string;
  description: string;
  priority: string;
}

interface OptimizationSuggestionsProps {
  suggestions: Suggestion[];
}

export default function OptimizationSuggestions({ suggestions }: OptimizationSuggestionsProps) {
  if (!suggestions || suggestions.length === 0) {
    return null;
  }

  const getCategoryColor = (category: string) => {
    const colors: Record<string, string> = {
      context: 'bg-blue-500',
      atomicity: 'bg-purple-500',
      boundary: 'bg-orange-500',
      verifiability: 'bg-green-500',
    };
    return colors[category] || 'bg-gray-500';
  };

  const getPriorityColor = (priority: string) => {
    const colors: Record<string, string> = {
      high: 'text-red-600',
      medium: 'text-yellow-600',
      low: 'text-green-600',
    };
    return colors[priority] || 'text-gray-600';
  };

  const getCategoryLabel = (category: string) => {
    const labels: Record<string, string> = {
      context: '上下文增强',
      atomicity: '原子化改进',
      boundary: '边界完备性',
      verifiability: '可验证性增强',
    };
    return labels[category] || category;
  };

  const getPriorityLabel = (priority: string) => {
    const labels: Record<string, string> = {
      high: '高优先级',
      medium: '中优先级',
      low: '低优先级',
    };
    return labels[priority] || priority;
  };

  return (
    <div className="mt-6">
      <h3 className="text-lg font-semibold mb-3">Optimization Suggestions</h3>
      <div className="space-y-3">
        {suggestions.map((suggestion, index) => (
          <div key={index} className="p-4 bg-white rounded-lg border border-gray-200 shadow-sm">
            <div className="flex items-start justify-between">
              <div className="flex items-center gap-2">
                <span className={`text-xs text-white px-2 py-1 rounded ${getCategoryColor(suggestion.category)}`}>
                  {getCategoryLabel(suggestion.category)}
                </span>
              </div>
              <span className={`text-xs font-medium ${getPriorityColor(suggestion.priority)}`}>
                {getPriorityLabel(suggestion.priority)}
              </span>
            </div>
            <p className="text-gray-700 mt-2">{suggestion.description}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
