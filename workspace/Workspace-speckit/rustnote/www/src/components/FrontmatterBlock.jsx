import { useState } from 'react';

const FRONTMATTER_REGEX = /^---\r?\n([\s\S]*?)\r?\n---\r?\n?/;

export function parseFrontmatter(rawFrontmatter) {
  if (!rawFrontmatter) return null;
  
  const lines = rawFrontmatter.split('\n');
  const metadata = {};
  
  for (const line of lines) {
    const match = line.match(/^(\s*)([^:]+):(\s*)(.*)$/);
    if (match) {
      const [, leadingSpace, key, , value] = match;
      metadata[key.trim()] = value.trim();
    }
  }
  
  return Object.keys(metadata).length > 0 ? metadata : null;
}

export function extractFrontmatter(content) {
  if (!content || typeof content !== 'string') {
    return { frontmatter: null, contentWithoutFrontmatter: content };
  }

  const match = content.match(FRONTMATTER_REGEX);
  if (match) {
    return {
      frontmatter: match[0],
      contentWithoutFrontmatter: content.slice(match[0].length),
    };
  }
  return { frontmatter: null, contentWithoutFrontmatter: content };
}

export function isValidFrontmatter(rawFrontmatter) {
  if (!rawFrontmatter) return true;
  const content = rawFrontmatter.replace(/^---\r?\n?/, '').replace(/\r?\n?---$/, '');
  const lines = content.split('\n');
  let valid = true;
  for (const line of lines) {
    if (line.trim() === '') continue;
    if (line.trim() === '---') continue;
    if (!line.match(/^(\s*)([^:]+):(\s*)(.*)$/)) {
      valid = false;
      break;
    }
  }
  return valid;
}

export default function FrontmatterBlock({ frontmatter, onToggle }) {
  const [isCollapsed, setIsCollapsed] = useState(true);
  const metadata = parseFrontmatter(frontmatter || '');
  const isValid = isValidFrontmatter(frontmatter || '');

  const handleToggle = () => {
    setIsCollapsed(!isCollapsed);
    if (onToggle) onToggle(!isCollapsed);
  };

  if (!frontmatter) return null;

  const lines = frontmatter.split('\n');

  return (
    <div
      className="frontmatter-block"
      style={{
        background: 'var(--bg-secondary)',
        border: '1px solid var(--border-color)',
        borderRadius: '6px',
        margin: '0 0 16px 0',
        overflow: 'hidden',
        fontFamily: 'var(--font-mono)',
        fontSize: '13px',
      }}
    >
      <div
        className="frontmatter-header"
        onClick={handleToggle}
        style={{
          display: 'flex',
          alignItems: 'center',
          padding: '8px 12px',
          cursor: 'pointer',
          userSelect: 'none',
          borderBottom: isCollapsed ? 'none' : '1px solid var(--border-color)',
          background: 'var(--bg-hover)',
        }}
      >
        <span
          className="frontmatter-toggle"
          style={{
            display: 'inline-block',
            width: 0,
            height: 0,
            borderTop: '5px solid transparent',
            borderBottom: '5px solid transparent',
            borderLeft: '6px solid var(--text-secondary)',
            marginRight: '8px',
            transition: 'transform 0.15s ease',
            transform: isCollapsed ? 'rotate(0deg)' : 'rotate(90deg)',
          }}
        />
        <span
          className="frontmatter-title"
          style={{
            fontWeight: 600,
            color: 'var(--text-secondary)',
            fontSize: '12px',
            textTransform: 'uppercase',
            letterSpacing: '0.5px',
          }}
        >
          Frontmatter
        </span>
        {metadata && (
          <span
            className="frontmatter-count"
            style={{
              marginLeft: '8px',
              padding: '2px 6px',
              background: 'var(--accent-color)',
              color: 'white',
              borderRadius: '10px',
              fontSize: '10px',
              fontWeight: 500,
            }}
          >
            {Object.keys(metadata).length} fields
          </span>
        )}
        {!isValid && (
          <span
            className="frontmatter-invalid"
            style={{
              marginLeft: '8px',
              padding: '2px 6px',
              background: '#dc3545',
              color: 'white',
              borderRadius: '10px',
              fontSize: '10px',
              fontWeight: 500,
            }}
          >
            Invalid
          </span>
        )}
      </div>
      <div
        className="frontmatter-body"
        style={{
          display: isCollapsed ? 'none' : 'block',
          padding: '12px',
        }}
      >
        {!isValid && (
          <div
            className="frontmatter-error"
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              marginBottom: '8px',
              padding: '8px',
              background: '#fff3cd',
              border: '1px solid #ffc107',
              borderRadius: '4px',
              color: '#856404',
              fontSize: '12px',
            }}
          >
            <span style={{ fontWeight: 'bold' }}>!</span>
            <span>Invalid frontmatter format</span>
          </div>
        )}
        <div className="frontmatter-content" style={{ fontFamily: 'var(--font-mono)' }}>
          {lines.map((line, index) => {
            const keyValueMatch = line.match(/^(\s*)([^:]+):(\s*)(.*)$/);
            if (keyValueMatch) {
              const [, leadingSpace, key, , value] = keyValueMatch;
              return (
                <div
                  key={index}
                  className="frontmatter-line"
                  style={{
                    display: 'flex',
                    marginBottom: '4px',
                  }}
                >
                  <span
                    className="frontmatter-key"
                    style={{
                      color: 'var(--accent-color)',
                      fontWeight: 500,
                    }}
                  >
                    {leadingSpace}{key}:
                  </span>
                  <span className="frontmatter-colon" style={{ marginRight: '4px' }}>:</span>
                  <span
                    className="frontmatter-value"
                    style={{
                      color: 'var(--text-primary)',
                    }}
                  >
                    {value}
                  </span>
                </div>
              );
            } else if (line.trim() === '---') {
              return (
                <div
                  key={index}
                  className="frontmatter-delimiter"
                  style={{
                    color: 'var(--text-secondary)',
                    marginBottom: '4px',
                  }}
                >
                  {line}
                </div>
              );
            } else if (line.trim() !== '') {
              return (
                <div
                  key={index}
                  className="frontmatter-raw"
                  style={{
                    color: 'var(--text-secondary)',
                    fontFamily: 'var(--font-mono)',
                  }}
                >
                  {line}
                </div>
              );
            }
            return null;
          })}
        </div>
      </div>
    </div>
  );
}
