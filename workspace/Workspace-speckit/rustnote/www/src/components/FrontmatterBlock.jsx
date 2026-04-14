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
    const newCollapsed = !isCollapsed;
    setIsCollapsed(newCollapsed);
    if (onToggle) onToggle(newCollapsed);
  };

  if (!frontmatter || !frontmatter.trim()) return null;

  const lines = frontmatter.split('\n');

  return (
    <div className="frontmatter-block">
      <div
        className="frontmatter-header"
        onClick={handleToggle}
      >
        <span className={`frontmatter-toggle${isCollapsed ? '' : ' expanded'}`} />
        <span className="frontmatter-title">
          Frontmatter
        </span>
        {metadata && (
          <span className="frontmatter-count">
            {Object.keys(metadata).length} fields
          </span>
        )}
        {!isValid && (
          <span className="frontmatter-invalid">
            Invalid
          </span>
        )}
      </div>
      <div className="frontmatter-body" style={{ display: isCollapsed ? 'none' : 'block' }}>
        {!isValid && (
          <div className="frontmatter-error">
            <span style={{ fontWeight: 'bold' }}>!</span>
            <span>Invalid frontmatter format</span>
          </div>
        )}
        <div className="frontmatter-content">
          {lines.map((line, index) => {
            const keyValueMatch = line.match(/^(\s*)([^:]+):(\s*)(.*)$/);
            if (keyValueMatch) {
              const [, leadingSpace, key, , value] = keyValueMatch;
              return (
                <div
                  key={index}
                  className="frontmatter-line"
                >
                  <span className="frontmatter-key">
                    {leadingSpace}{key}:
                  </span>
                  <span className="frontmatter-colon">:</span>
                  <span className="frontmatter-value">
                    {value}
                  </span>
                </div>
              );
            } else if (line.trim() === '---') {
              return (
                <div
                  key={index}
                  className="frontmatter-delimiter"
                >
                  {line}
                </div>
              );
            } else if (line.trim() !== '') {
              return (
                <div
                  key={index}
                  className="frontmatter-raw"
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
