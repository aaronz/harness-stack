import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useSettings } from '../contexts/SettingsContext';

export default function OutlinePanel({ onHeadingClick }) {
  const { settings } = useSettings();
  const [headings, setHeadings] = useState([]);

  useEffect(() => {
    async function fetchOutline() {
      try {
        const content = document.getElementById('editor-content')?.textContent || '';
        if (content) {
          const result = await invoke('render_for_editor', {
            markdown: content,
            cursorOffset: 0,
          });
          if (result?.headings) {
            setHeadings(result.headings);
          }
        }
      } catch (e) {
        console.error('Failed to fetch outline:', e);
      }
    }

    const interval = setInterval(fetchOutline, 2000);
    return () => clearInterval(interval);
  }, []);

  if (!settings.outlineVisible) {
    return null;
  }

  return (
    <div
      id="outline-panel"
      className="w-64 border-l overflow-y-auto"
      style={{
        backgroundColor: 'var(--bg-primary)',
        borderColor: 'var(--border-color)',
      }}
    >
      <div
        className="px-4 py-3 font-semibold text-sm border-b"
        style={{ borderColor: 'var(--border-color)' }}
      >
        Outline
      </div>

      <div id="outline-list" className="py-2">
        {headings.length === 0 ? (
          <div
            className="px-4 py-2 text-sm"
            style={{ color: 'var(--text-secondary)' }}
          >
            No headings found
          </div>
        ) : (
          headings.map((heading, index) => (
            <div
              key={index}
              onClick={() => onHeadingClick?.(heading)}
              className={`px-4 py-1.5 text-sm cursor-pointer ${
                heading.level >= 1 && heading.level <= 6 ? `pl-${Math.min(heading.level * 4, 16)}` : ''
              }`}
              style={{
                paddingLeft: `${16 + (heading.level - 1) * 12}px`,
                fontWeight: heading.level <= 2 ? 600 : 400,
                color: 'var(--text-primary)',
              }}
            >
              {heading.text}
            </div>
          ))
        )}
      </div>
    </div>
  );
}