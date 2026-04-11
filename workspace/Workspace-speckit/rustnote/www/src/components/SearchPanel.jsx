import { useState, useEffect, useCallback, useRef } from 'react';
import { useDocument } from '../contexts/DocumentContext';

export default function SearchPanel({ isVisible, onClose }) {
  const { currentDocument, updateContent } = useDocument();
  const [searchQuery, setSearchQuery] = useState('');
  const [replaceQuery, setReplaceQuery] = useState('');
  const [caseSensitive, setCaseSensitive] = useState(false);
  const [matchCount, setMatchCount] = useState(0);
  const [currentMatch, setCurrentMatch] = useState(0);
  const searchInputRef = useRef(null);

  useEffect(() => {
    if (isVisible && searchInputRef.current) {
      searchInputRef.current.focus();
    }
  }, [isVisible]);

  const escapeRegex = (string) => string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

  const performSearch = useCallback((query, docContent, caseSens) => {
    if (!query || !docContent) {
      setMatchCount(0);
      setCurrentMatch(0);
      return;
    }

    try {
      const flags = caseSens ? 'g' : 'gi';
      const regex = new RegExp(escapeRegex(query), flags);
      const matches = docContent.match(regex);
      setMatchCount(matches ? matches.length : 0);
      setCurrentMatch(matches && matches.length > 0 ? 1 : 0);
    } catch (e) {
      setMatchCount(0);
      setCurrentMatch(0);
    }
  }, []);

  const handleSearchChange = (e) => {
    const query = e.target.value;
    setSearchQuery(query);
    if (currentDocument?.content !== undefined) {
      performSearch(query, currentDocument.content, caseSensitive);
    }
  };

  const handleReplaceChange = (e) => {
    setReplaceQuery(e.target.value);
  };

  const handleCaseSensitiveToggle = () => {
    const newValue = !caseSensitive;
    setCaseSensitive(newValue);
    if (searchQuery && currentDocument?.content !== undefined) {
      performSearch(searchQuery, currentDocument.content, newValue);
    }
  };

  const getCursorOffset = () => {
    const editor = document.getElementById('editor-content');
    if (!editor) return 0;

    const selection = window.getSelection();
    if (!selection.rangeCount) return 0;

    const range = selection.getRangeAt(0);
    const preCaretRange = range.cloneRange();
    preCaretRange.selectNodeContents(editor);
    preCaretRange.setEnd(range.startContainer, range.startOffset);
    return preCaretRange.toString().length;
  };

  const findNext = useCallback(() => {
    if (!searchQuery || !currentDocument?.content || matchCount === 0) return;

    const content = currentDocument.content;
    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const regex = new RegExp(escapeRegex(searchQuery), flags);

      const editor = document.getElementById('editor-content');
      if (!editor) return;

      const selection = window.getSelection();
      if (!selection.rangeCount) return;

      const range = selection.getRangeAt(0);
      const preCaretRange = range.cloneRange();
      preCaretRange.selectNodeContents(editor);
      preCaretRange.setEnd(range.startContainer, range.startOffset);
      const cursorPos = preCaretRange.toString().length;

      let match;
      let searchFrom = cursorPos;
      let foundPos = -1;

      while ((match = regex.exec(content)) !== null) {
        if (match.index > searchFrom) {
          foundPos = match.index;
          break;
        }
      }

      if (foundPos === -1) {
        regex.lastIndex = 0;
        while ((match = regex.exec(content)) !== null) {
          if (match.index >= 0) {
            foundPos = match.index;
            break;
          }
        }
      }

      if (foundPos !== -1) {
        const newRange = document.createRange();
        const walker = document.createTreeWalker(editor, NodeFilter.SHOW_TEXT, null, false);

        let charCount = 0;
        let node;
        let startNode = null;
        let startOffset = 0;
        let endNode = null;
        let endOffset = 0;

        while (node = walker.nextNode()) {
          const nodeLength = node.textContent.length;
          if (charCount + nodeLength > foundPos) {
            if (!startNode) {
              startNode = node;
              startOffset = foundPos - charCount;
            }
          }
          if (charCount + nodeLength >= foundPos + searchQuery.length) {
            endNode = node;
            endOffset = foundPos + searchQuery.length - charCount;
            break;
          }
          charCount += nodeLength;
        }

        if (startNode && endNode) {
          newRange.setStart(startNode, startOffset);
          newRange.setEnd(endNode, endOffset);
          selection.removeAllRanges();
          selection.addRange(newRange);

          const nextMatch = currentMatch >= matchCount ? 1 : currentMatch + 1;
          setCurrentMatch(nextMatch);
        }
      }
    } catch (e) {
      console.error('Find next error:', e);
    }
  }, [searchQuery, currentDocument, caseSensitive, matchCount, currentMatch]);

  const replaceMatch = useCallback(() => {
    if (!searchQuery || !currentDocument?.content) return;

    const selection = window.getSelection();
    if (!selection.rangeCount) return;

    const selectedText = selection.toString();
    if (selectedText !== searchQuery &&
        (caseSensitive ? selectedText !== searchQuery : selectedText.toLowerCase() !== searchQuery.toLowerCase())) {
      findNext();
      return;
    }

    const content = currentDocument.content;
    const cursorPos = getCursorOffset();

    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const regex = new RegExp(escapeRegex(searchQuery), flags);

      let match;
      let matchPos = -1;
      while ((match = regex.exec(content)) !== null) {
        if (match.index === cursorPos || (match.index < cursorPos && match.index + match[0].length > cursorPos)) {
          matchPos = match.index;
          break;
        }
      }

      if (matchPos === -1) {
        findNext();
        return;
      }

      const newContent = content.substring(0, matchPos) + replaceQuery + content.substring(matchPos + searchQuery.length);
      updateContent(newContent);

      const editor = document.getElementById('editor-content');
      if (editor) {
        const newRange = document.createRange();
        const walker = document.createTreeWalker(editor, NodeFilter.SHOW_TEXT, null, false);

        let charCount = 0;
        let node;
        let startNode = null;
        let startOffset = 0;
        let endNode = null;
        let endOffset = 0;

        while (node = walker.nextNode()) {
          const nodeLength = node.textContent.length;
          if (charCount + nodeLength > matchPos) {
            if (!startNode) {
              startNode = node;
              startOffset = matchPos - charCount;
            }
          }
          if (charCount + nodeLength >= matchPos + replaceQuery.length) {
            endNode = node;
            endOffset = matchPos + replaceQuery.length - charCount;
            break;
          }
          charCount += nodeLength;
        }

        if (startNode && endNode) {
          newRange.setStart(startNode, startOffset);
          newRange.setEnd(endNode, endOffset);
          selection.removeAllRanges();
          selection.addRange(newRange);
        }
      }

      performSearch(searchQuery, newContent, caseSensitive);
    } catch (e) {
      console.error('Replace error:', e);
    }
  }, [searchQuery, replaceQuery, currentDocument, caseSensitive, updateContent, findNext, performSearch]);

  const replaceAll = useCallback(() => {
    if (!searchQuery || !currentDocument?.content) return;

    const content = currentDocument.content;

    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const regex = new RegExp(escapeRegex(searchQuery), flags);
      const newContent = content.replace(regex, replaceQuery);

      updateContent(newContent);
      setMatchCount(0);
      setCurrentMatch(0);
    } catch (e) {
      console.error('Replace all error:', e);
    }
  }, [searchQuery, replaceQuery, currentDocument, caseSensitive, updateContent]);

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      findNext();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      onClose();
    }
  };

  if (!isVisible) {
    return null;
  }

  return (
    <div
      id="search-panel"
      className="flex items-center gap-2 px-4 py-2 border-b"
      style={{
        backgroundColor: 'var(--bg-secondary)',
        borderColor: 'var(--border-color)',
      }}
      onKeyDown={handleKeyDown}
    >
      <div className="flex items-center gap-1">
        <input
          ref={searchInputRef}
          type="text"
          id="search-input"
          value={searchQuery}
          onChange={handleSearchChange}
          placeholder="Find..."
          className="px-2 py-1 text-sm border rounded outline-none w-48"
          style={{
            backgroundColor: 'var(--bg-primary)',
            borderColor: 'var(--border-color)',
            color: 'var(--text-primary)',
          }}
        />

        <span
          id="match-count"
          className="text-xs px-2"
          style={{ color: 'var(--text-secondary)' }}
        >
          {matchCount > 0 ? `${currentMatch}/${matchCount}` : 'No matches'}
        </span>
      </div>

      <button
        id="btn-case-sensitive"
        onClick={handleCaseSensitiveToggle}
        className={`px-2 py-1 text-xs border rounded cursor-pointer transition-colors ${
          caseSensitive ? 'text-white' : ''
        }`}
        style={{
          backgroundColor: caseSensitive ? 'var(--accent-color)' : 'var(--bg-primary)',
          borderColor: caseSensitive ? 'var(--accent-color)' : 'var(--border-color)',
          color: caseSensitive ? 'white' : 'var(--text-primary)',
        }}
        title="Case Sensitive (Alt+C)"
      >
        Aa
      </button>

      <button
        id="btn-find-next"
        onClick={findNext}
        className="px-3 py-1 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title="Find Next (Enter)"
      >
        Find
      </button>

      <div className="w-px h-6 mx-1" style={{ backgroundColor: 'var(--border-color)' }} />

      <div className="flex items-center gap-1">
        <input
          type="text"
          id="replace-input"
          value={replaceQuery}
          onChange={handleReplaceChange}
          placeholder="Replace..."
          className="px-2 py-1 text-sm border rounded outline-none w-48"
          style={{
            backgroundColor: 'var(--bg-primary)',
            borderColor: 'var(--border-color)',
            color: 'var(--text-primary)',
          }}
        />
      </div>

      <button
        id="btn-replace"
        onClick={replaceMatch}
        className="px-3 py-1 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title="Replace (Ctrl+Enter)"
      >
        Replace
      </button>

      <button
        id="btn-replace-all"
        onClick={replaceAll}
        className="px-3 py-1 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title="Replace All (Ctrl+Shift+Enter)"
      >
        All
      </button>

      <div className="flex-1" />

      <button
        id="btn-close-search"
        onClick={onClose}
        className="px-2 py-1 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title="Close (Escape)"
      >
        ×
      </button>
    </div>
  );
}