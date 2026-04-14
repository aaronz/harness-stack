import { useState, useEffect, useRef, useCallback } from 'react';

/**
 * Validates URL protocol for security.
 * Blocks dangerous protocols: javascript:, data:, vbscript:
 * Allows safe protocols: http:, https:, mailto:
 * 
 * @param {string} url - The URL to validate
 * @returns {{ valid: boolean, error: string | null }}
 */
export function validateUrlProtocol(url) {
  if (!url || !url.trim()) {
    return { valid: false, error: 'URL is required' };
  }
  
  const trimmedUrl = url.trim().toLowerCase();
  
  // List of dangerous protocols to block
  const dangerousProtocols = [
    'javascript:',
    'data:',
    'vbscript:',
  ];
  
  for (const protocol of dangerousProtocols) {
    if (trimmedUrl.startsWith(protocol)) {
      return { 
        valid: false, 
        error: `Invalid URL protocol. '${protocol}' is not allowed for security reasons.` 
      };
    }
  }
  
  // List of allowed protocols
  const allowedProtocols = [
    'http://',
    'https://',
    'mailto:',
  ];
  
  const hasAllowedProtocol = allowedProtocols.some(protocol => 
    trimmedUrl.startsWith(protocol.toLowerCase())
  );
  
  if (!hasAllowedProtocol) {
    return { 
      valid: false, 
      error: 'Invalid URL protocol. Only http://, https://, and mailto: are allowed.' 
    };
  }
  
  return { valid: true, error: null };
}

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
  const [urlError, setUrlError] = useState(null);
  const urlInputRef = useRef(null);
  const popoverRef = useRef(null);

  useEffect(() => {
    if (isVisible) {
      setUrl(initialUrl);
      setText(initialText);
      setUrlError(null);
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
        const validation = validateUrlProtocol(url.trim());
        if (validation.valid) {
          setUrlError(null);
          onSave(url.trim(), text.trim());
        } else {
          setUrlError(validation.error);
        }
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
            onChange={(e) => {
              setUrl(e.target.value);
              if (urlError) {
                const validation = validateUrlProtocol(e.target.value);
                if (validation.valid) {
                  setUrlError(null);
                }
              }
            }}
            placeholder="https://example.com"
            className="w-full px-3 py-2 text-sm border rounded outline-none"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: urlError ? '#dc2626' : 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          />
          {urlError && (
            <p 
              id="link-popover-error"
              className="text-xs mt-1" 
              style={{ color: '#dc2626' }}
            >
              {urlError}
            </p>
          )}
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
                const validation = validateUrlProtocol(url.trim());
                if (validation.valid) {
                  setUrlError(null);
                  onSave(url.trim(), text.trim());
                } else {
                  setUrlError(validation.error);
                }
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