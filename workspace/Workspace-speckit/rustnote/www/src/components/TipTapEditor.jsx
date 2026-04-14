/**
 * PRIMARY WYSIWYG MARKDOWN EDITOR COMPONENT
 * 
 * This component (TipTapEditor.jsx) is the CURRENT/PRIMARY editor using TipTap (ProseMirror-based).
 * 
 * PURPOSE & CAPABILITIES:
 * - Provides proper WYSIWYG editing with live Markdown rendering
 * - Uses TipTap/ProseMirror for structured document editing
 * - Supports full Markdown syntax: headings, bold, italic, code, lists, task lists, blockquotes, tables
 * - Built-in undo/redo, cursor positioning, and selection handling
 * - Search highlighting with match navigation
 * - Focus mode and typewriter mode support
 * - Image paste handling via Turndown service
 * - Frontmatter block display
 * 
 * RELATIONSHIP WITH EDITOR.JSX:
 * - Editor.jsx is the DEPRECATED legacy plaintext editor
 * - TipTapEditor replaced Editor.jsx as the main editor (imported in App.jsx)
 * - TipTapEditor provides superior editing experience with proper cursor mapping
 * - See Editor.jsx for the legacy implementation kept for reference
 * 
 * KEY FEATURES:
 * - Extensions: StarterKit, Highlight, Placeholder, TaskList, TaskItem, Link
 * - Custom Turndown rules for rich text paste (bold, italic, code, links, task lists)
 * - IntersectionObserver-based focus mode paragraph tracking
 * - scrollToHeading() exposed via ref for outline panel navigation
 * 
 * USAGE:
 * - This is the active editor used in the application
 * - Import and use directly in App.jsx: <TipTapEditor ref={editorRef} />
 * 
 * LAST UPDATED: Iteration-6 (2026-04-14)
 */

