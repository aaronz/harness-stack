import { useRef, useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';
import { useSearch } from '../contexts/SearchContext';

export default function Editor() {
  const { currentDocument, updateContent, setInsertTextCallback } = useDocument();
  const { settings } = useSettings();
  const { isSearchVisible, searchQuery, currentMatch, matchCount } = useSearch();
  const editorRef = useRef(null);
  const [cursorOffset, setCursorOffset] = useState(0);
  const renderTimeoutRef = useRef(null);
  const lastHighlightedQueryRef = useRef('');

  useEffect(() => {
    if (editorRef.current && currentDocument?.content !== undefined) {
      const currentContent = editorRef.current.textContent;
      if (currentContent !== currentDocument.content) {
        editorRef.current.textContent = currentDocument.content || '';
      }
    }
  }, [currentDocument?.content]);

  const handleInput = useCallback(() => {
    if (!editorRef.current) return;
    const content = editorRef.current.textContent || '';
    updateContent(content);
    debouncedRender();
  }, [updateContent]);

  const debouncedRender = useCallback(() => {
    if (renderTimeoutRef.current) {
      clearTimeout(renderTimeoutRef.current);
    }
    renderTimeoutRef.current = setTimeout(() => {
      render();
    }, 50);
  }, []);

  async function render() {
    if (!editorRef.current) return;
    const content = editorRef.current.textContent || '';

    try {
      const result = await invoke('render_for_editor_with_highlighting', {
        markdown: content,
        cursorOffset,
        includeHighlighting: true,
      });

      if (result && result.html) {
        editorRef.current.innerHTML = wrapWithDecorations(result.html);
      }
    } catch (e) {
      console.error('Render error:', e);
    }
  }

  function wrapWithDecorations(html) {
    let decorated = html;

    decorated = decorated.replace(/\*\*(.+?)\*\*/g, '<span class="md-bold" data-md="**">**$1**</span>');
    decorated = decorated.replace(/__(.+?)__/g, '<span class="md-bold" data-md="__">__$1__</span>');
    decorated = decorated.replace(/\*(.+?)\*/g, '<span class="md-italic" data-md="*">*$1*</span>');
    decorated = decorated.replace(/_(.+?)_/g, '<span class="md-italic" data-md="_">_$1_</span>');
    decorated = decorated.replace(/`(.+?)`/g, '<span class="md-code" data-md="`">`$1`</span>');
    decorated = decorated.replace(/^### (.+)$/gm, '<span class="md-heading md-heading-3" data-md="### ">### $1</span>');
    decorated = decorated.replace(/^## (.+)$/gm, '<span class="md-heading md-heading-2" data-md="## ">## $1</span>');
    decorated = decorated.replace(/^# (.+)$/gm, '<span class="md-heading md-heading-1" data-md="# "># $1</span>');
    decorated = decorated.replace(/^> (.+)$/gm, '<span class="md-blockquote" data-md="> ">> $1</span>');
    decorated = decorated.replace(/^- \[ \] (.+)$/gm, '<span class="md-task-list-item md-task-unchecked" data-md="- [ ] ">- [ ] $1</span>');
    decorated = decorated.replace(/^- \[x\] (.+)$/gi, '<span class="md-task-list-item md-task-checked" data-md="- [x] ">- [x] $1</span>');
    decorated = decorated.replace(/^- (.+)$/gm, '<span class="md-list-item" data-md="- ">- $1</span>');

    return decorated;
  }

  async function handleKeyDown(e) {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const modifier = isMac ? e.metaKey : e.ctrlKey;

    if (modifier && e.key === 's') {
      e.preventDefault();
      const { saveDocument } = useDocument();
      saveDocument();
      return;
    }

    if (modifier && e.key === 'b') {
      e.preventDefault();
      wrapSelection('**', '**');
      return;
    }

    if (modifier && e.key === 'i') {
      e.preventDefault();
      wrapSelection('*', '*');
      return;
    }

    if (e.key === 'Enter' && !modifier) {
      await handleEnter(e);
    } else if (e.key === 'Backspace') {
      await handleBackspace(e);
    } else if (e.key === 'Tab') {
      e.preventDefault();
      handleTab(e.shiftKey);
    }
  }

  async function handleEnter(e) {
    const content = editorRef.current?.textContent || '';
    const offset = getCursorOffset();

    try {
      const result = await invoke('editor_apply_transform', {
        transform: { Enter: null },
        content,
        cursorOffset: offset,
      });

      if (result && result.content !== content) {
        e.preventDefault();
        if (editorRef.current) {
          editorRef.current.textContent = result.content;
          setCursorOffset(result.cursor_offset);
          updateContent(result.content);
        }
      }
    } catch (err) {
      console.error('Transform error:', err);
    }
  }

  async function handleBackspace(e) {
    const content = editorRef.current?.textContent || '';
    const offset = getCursorOffset();

    if (offset === 0) return;

    try {
      const result = await invoke('editor_apply_transform', {
        transform: { Backspace: null },
        content,
        cursorOffset: offset,
      });

      if (result && result.content !== content) {
        e.preventDefault();
        if (editorRef.current) {
          editorRef.current.textContent = result.content;
          setCursorOffset(result.cursor_offset);
          updateContent(result.content);
        }
      }
    } catch (err) {
      console.error('Backspace error:', err);
    }
  }

  function handleTab(shiftKey) {
    const content = editorRef.current?.textContent || '';
    const offset = getCursorOffset();

    invoke('editor_apply_transform', {
      transform: shiftKey ? { ShiftTab: null } : { Tab: null },
      content,
      cursorOffset: offset,
    }).then(result => {
      if (result && result.content !== content && editorRef.current) {
        editorRef.current.textContent = result.content;
        setCursorOffset(result.cursor_offset);
        updateContent(result.content);
      }
    }).catch(err => {
      console.error('Tab error:', err);
      insertText('    ');
    });
  }

  function getCursorOffset() {
    const selection = window.getSelection();
    if (!selection.rangeCount || !editorRef.current) return 0;

    const range = selection.getRangeAt(0);
    const preCaretRange = range.cloneRange();
    preCaretRange.selectNodeContents(editorRef.current);
    preCaretRange.setEnd(range.startContainer, range.startOffset);
    return preCaretRange.toString().length;
  }

  function wrapSelection(before, after) {
    const selection = window.getSelection();
    if (!selection.rangeCount) return;

    const range = selection.getRangeAt(0);
    const selectedText = range.toString();
    if (!selectedText) return;

    const wrapper = document.createElement('span');
    wrapper.className = `md-${before === '**' ? 'bold' : 'italic'}`;
    wrapper.setAttribute('data-md', before);
    wrapper.textContent = selectedText;

    range.deleteContents();
    range.insertNode(wrapper);
    range.setStartAfter(wrapper);
    range.collapse(true);
    selection.removeAllRanges();
    selection.addRange(range);

    if (editorRef.current) {
      updateContent(editorRef.current.textContent);
      debouncedRender();
    }
  }

  function insertText(text) {
    const selection = window.getSelection();
    if (!selection.rangeCount) return;

    const range = selection.getRangeAt(0);
    range.deleteContents();

    const textNode = document.createTextNode(text);
    range.insertNode(textNode);
    range.setStartAfter(textNode);
    range.collapse(true);
    selection.removeAllRanges();
    selection.addRange(range);

    if (editorRef.current) {
      updateContent(editorRef.current.textContent);
      debouncedRender();
    }
  }

  useEffect(() => {
    setInsertTextCallback(insertText);
  }, [setInsertTextCallback, insertText]);

  useEffect(() => {
    return () => {
      if (renderTimeoutRef.current) {
        clearTimeout(renderTimeoutRef.current);
      }
    };
  }, []);

  const escapeRegex = (string) => string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

  function applyHighlights() {
    if (!editorRef.current) return;
    const editor = editorRef.current;

    if (!searchQuery || matchCount === 0) {
      clearHighlights();
      return;
    }

    if (searchQuery === lastHighlightedQueryRef.current) {
      updateCurrentMatchHighlight();
      return;
    }

    lastHighlightedQueryRef.current = searchQuery;

    const content = currentDocument?.content || '';
    try {
      const regex = new RegExp(escapeRegex(searchQuery), 'gi');
      const span = document.createElement('span');

      let matchIndex = 0;
      let match;
      const walker = document.createTreeWalker(editor, NodeFilter.SHOW_TEXT, null, false);
      const textNodes = [];
      let node;
      while (node = walker.nextNode()) {
        if (!node.parentElement?.closest('.current-match, .match')) {
          textNodes.push(node);
        }
      }

      for (const textNode of textNodes) {
        const text = textNode.textContent || '';
        regex.lastIndex = 0;

        while ((match = regex.exec(text)) !== null) {
          matchIndex++;
          const isCurrentMatch = matchIndex === currentMatch;

          const mark = document.createElement('mark');
          mark.className = isCurrentMatch ? 'current-match' : 'match';
          mark.setAttribute('data-match-index', matchIndex.toString());
          mark.textContent = match[0];

          const range = document.createRange();
          range.setStart(textNode, match.index);
          range.setEnd(textNode, match.index + match[0].length);
          range.deleteContents();
          range.insertNode(mark);
        }
      }

      updateCurrentMatchHighlight();
    } catch (e) {
      console.error('Highlight error:', e);
    }
  }

  function updateCurrentMatchHighlight() {
    if (!editorRef.current) return;
    const editor = editorRef.current;

    const allMarks = editor.querySelectorAll('mark');
    allMarks.forEach(mark => {
      const idx = parseInt(mark.getAttribute('data-match-index') || '0', 10);
      if (idx === currentMatch) {
        mark.className = 'current-match';
      } else {
        mark.className = 'match';
      }
    });

    const currentMark = editor.querySelector(`mark[data-match-index="${currentMatch}"]`);
    if (currentMark) {
      currentMark.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  }

  function clearHighlights() {
    if (!editorRef.current) return;
    const editor = editorRef.current;
    const marks = editor.querySelectorAll('mark');
    marks.forEach(mark => {
      const text = document.createTextNode(mark.textContent || '');
      mark.parentNode?.replaceChild(text, mark);
    });
    lastHighlightedQueryRef.current = '';
  }

  useEffect(() => {
    if (isSearchVisible && searchQuery && matchCount > 0) {
      applyHighlights();
    } else if (!isSearchVisible) {
      clearHighlights();
    }
  }, [isSearchVisible, searchQuery, currentMatch, matchCount]);

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div
        id="editor-content"
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        onInput={handleInput}
        onKeyDown={handleKeyDown}
        className={`
          flex-1 p-5 font-sans text-base leading-relaxed overflow-y-auto whitespace-pre-wrap break-word outline-none
          ${settings.focusMode ? 'focus-mode' : ''}
          ${settings.typewriterMode ? 'typewriter-mode' : ''}
        `}
        style={{
          backgroundColor: 'var(--bg-primary)',
          color: 'var(--text-primary)',
          caretColor: 'var(--accent-color)',
        }}
      />
    </div>
  );
}