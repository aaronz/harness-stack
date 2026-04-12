import { useRef, useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import TurndownService from 'turndown';
import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';
import { useSearch } from '../contexts/SearchContext';
import { useAutoSaveTimer } from '../hooks/useAutoSaveTimer';

const turndownService = new TurndownService({
  headingStyle: 'atx',
  codeBlockStyle: 'fenced',
  bulletListMarker: '-',
  emDelimiter: '*',
  strongDelimiter: '**',
  linkStyle: 'inlined',
});

export default function Editor() {
  const { currentDocument, updateContent, setInsertTextCallback, saveDocument } = useDocument();
  const { settings, toggleFocusMode, toggleTypewriterMode } = useSettings();
  const { isSearchVisible, searchQuery, currentMatch, matchCount } = useSearch();
  const editorRef = useRef(null);
  const [cursorOffset, setCursorOffset] = useState(0);
  const renderTimeoutRef = useRef(null);
  const lastHighlightedQueryRef = useRef('');
  const activeBlockRef = useRef(null);
  const typewriterScrollRef = useRef(null);

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

    const needsHighlighting = /[*_`#>\-\[\]]/.test(content);
    
    if (!needsHighlighting) {
      return;
    }

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

    if (modifier && e.shiftKey && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      toggleFocusMode();
      return;
    }

    if (modifier && e.shiftKey && e.key.toLowerCase() === 't') {
      e.preventDefault();
      toggleTypewriterMode();
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

  function handlePaste(e) {
    e.preventDefault();
    const clipboardData = e.clipboardData;
    
    const types = clipboardData?.types || [];
    const hasHtml = types.includes('text/html');
    
    if (hasHtml) {
      const htmlContent = clipboardData?.getData('text/html') || '';
      
      // FR-018-B1.3: HTML to Markdown conversion
      if (htmlContent && htmlContent.trim()) {
        try {
          // Sanitize: remove script tags and event handlers before conversion
          const sanitizedHtml = htmlContent
            .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '')
            .replace(/\s*on\w+\s*=\s*["'][^"']*["']/gi, '')
            .replace(/\s*on\w+\s*=\s*[^\s>]+/gi, '');
          
          // Convert HTML to Markdown
          const markdown = turndownService.turndown(sanitizedHtml);
          
          if (markdown && markdown.trim()) {
            insertText(markdown);
          } else {
            // Fallback to plain text if conversion yields empty
            const textContent = clipboardData?.getData('text/plain') || '';
            if (textContent) {
              insertText(textContent);
            }
          }
        } catch (err) {
          console.error('HTML to Markdown conversion error:', err);
          // Fallback to plain text on malformed HTML
          const textContent = clipboardData?.getData('text/plain') || '';
          if (textContent) {
            insertText(textContent);
          }
        }
      } else {
        // Empty HTML content - fallback to plain text
        const textContent = clipboardData?.getData('text/plain') || '';
        if (textContent) {
          insertText(textContent);
        }
      }
    } else {
      // Plain text fallback - no HTML available
      const text = clipboardData?.getData('text/plain') || '';
      if (text) {
        insertText(text);
      }
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

  function findNearestBlockElement(offset) {
    if (!editorRef.current) return null;
    const editor = editorRef.current;
    const walker = document.createTreeWalker(editor, NodeFilter.SHOW_TEXT, null, false);
    let currentOffset = 0;
    let node;
    let targetNode = null;

    while (node = walker.nextNode()) {
      const nodeLength = node.textContent?.length || 0;
      if (currentOffset + nodeLength >= offset) {
        targetNode = node;
        break;
      }
      currentOffset += nodeLength;
    }

    if (!targetNode) {
      return editor.querySelector('p, h1, h2, h3, h4, h5, h6, blockquote, pre, li');
    }

    let element = targetNode.parentElement;
    const blockTags = ['P', 'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 'BLOCKQUOTE', 'PRE', 'LI', 'DIV'];
    while (element && element !== editor) {
      if (blockTags.includes(element.tagName)) {
        return element;
      }
      element = element.parentElement;
    }

    return editor.querySelector('p, h1, h2, h3, h4, h5, h6, blockquote, pre, li');
  }

  function updateActiveBlock() {
    if (!settings.focusMode || !editorRef.current) return;

    const currentOffset = getCursorOffset();
    const newActiveBlock = findNearestBlockElement(currentOffset);

    if (activeBlockRef.current && activeBlockRef.current !== newActiveBlock) {
      activeBlockRef.current.classList.remove('active');
    }

    if (newActiveBlock) {
      newActiveBlock.classList.add('active');
      activeBlockRef.current = newActiveBlock;
    }
  }

  useEffect(() => {
    if (isSearchVisible && searchQuery && matchCount > 0) {
      applyHighlights();
    } else if (!isSearchVisible) {
      clearHighlights();
    }
  }, [isSearchVisible, searchQuery, currentMatch, matchCount]);

  useEffect(() => {
    if (settings.focusMode) {
      document.body.classList.add('focus-mode');
    } else {
      document.body.classList.remove('focus-mode');
    }

    return () => {
      document.body.classList.remove('focus-mode');
    };
  }, [settings.focusMode]);

  useEffect(() => {
    if (settings.typewriterMode) {
      document.body.classList.add('typewriter-mode');
    } else {
      document.body.classList.remove('typewriter-mode');
    }

    return () => {
      document.body.classList.remove('typewriter-mode');
    };
  }, [settings.typewriterMode]);

  useEffect(() => {
    if (!settings.focusMode) return;

    document.addEventListener('selectionchange', updateActiveBlock);

    return () => {
      document.removeEventListener('selectionchange', updateActiveBlock);
    };
  }, [settings.focusMode]);

  useEffect(() => {
    if (!settings.focusMode) return;

    const editor = editorRef.current;
    if (!editor) return;

    editor.addEventListener('keyup', updateActiveBlock);
    editor.addEventListener('mouseup', updateActiveBlock);

    return () => {
      editor.removeEventListener('keyup', updateActiveBlock);
      editor.removeEventListener('mouseup', updateActiveBlock);
    };
  }, [settings.focusMode]);

  // Typewriter Mode: Scroll centering on cursor move
  useEffect(() => {
    if (!settings.typewriterMode) return;

    function scrollToCursor() {
      const selection = window.getSelection();
      if (!selection.rangeCount || !editorRef.current) return;

      const range = selection.getRangeAt(0);
      const cursorNode = range.startContainer;

      // Find the block element containing the cursor
      let element = cursorNode.parentElement;
      const blockTags = ['P', 'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 'BLOCKQUOTE', 'PRE', 'LI', 'DIV', 'SPAN'];
      while (element && element !== editorRef.current) {
        if (blockTags.includes(element.tagName)) {
          element.scrollIntoView({ block: 'center', behavior: 'smooth' });
          break;
        }
        element = element.parentElement;
      }
    }

    function debouncedScroll() {
      if (typewriterScrollRef.current) {
        clearTimeout(typewriterScrollRef.current);
      }
      typewriterScrollRef.current = setTimeout(scrollToCursor, 50);
    }

    document.addEventListener('selectionchange', debouncedScroll);

    return () => {
      document.removeEventListener('selectionchange', debouncedScroll);
      if (typewriterScrollRef.current) {
        clearTimeout(typewriterScrollRef.current);
      }
    };
  }, [settings.typewriterMode]);

  useAutoSaveTimer(
    currentDocument?.isDirty,
    settings.autoSave,
    settings.autoSaveInterval,
    currentDocument?.content,
    saveDocument
  );

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      <div
        id="editor-content"
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        onInput={handleInput}
        onKeyDown={handleKeyDown}
        onPaste={handlePaste}
        className={`
          flex-1 p-5 font-sans text-base leading-relaxed overflow-y-auto whitespace-pre-wrap break-word outline-none
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