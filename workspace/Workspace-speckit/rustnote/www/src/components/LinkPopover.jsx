import { useState, useEffect, useRef, useCallback } from 'react';

export default function LinkPopover({
  isVisible,
  position,
  initialUrl = '',
  initialText = '',
  onSave,
  onCancel,
  onDelete,
}) {
  const [url, setUrl] = useState(initialUrl);
  const [text, setText] = useState(initialText);
  const urlInputRef = useRef(null);
  const popoverRef = useRef(null);

  useEffect(() => {
    if (isVisible) {
      setUrl(initialUrl);
      setText(initialText);
      setTimeout(() => {
        urlInputRef.current?.focus();
        urlInputRef.current?.select();
      }, 10);
    }
  }, [isVisible, initialUrl, initialText]);

  const handleKeyDown = useCallback((e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (url.trim()) {
        onSave(url.trim(), text.trim());
      }
    } else if (e.key === 'Escape') {
      e.preventDefault();
      onCancel();
    }
  }, [url, text, onSave, onCancel]);

  useEffect(() => {
    if (isVisible) {
      document.addEventListener('keydown', handleKeyDown);
      return () => document.removeEventListener('keydown', handleKeyDown);
    }
  }, [isVisible, handleKeyDown]);

  if (!isVisible) {
    return null;
  }

  const style = {
    position: 'absolute',
    left: position?.left ?? position?.x ?? 0,
    top: position?.top ?? position?.y ?? 0,
    zIndex: 50,
  };

  return (
    <div
      ref={popoverRef}
      className="bg-white dark:bg-gray-800 rounded-lg shadow-xl border"
      style={{
        ...style,
        borderColor: 'var(--border-color)',
        minWidth: '280px',
      }}
      onKeyDown={(e) => e.stopPropagation()}
      >
        <div
          className="flex items-center justify-between px-4 py-2 border-b"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <h3 className="text-sm font-semibold" style={{ color: 'var(--text-primary)' }}>
            Edit Link
          </h3>
        </div>

        <div className="px-4 py-3 space-y-3">
          <div>
            <label
              className="block text-xs font-medium mb-1"
              style={{ color: 'var(--text-secondary)' }}
            >
              URL
            </label>
          <input
            ref={urlInputRef}
            type="text"
            id="link-popover-url"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            placeholder="https://example.com"
            className="w-full px-3 py-2 text-sm border rounded outline-none"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          />
          </div>

          <div>
            <label
              className="block text-xs font-medium mb-1"
              style={{ color: 'var(--text-secondary)' }}
            >
              Link Text
            </label>
          <input
            type="text"
            id="link-popover-text"
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="Link text"
            className="w-full px-3 py-2 text-sm border rounded outline-none"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          />
        </div>
        </div>

        <div
          className="flex justify-between px-4 py-2 border-t"
          style={{ borderColor: 'var(--border-color)' }}
        >
        <div>
          {onDelete && (
            <button
              id="btn-link-delete"
              onClick={onDelete}
              className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
              style={{
                backgroundColor: 'transparent',
                borderColor: '#dc2626',
                color: '#dc2626',
              }}
              title="Delete link"
            >
              Delete
            </button>
          )}
        </div>
        <div className="flex gap-2">
          <button
            id="btn-link-cancel"
            onClick={onCancel}
            className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          >
            Cancel
          </button>
          <button
            id="btn-link-save"
            onClick={() => {
              if (url.trim()) {
                onSave(url.trim(), text.trim());
              }
            }}
            disabled={!url.trim()}
            className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--accent-color)',
              borderColor: 'var(--accent-color)',
              color: 'white',
            }}
          >
            Save
          </button>
        </div>
      </div>
    </div>
  );
}