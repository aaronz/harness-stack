import { useState, useEffect } from 'react';

interface HistoryItem {
  id: string;
  text: string;
  submittedAt: string;
  score?: {
    totalScore: number;
    level: string;
  };
}

interface RequirementsHistoryProps {
  onSelectRequirement?: (id: string) => void;
}

export default function RequirementsHistory({ onSelectRequirement }: RequirementsHistoryProps) {
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchHistory();
  }, []);

  const fetchHistory = async () => {
    try {
      const response = await fetch('/api/requirements?limit=20');
      const data = await response.json();
      setHistory(data.requirements || []);
    } catch (err) {
      setError('Failed to load history');
    } finally {
      setIsLoading(false);
    }
  };

  const getLevelColor = (level?: string) => {
    const colors: Record<string, string> = {
      S: 'bg-green-100 text-green-800',
      A: 'bg-blue-100 text-blue-800',
      B: 'bg-yellow-100 text-yellow-800',
      C: 'bg-red-100 text-red-800',
    };
    return colors[level || ''] || 'bg-gray-100 text-gray-800';
  };

  if (isLoading) {
    return <div className="p-4 text-gray-500">Loading history...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">{error}</div>;
  }

  if (history.length === 0) {
    return <div className="p-4 text-gray-500">No requirements history yet</div>;
  }

  return (
    <div className="bg-white rounded-lg border border-gray-200 overflow-hidden">
      <div className="p-4 border-b border-gray-200">
        <h3 className="text-lg font-semibold">Requirements History</h3>
      </div>
      <div className="divide-y divide-gray-200 max-h-96 overflow-y-auto">
        {history.map((item) => (
          <button
            key={item.id}
            onClick={() => onSelectRequirement?.(item.id)}
            className="w-full p-4 text-left hover:bg-gray-50 transition-colors"
          >
            <div className="flex items-start justify-between">
              <div className="flex-1 min-w-0">
                <p className="text-sm text-gray-900 truncate">
                  {item.text.substring(0, 100)}
                  {item.text.length > 100 ? '...' : ''}
                </p>
                <p className="text-xs text-gray-500 mt-1">
                  {new Date(item.submittedAt).toLocaleString()}
                </p>
              </div>
              {item.score && (
                <span className={`ml-3 px-2 py-1 rounded text-xs font-medium ${getLevelColor(item.score.level)}`}>
                  {item.score.level} ({item.score.totalScore})
                </span>
              )}
            </div>
          </button>
        ))}
      </div>
    </div>
  );
}