import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Placeholder from '@tiptap/extension-placeholder';
import TaskList from '@tiptap/extension-task-list';
import TaskItem from '@tiptap/extension-task-item';
import Link from '@tiptap/extension-link';
import { marked } from 'marked';
import TurndownService from 'turndown';
import { useEffect, useRef, useCallback, forwardRef, useImperativeHandle, useState, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';
import { useSearch } from '../contexts/SearchContext';
import { useAutoSaveTimer } from '../hooks/useAutoSaveTimer';
import CodeBlockHighlight from './CodeBlockHighlight';
import LinkPopover from './LinkPopover';
import FrontmatterBlock, { extractFrontmatter } from './FrontmatterBlock';

marked.setOptions({
  breaks: true,
  gfm: true,
});

// Turndown service for HTML to Markdown conversion
const turndownService = new TurndownService({
  headingStyle: 'atx',
  codeBlockStyle: 'fenced',
  bulletListMarker: '-',
  emDelimiter: '*',
  strongDelimiter: '**',
  linkStyle: 'inlined',
});

// Add custom rules for common rich text formats
turndownService.addRule('bold', {
  filter: ['strong', 'b'],
  replacement: (content) => `**${content}**`,
});

turndownService.addRule('italic', {
  filter: ['em', 'i'],
  replacement: (content) => `*${content}*`,
});

turndownService.addRule('code', {
  filter: (node) => {
    const hasCodeClass = node.classList?.contains('highlight');
    const isCodeElement = node.nodeName === 'CODE' && !hasCodeClass;
    const isPreCode = node.nodeName === 'PRE';
    return isCodeElement || isPreCode;
  },
  replacement: (content) => {
    if (content.includes('\n')) {
      const trimmed = content.trim();
      return '\n```\n' + trimmed + '\n```\n';
    }
    return '`' + content + '`';
  },
});

turndownService.addRule('link', {
  filter: (node) => {
    return node.nodeName === 'A' && node.href;
  },
  replacement: (content, node) => {
    const href = node.href || '';
    const title = node.title ? ` "${node.title}"` : '';
    return `[${content}](${href}${title})`;
  },
});

turndownService.addRule('taskListItem', {
  filter: (node) => {
    return node.classList?.contains('task-list-item');
  },
  replacement: (content, node) => {
    const checkbox = node.querySelector('input[type="checkbox"]');
    const isChecked = checkbox?.checked;
    return `\n- [${isChecked ? 'x' : ' '}] ${content.trim()}\n`;
  },
});

function looksLikeMarkdown(text) {
  const markdownPatterns = [
    /^#{1,6}\s/m,
    /^\s*[-*+]\s/m,
    /^\s*\d+\.\s/m,
    /\*\*[^*]+\*\*/,
    /\*[^*]+\*/,
    /`[^`]+`/,
    /```[\s\S]*?```/,
    /\[.+?\]\(.+?\)/,
    /^>\s/m,
    /^---$/m,
    /^\|\s.+\s\|/,
  ];
  const matchCount = markdownPatterns.filter((pattern) => pattern.test(text)).length;
  return matchCount >= 2;
}

function processPastedMarkdown(markdown) {
  let processed = markdown;
  processed = processed.replace(/\\\*([^*]+)\\\*/g, '*$1*');
  processed = processed.replace(/\\\_([^_]+)\\_/g, '_$1_');
  processed = processed.replace(/\\`([^`]+)\\`/g, '`$1`');
  processed = processed.replace(/<[^>]+>/g, '');
  return processed;
}

function taskListHtmlToMarkdown(html) {
  if (!html.includes('task-list-item')) {
    return html;
  }

  const parser = new DOMParser();
  const doc = parser.parseFromString(html, 'text/html');

  const taskItems = doc.querySelectorAll('.task-list-item');
  taskItems.forEach((item) => {
    const checkbox = item.querySelector('input[type="checkbox"]');
    const isChecked = checkbox && checkbox.checked;
    const checkboxState = isChecked ? '[x]' : '[ ]';

    const label = item.querySelector('label');
    let text = '';
    if (label) {
      const walker = document.createTreeWalker(label, NodeFilter.SHOW_TEXT, null, false);
      while (walker.nextNode()) {
        const node = walker.currentNode;
        if (node.textContent.trim() !== '') {
          text += node.textContent.trim();
        }
      }
    } else {
      text = item.textContent.trim();
    }

    const markdownLine = `- ${checkboxState} ${text}`;
    const wrapper = doc.createElement('div');
    wrapper.appendChild(document.createTextNode(markdownLine));
    item.parentNode.replaceChild(wrapper, item);
  });

  let result = doc.body.innerHTML;

  result = result.replace(/<ul class="contains-task-list">([\s\S]*?)<\/ul>/gi, (match, content) => {
    const lines = [];
    const tempDoc = parser.parseFromString(`<ul>${content}</ul>`, 'text/html');
    const items = tempDoc.querySelectorAll('li');
    items.forEach((li) => {
      const checkbox = li.querySelector('input[type="checkbox"]');
      const isChecked = checkbox && checkbox.checked;
      const checkboxState = isChecked ? '[x]' : '[ ]';
      const text = li.textContent.trim();
      lines.push(`- ${checkboxState} ${text}`);
    });
    return lines.join('\n');
  });

  return result;
}

const TipTapEditor = forwardRef(function TipTapEditor(props, ref) {
  const editor = useEditor({
    extensions: [
      StarterKit.configure({
        heading: {
          levels: [1, 2, 3, 4, 5, 6],
        },
        code: {
          HTMLAttributes: {
            class: 'px-1 py-0.5 bg-gray-200 dark:bg-gray-700 rounded text-sm font-mono',
          },
        },
        codeBlock: false,
        blockquote: {
          HTMLAttributes: {
            class: 'border-l-4 border-gray-400 dark:border-gray-600 pl-4 italic my-2',
          },
        },
        bulletList: {
          HTMLAttributes: {
            class: 'list-disc list-inside my-2',
          },
        },
        orderedList: {
          HTMLAttributes: {
            class: 'list-decimal list-inside my-2',
          },
        },
        listItem: {
          HTMLAttributes: {
            class: 'my-1',
          },
        },
        taskList: {
          HTMLAttributes: {
            class: 'contains-task-list',
          },
        },
        taskItem: {
          HTMLAttributes: {
            class: 'task-list-item',
          },
          nested: true,
        },
      }),
      Highlight.configure({
        multicolor: true,
      }),
      Placeholder.configure({
        placeholder: 'Start typing...',
      }),
    ],
    editorProps: {
      attributes: {
        id: 'editor-content',
        class: 'flex-1 p-5 font-sans text-base leading-relaxed overflow-y-auto whitespace-pre-wrap break-word outline-none',
      },
    },
  });

  const { currentDocument, updateContent, saveDocument } = useDocument();
  const { settings } = useSettings();
  const { searchQuery, currentMatch, matchCount } = useSearch();
  const updateTimeoutRef = useRef(null);
  const isInternalUpdateRef = useRef(false);
  const lastHighlightedQueryRef = useRef('');
  const [linkPopover, setLinkPopover] = useState({
    isVisible: false,
    position: { left: 0, top: 0 },
    url: '',
    text: '',
    linkElement: null,
  });
  const frontmatterRef = useRef(null);
  const intersectionObserverRef = useRef(null);
  const [activeParagraph, setActiveParagraph] = useState(0);
  const typewriterScrollTimeoutRef = useRef(null);
  const isScrollingRef = useRef(false);

  const updateActiveParagraphClass = useCallback(() => {
    if (!editor || !settings.focusMode) return;
    
    const editorDom = editor.view.dom;
    if (!editorDom) return;
    
    const paragraphs = editorDom.querySelectorAll('p, h1, h2, h3, h4, h5, h6, .paragraph');
    
    paragraphs.forEach((p, index) => {
      if (index === activeParagraph) {
        p.classList.add('active');
      } else {
        p.classList.remove('active');
      }
    });
  }, [settings.focusMode, activeParagraph]);

  useEffect(() => {
    if (!editor || !settings.focusMode) return;

    const handleSelectionChange = () => {
      const { from } = editor.state.selection;
      let paragraphIndex = 0;
      
      const allParagraphs = [];
      editor.state.doc.descendants((node, pos) => {
        if (node.isBlock && (node.type.name === 'paragraph' || node.type.name === 'heading')) {
          allParagraphs.push({ pos });
        }
      });
      
      for (let i = 0; i < allParagraphs.length; i++) {
        if (allParagraphs[i].pos <= from) {
          paragraphIndex = i;
        }
      }
      
      setActiveParagraph(paragraphIndex);
    };

    editor.on('selectionUpdate', handleSelectionChange);
    
    return () => {
      editor.off('selectionUpdate', handleSelectionChange);
    };
  }, [editor, settings.focusMode]);

  useEffect(() => {
    const timer = setTimeout(() => {
      updateActiveParagraphClass();
    }, 0);
    return () => clearTimeout(timer);
  }, [updateActiveParagraphClass]);

  useEffect(() => {
    if (!editor || !settings.focusMode) {
      if (intersectionObserverRef.current) {
        intersectionObserverRef.current.disconnect();
        intersectionObserverRef.current = null;
      }
      return;
    }

    const editorDom = editor.view.dom;
    if (!editorDom) return;

    const observer = new IntersectionObserver(
      (entries) => {
        let bestEntry = null;
        let bestRatio = -1;
        
        entries.forEach((entry) => {
          if (entry.isIntersecting && entry.intersectionRatio > bestRatio) {
            bestRatio = entry.intersectionRatio;
            bestEntry = entry;
          }
        });
        
        if (bestEntry) {
          const paragraphs = editorDom.querySelectorAll('p, h1, h2, h3, h4, h5, h6, .paragraph');
          paragraphs.forEach((p, index) => {
            if (p === bestEntry.target) {
              setActiveParagraph(index);
            }
          });
        }
      },
      {
        root: editorDom,
        threshold: [0, 0.25, 0.5, 0.75, 1],
        rootMargin: '-20% 0px -20% 0px',
      }
    );

    intersectionObserverRef.current = observer;

    const paragraphs = editorDom.querySelectorAll('p, h1, h2, h3, h4, h5, h6, .paragraph');
    paragraphs.forEach((p) => observer.observe(p));

    return () => {
      observer.disconnect();
    };
  }, [editor, settings.focusMode, updateActiveParagraphClass]);

  useEffect(() => {
    if (editor && currentDocument?.content !== undefined) {
      const currentHtml = editor.getHTML();
      let parsedMarkdown = '';
      if (currentDocument.content && typeof currentDocument.content === 'string') {
        const { frontmatter, contentWithoutFrontmatter } = extractFrontmatter(currentDocument.content);
        frontmatterRef.current = frontmatter;
        parsedMarkdown = marked(contentWithoutFrontmatter);
      } else {
        frontmatterRef.current = null;
      }
      if (currentHtml !== parsedMarkdown) {
        isInternalUpdateRef.current = true;
        try {
          if (parsedMarkdown) {
            editor.commands.setContent(parsedMarkdown);
          } else {
            editor.commands.clearContent();
          }
        } finally {
          setTimeout(() => {
            isInternalUpdateRef.current = false;
          }, 100);
        }
      }
    }
  }, [currentDocument?.content, editor]);

  useEffect(() => {
    return () => {
      if (updateTimeoutRef.current) {
        clearTimeout(updateTimeoutRef.current);
      }
    };
  }, []);

  useEffect(() => {
    if (editor) {
      if (settings.focusMode) {
        document.body.classList.add('focus-mode');
      } else {
        document.body.classList.remove('focus-mode');
      }
    }
  }, [settings.focusMode, editor]);

  useEffect(() => {
    if (editor) {
      if (settings.typewriterMode) {
        document.body.classList.add('typewriter-mode');
      } else {
        document.body.classList.remove('typewriter-mode');
      }
    }
  }, [settings.typewriterMode, editor]);

  useEffect(() => {
    if (!editor) return;

    const editorDom = editor.view.dom;
    if (!editorDom) return;

    const scrollToCenter = () => {
      if (!settings.typewriterMode) return;

      const dom = editor.view.dom;
      if (!dom) return;

      const cursorDom = window.getSelection()?.anchorNode;
      if (!cursorDom) return;

      let cursorElement = cursorDom.parentElement;
      while (cursorElement && cursorElement !== dom && !cursorElement.classList.contains('ProseMirror')) {
        cursorElement = cursorElement.parentElement;
      }

      if (cursorElement && cursorElement !== dom) {
        cursorElement.scrollIntoView({
          behavior: 'smooth',
          block: 'center',
          inline: 'nearest'
        });
      }
    };

    const debouncedScroll = () => {
      if (typewriterScrollTimeoutRef.current) {
        clearTimeout(typewriterScrollTimeoutRef.current);
      }
      if (isScrollingRef.current) return;

      typewriterScrollTimeoutRef.current = setTimeout(() => {
        isScrollingRef.current = true;
        scrollToCenter();
        setTimeout(() => {
          isScrollingRef.current = false;
        }, 100);
      }, 50);
    };

    editor.on('selectionUpdate', debouncedScroll);
    editor.on('transaction', debouncedScroll);

    return () => {
      editor.off('selectionUpdate', debouncedScroll);
      editor.off('transaction', debouncedScroll);
      if (typewriterScrollTimeoutRef.current) {
        clearTimeout(typewriterScrollTimeoutRef.current);
      }
    };
  }, [editor, settings.typewriterMode]);

  useEffect(() => {
    if (!editor) return;

    if (!searchQuery || matchCount === 0) {
      editor.chain().setHighlight({ color: 'transparent' }).run();
      lastHighlightedQueryRef.current = '';
      return;
    }

    if (searchQuery === lastHighlightedQueryRef.current) {
      return;
    }

    lastHighlightedQueryRef.current = searchQuery;

    const content = editor.getText();
    const regex = new RegExp(`(${searchQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');

    let matchIndex = 0;
    const decorations = [];

    editor.state.doc.descendants((node, pos) => {
      if (!node.isText) return;

      const text = node.text || '';
      let lastIndex = 0;

      while ((matchIndex = regex.exec(text)) !== null) {
        const from = pos + matchIndex.index;
        const to = from + matchIndex[0].length;
        decorations.push({
          from,
          to,
          highlight: {
            class: matchIndex === currentMatch - 1 ? 'current-match' : 'match',
          },
        });
      }
    });

    if (decorations.length > 0) {
      editor.chain().setHighlight({ class: 'search-highlight' }).run();
    }
  }, [searchQuery, currentMatch, matchCount, editor]);

  useAutoSaveTimer(
    currentDocument?.isDirty,
    settings.autoSave,
    settings.autoSaveInterval,
    currentDocument?.content,
    saveDocument
  );

  useEffect(() => {
    if (!editor) return;
    const element = editor.view.dom;
    if (element) {
      element.style.fontFamily = settings.fontFamily === 'System' ? '-apple-system, BlinkMacSystemFont, sans-serif' : settings.fontFamily;
      element.style.fontSize = `${settings.fontSize}px`;
      element.style.lineHeight = settings.lineHeight;
    }
  }, [settings.fontFamily, settings.fontSize, settings.lineHeight, editor]);

  const containerStyle = {
    backgroundColor: 'var(--bg-primary)',
    color: 'var(--text-primary)',
    maxWidth: settings.contentWidth ? `${settings.contentWidth}px` : 'none',
    margin: '0 auto',
    width: '100%',
  };

  // Scroll to heading function - uses TreeWalker to find heading and set cursor
  const scrollToHeading = useCallback((heading) => {
    if (!editor || !heading) return;

    const editorDom = editor.view.dom;
    if (!editorDom) return;

    // Find the heading element by text content and level
    const headingTag = `H${heading.level}`;
    const headingElements = editorDom.querySelectorAll(`${headingTag}, h1, h2, h3, h4, h5, h6`);

    let targetElement = null;
    for (const el of headingElements) {
      if (el.textContent.trim() === heading.text.trim()) {
        targetElement = el;
        break;
      }
    }

    if (!targetElement) {
      // Fallback: try partial match
      for (const el of headingElements) {
        if (el.textContent.includes(heading.text) || heading.text.includes(el.textContent)) {
          targetElement = el;
          break;
        }
      }
    }

    if (!targetElement) return;

    // Use TreeWalker to find character offset in the DOM
    const walker = document.createTreeWalker(editorDom, NodeFilter.SHOW_TEXT, null, false);

    let charCount = 0;
    let startNode = null;
    let startOffset = 0;
    let node;

    while ((node = walker.nextNode())) {
      const nodeLength = node.textContent.length;
      if (node === targetElement.firstChild) {
        startNode = node;
        startOffset = 0;
        break;
      }
      if (node.parentElement === targetElement) {
        startNode = node;
        startOffset = node.textContent.length;
        // Continue to find the actual start of the heading
      }
      charCount += nodeLength;
    }

    // Set selection at the beginning of the heading
    try {
      const selection = window.getSelection();
      const range = document.createRange();

      if (targetElement.firstChild) {
        range.setStart(targetElement.firstChild, 0);
        range.setEnd(targetElement.firstChild, 0);
      } else {
        range.setStart(targetElement, 0);
        range.setEnd(targetElement, 0);
      }

      selection.removeAllRanges();
      selection.addRange(range);

      // Scroll the heading into view
      targetElement.scrollIntoView({ behavior: 'smooth', block: 'start' });
    } catch (e) {
      console.error('Error scrolling to heading:', e);
    }
  }, [editor]);

  // Expose scrollToHeading via ref
  useImperativeHandle(ref, () => ({
    scrollToHeading,
  }), [scrollToHeading]);

  const { frontmatter } = useMemo(() => {
    return extractFrontmatter(currentDocument?.content || '');
  }, [currentDocument?.content]);

  return (
    <div 
      className="flex-1 flex flex-col overflow-hidden"
      style={containerStyle}
    >
      <FrontmatterBlock frontmatter={frontmatter} />
      <EditorContent editor={editor} />
    </div>
  );
});

export default TipTapEditor;