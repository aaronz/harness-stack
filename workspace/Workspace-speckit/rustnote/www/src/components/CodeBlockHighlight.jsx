import { Node, mergeAttributes } from '@tiptap/core';
import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

const CodeBlockHighlightExtension = Node.create({
  name: 'codeBlock',
  group: 'block',
  content: 'text*',
  marks: '',
  defining: true,

  addAttributes() {
    return {
      language: {
        default: '',
        parseHTML: (element) => {
          const className = element.className || '';
          const langMatch = className.match(/language-(\w+)/);
          return langMatch ? langMatch[1] : '';
        },
        renderHTML: (attributes) => {
          if (!attributes.language) return {};
          return { 'data-language': attributes.language };
        },
      },
    };
  },

  parseHTML() {
    return [
      {
        tag: 'pre',
        getAttrs: (element) => {
          const codeElement = element.querySelector('code');
          const className = codeElement?.className || element.className || '';
          const langMatch = className.match(/language-(\w+)/);
          return { language: langMatch ? langMatch[1] : '' };
        },
      },
    ];
  },

  renderHTML({ node, HTMLAttributes }) {
    return [
      'pre',
      mergeAttributes(HTMLAttributes, {
        class: `code-block ${HTMLAttributes.class || ''}`.trim(),
        'data-language': node.attrs.language || '',
      }),
      ['code', { class: `language-${node.attrs.language || ''}` }, 0],
    ];
  },

  addNodeView() {
    return ({ node }) => {
      const host = document.createElement('div');
      host.className = 'code-block-highlight-wrapper';
      host.style.position = 'relative';

      const language = node.attrs.language || '';
      const initialCode = node.textContent;
      const [isLoading, setIsLoading] = useState(true);
      const contentRef = useRef(null);
      const observerRef = useRef(null);
      const isUpdatingRef = useRef(false);

      const applyHighlighting = async (code, lang) => {
        if (!code || isUpdatingRef.current || !contentRef.current) return;
        isUpdatingRef.current = true;

        try {
          const html = await invoke('get_highlighted_code_html', {
            code: code,
            language: lang || '',
          });

          if (contentRef.current) {
            const tempDiv = document.createElement('div');
            tempDiv.innerHTML = html;
            contentRef.current.innerHTML = tempDiv.innerHTML;
          }
        } catch (error) {
          console.error('Failed to highlight code:', error);
          if (contentRef.current) {
            contentRef.current.textContent = code;
          }
        } finally {
          isUpdatingRef.current = false;
          setIsLoading(false);
        }
      };

      useEffect(() => {
        applyHighlighting(initialCode, language);
      }, []);

      useEffect(() => {
        if (!contentRef.current) return;

        observerRef.current = new MutationObserver((mutations) => {
          if (isUpdatingRef.current) return;

          for (const mutation of mutations) {
            if (mutation.type === 'childList' || mutation.type === 'characterData') {
              const newCode = mutation.target.textContent || '';
              if (newCode.trim() !== '') {
                setTimeout(() => applyHighlighting(newCode, language), 300);
              }
            }
          }
        });

        observerRef.current.observe(contentRef.current, {
          childList: true,
          characterData: true,
          subtree: true,
        });

        return () => {
          if (observerRef.current) observerRef.current.disconnect();
        };
      }, [node]);

      host.innerHTML = `
        <div class="code-block-container" style="position: relative; background: #282c34; border-radius: 0.5rem; overflow: hidden;">
          ${language ? `
            <div class="code-block-header" style="display: flex; justify-content: space-between; align-items: center; padding: 0.5rem 1rem; background: #21252b; border-bottom: 1px solid #181a1f; color: #abb2bf; font-size: 0.75rem; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;">
              <span class="code-block-language">${language}</span>
            </div>
          ` : ''}
          <div class="code-block-content" ref="contentRef" style="padding: 1rem; overflow-x: auto; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; font-size: 0.875rem; line-height: 1.5; color: #abb2bf;">
            <code class="language-${language}">${initialCode}</code>
          </div>
        </div>
      `;

      const contentElement = host.querySelector('.code-block-content');
      if (contentElement) {
        contentRef.current = contentElement;
        setTimeout(() => applyHighlighting(initialCode, language), 0);
      }

      return {
        dom: host,
        contentDOM: contentElement,
        update: (updatedNode) => {
          if (updatedNode.type.name !== 'codeBlock') return false;
          const newLanguage = updatedNode.attrs.language || '';
          const newCode = updatedNode.textContent;
          if (newLanguage !== language) {
            setTimeout(() => applyHighlighting(newCode, newLanguage), 0);
          }
          return true;
        },
      };
    };
  },
});

export default CodeBlockHighlightExtension;