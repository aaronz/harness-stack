import { useEffect, useState } from 'react';

export default function RecoveryModal({ 
  isVisible, 
  snapshots, 
  onRecover,
  onStartFresh,
  onDeleteSnapshot,
  isLoading 
}) {
  const [selectedId, setSelectedId] = useState(null);

  useEffect(() => {
    if (snapshots && snapshots.length > 0) {
      setSelectedId(snapshots[0].id);
    }
  }, [snapshots]);

  useEffect(() => {
    const handleKeyDown = (e) => {
      if (!isVisible) return;
      
      if (e.key === 'Escape') {
        onStartFresh();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isVisible, onStartFresh]);

  if (!isVisible) {
    return null;
  }

  const formatTimestamp = (timestamp) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diffMs = now - date;
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins} minute${diffMins > 1 ? 's' : ''} ago`;
    if (diffHours < 24) return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`;
    if (diffDays < 7) return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`;
    
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
  };

  const handleRecover = () => {
    if (selectedId) {
      onRecover(selectedId);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/50" onClick={onStartFresh} />
      
      <div
        className="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-2xl mx-4 max-h-[80vh] flex flex-col"
        style={{
          backgroundColor: 'var(--bg-primary)',
          color: 'var(--text-primary)',
        }}
      >
        <div
          className="flex items-center justify-between px-6 py-4 border-b flex-shrink-0"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <div className="flex items-center gap-3">
            <div
              className="flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center"
              style={{ backgroundColor: 'var(--accent-color)', opacity: 0.2 }}
            >
              <svg
                className="w-5 h-5"
                style={{ color: 'var(--accent-color)' }}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                />
              </svg>
            </div>
            <div>
              <h2 className="text-lg font-semibold">Crash Recovery</h2>
              <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                Found {snapshots?.length || 0} recovery snapshot{snapshots?.length !== 1 ? 's' : ''}
              </p>
            </div>
          </div>
          <button
            onClick={onStartFresh}
            className="text-2xl leading-none opacity-60 hover:opacity-100"
            style={{ color: 'var(--text-primary)' }}
          >
            ×
          </button>
        </div>

        <div className="flex-1 overflow-y-auto px-6 py-4">
          {snapshots && snapshots.length > 0 ? (
            <div className="space-y-3">
              {snapshots.map((snapshot) => (
                <div
                  key={snapshot.id}
                  className={`p-4 rounded-lg border-2 cursor-pointer transition-all ${
                    selectedId === snapshot.id 
                      ? 'border-[var(--accent-color)]' 
                      : 'border-transparent'
                  }`}
                  style={{ 
                    backgroundColor: selectedId === snapshot.id 
                      ? 'var(--bg-secondary)' 
                      : 'transparent',
                    borderColor: selectedId === snapshot.id ? 'var(--accent-color)' : 'var(--border-color)',
                  }}
                  onClick={() => setSelectedId(snapshot.id)}
                >
                  <div className="flex items-start justify-between gap-4">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-1">
                        <h3 className="font-medium truncate" style={{ color: 'var(--text-primary)' }}>
                          {snapshot.title || 'Untitled'}
                        </h3>
                        {snapshot.file_path && (
                          <span 
                            className="text-xs px-2 py-0.5 rounded flex-shrink-0"
                            style={{ 
                              backgroundColor: 'var(--bg-secondary)',
                              color: 'var(--text-secondary)'
                            }}
                          >
                            {snapshot.file_path.split('/').pop()}
                          </span>
                        )}
                      </div>
                      <p className="text-xs mb-2" style={{ color: 'var(--text-secondary)' }}>
                        {formatTimestamp(snapshot.timestamp)}
                      </p>
                      <p 
                        className="text-sm truncate" 
                        style={{ color: 'var(--text-secondary)' }}
                      >
                        {snapshot.content_preview || 'No content preview'}
                      </p>
                    </div>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        onDeleteSnapshot(snapshot.id);
                      }}
                      className="flex-shrink-0 p-1 opacity-60 hover:opacity-100"
                      style={{ color: 'var(--text-secondary)' }}
                      title="Delete this snapshot"
                    >
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-8" style={{ color: 'var(--text-secondary)' }}>
              <p>No recovery snapshots found.</p>
            </div>
          )}
        </div>

        <div
          className="flex justify-end gap-2 px-6 py-4 border-t flex-shrink-0"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <button
            onClick={onStartFresh}
            disabled={isLoading}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          >
            Start Fresh
          </button>
          <button
            onClick={handleRecover}
            disabled={isLoading || !selectedId}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--accent-color)',
              borderColor: 'var(--accent-color)',
              color: 'white',
            }}
          >
            {isLoading ? 'Recovering...' : 'Recover Selected'}
          </button>
        </div>
      </div>
    </div>
  );
}