import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';
import { marked } from 'marked';
import { useEffect, useRef } from 'react';
import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';

marked.setOptions({
  breaks: true,
  gfm: true,
});

export default function TipTapEditor() {
  const { currentDocument, updateContent, saveDocument } = useDocument();
  const { settings } = useSettings();
  const updateTimeoutRef = useRef(null);
  const isInternalUpdateRef = useRef(false);

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
        codeBlock: {
          HTMLAttributes: {
            class: 'p-4 bg-gray-100 dark:bg-gray-800 rounded-lg font-mono text-sm overflow-x-auto',
          },
        },
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
        const markdown = marked(html);
        updateContent(markdown);
      }, 150);
    },
  });

  useEffect(() => {
    if (editor && currentDocument?.content !== undefined) {
      const currentHtml = editor.getHTML();
      if (currentDocument.content !== undefined) {
        let parsedMarkdown = '';
        if (currentDocument.content && currentDocument.content.trim()) {
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

  return (
    <div 
      className="flex-1 flex flex-col overflow-hidden"
      style={{
        backgroundColor: 'var(--bg-primary)',
        color: 'var(--text-primary)',
      }}
    >
      <EditorContent editor={editor} />
    </div>
  );
}