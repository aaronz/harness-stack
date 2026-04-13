import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Highlight from '@tiptap/extension-highlight';
import Placeholder from '@tiptap/extension-placeholder';
import TaskList from '@tiptap/extension-task-list';
import TaskItem from '@tiptap/extension-task-item';
import Link from '@tiptap/extension-link';
import { marked } from 'marked';
import { useEffect, useRef, useCallback, forwardRef, useImperativeHandle, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';
import { useSearch } from '../contexts/SearchContext';
import { useAutoSaveTimer } from '../hooks/useAutoSaveTimer';
import CodeBlockHighlight from './CodeBlockHighlight';
import LinkPopover from './LinkPopover';

marked.setOptions({
  breaks: true,
  gfm: true,
});

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
      CodeBlockHighlight,
      TaskList,
      TaskItem.configure({
        HTMLAttributes: {
          class: 'task-list-item',
        },
      }),
      Highlight.configure({
        multicolor: true,
      }),
      Link.configure({
        openOnClick: false,
        HTMLAttributes: {
          class: 'cursor-pointer',
        },
      }),
      Placeholder.configure({
        placeholder: 'Start typing...',
      }),
    ],
    content: '',
    editorProps: {
      attributes: {
        id: 'editor-content',
        class: 'flex-1 p-5 font-sans text-base leading-relaxed overflow-y-auto whitespace-pre-wrap break-word outline-none',
      },
      handleKeyDown: (view, event) => {
        const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
        const modifier = isMac ? event.metaKey : event.ctrlKey;

        if (modifier && event.key === 's') {
          event.preventDefault();
          saveDocument();
          return true;
        }

        if (modifier && event.key === 'b') {
          event.preventDefault();
          editor.chain().focus().toggleBold().run();
          return true;
        }

        if (modifier && event.key === 'i') {
          event.preventDefault();
          editor.chain().focus().toggleItalic().run();
          return true;
        }

        if (modifier && event.key === 'z') {
          event.preventDefault();
          editor.chain().focus().undo().run();
          return true;
        }

        if (modifier && event.key === 'y') {
          event.preventDefault();
          editor.chain().focus().redo().run();
          return true;
        }

        return false;
      },
      handleClick: (view, event) => {
        const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
        const modifier = isMac ? event.metaKey : event.ctrlKey;

        const link = event.target.closest('a');
        if (!link) {
          if (linkPopover.isVisible) {
            setLinkPopover((prev) => ({ ...prev, isVisible: false }));
          }
          return false;
        }

        const href = link.getAttribute('href');

        if (modifier) {
          event.preventDefault();
          if (href) {
            window.open(href, '_blank', 'noopener,noreferrer');
          }
          return true;
        }

        event.preventDefault();

        const rect = link.getBoundingClientRect();
        const editorDom = editor.view.dom;
        const editorRect = editorDom.getBoundingClientRect();

        setLinkPopover({
          isVisible: true,
          position: {
            left: rect.left - editorRect.left + rect.width / 2,
            top: rect.bottom - editorRect.top + 8,
          },
          url: href || '',
          text: link.textContent || '',
          linkElement: link,
        });

        return true;
      },
      handlePaste: (view, event) => {
        const items = event.clipboardData?.items;
        if (!items) return false;

        for (const item of items) {
          if (item.type.startsWith('image/')) {
            event.preventDefault();
            const blob = item.getAsFile();
            if (blob) {
              const reader = new FileReader();
              reader.onload = async (e) => {
                const base64 = e.target.result;
                const workspacePath = currentDocument?.filePath
                  ? currentDocument.filePath.substring(0, currentDocument.filePath.lastIndexOf('/'))
                  : null;

                if (!workspacePath) {
                  console.error('Cannot paste image: document has not been saved to a workspace');
                  return;
                }

                try {
                  const imageInfo = await invoke('save_image_from_base64_cmd', {
                    base64_data: base64,
                    workspace_path: workspacePath,
                  });

                  const markdown = `![${imageInfo.file_name}](${imageInfo.relative_path})`;
                  editor.chain().focus().insertContent(markdown).run();
                } catch (err) {
                  console.error('Failed to save image:', err);
                }
              };
              reader.readAsDataURL(blob);
            }
            return true;
          }
        }
        return false;
      },
    },
    onUpdate: ({ editor }) => {
      if (isInternalUpdateRef.current) {
        return;
      }

      if (updateTimeoutRef.current) {
        clearTimeout(updateTimeoutRef.current);
      }

      updateTimeoutRef.current = setTimeout(() => {
        const html = editor.getHTML();
        const processedHtml = taskListHtmlToMarkdown(html);
        const markdown = marked(processedHtml);
        updateContent(markdown);
      }, 150);
    },
  });

  useEffect(() => {
    if (editor && currentDocument?.content !== undefined) {
      const currentHtml = editor.getHTML();
      let parsedMarkdown = '';
      if (currentDocument.content && typeof currentDocument.content === 'string') {
        parsedMarkdown = marked(currentDocument.content);
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

  return (
    <div 
      className="flex-1 flex flex-col overflow-hidden"
      style={containerStyle}
    >
      <EditorContent editor={editor} />
    </div>
  );
});

export default TipTapEditor;