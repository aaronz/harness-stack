import { useRef, useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';

export default function Editor() {
  const { currentDocument, updateContent, setInsertTextCallback } = useDocument();
  const { settings } = useSettings();
  const editorRef = useRef(null);
  const [cursorOffset, setCursorOffset] = useState(0);
  const renderTimeoutRef = useRef(null);

  // Search state management
  const [searchQuery, setSearchQuery] = useState('');
  const [replaceQuery, setReplaceQuery] = useState('');
  const [caseSensitive, setCaseSensitive] = useState(false);
  const [currentMatchIndex, setCurrentMatchIndex] = useState(-1);
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [isReplaceOpen, setIsReplaceOpen] = useState(false);

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

    if (modifier && e.key === 'f') {
      e.preventDefault();
      setIsSearchOpen(true);
      setIsReplaceOpen(false);
      return;
    }

    if (modifier && e.key === 'h') {
      e.preventDefault();
      setIsSearchOpen(true);
      setIsReplaceOpen(true);
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