import { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useDocument } from '../contexts/DocumentContext';
import { useToast } from '../contexts/ToastContext';

const VALID_EXTENSIONS = ['.md', '.markdown'];

function isValidMarkdownFile(fileName) {
  const lower = fileName.toLowerCase();
  return VALID_EXTENSIONS.some(ext => lower.endsWith(ext));
}

export default function DropZone({ children }) {
  const [isDragging, setIsDragging] = useState(false);
  const [dragStatus, setDragStatus] = useState('idle');
  const { currentDocument } = useDocument();
  const { error } = useToast();

  const handleDragEnter = useCallback((e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
    
    if (e.dataTransfer.items && e.dataTransfer.items.length > 0) {
      const item = e.dataTransfer.items[0];
      if (item.kind === 'file') {
        const file = item.getAsFile();
        if (file && isValidMarkdownFile(file.name)) {
          setDragStatus('valid');
        } else {
          setDragStatus('invalid');
        }
      }
    }
  }, []);

  const handleDragOver = useCallback((e) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const handleDragLeave = useCallback((e) => {
    e.preventDefault();
    e.stopPropagation();
    
    if (e.relatedTarget && e.currentTarget.contains(e.relatedTarget)) {
      return;
    }
    setIsDragging(false);
    setDragStatus('idle');
  }, []);

  const handleDrop = useCallback(async (e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
    setDragStatus('idle');

    const files = e.dataTransfer.files;
    if (!files || files.length === 0) return;

    const file = files[0];
    
    if (!isValidMarkdownFile(file.name)) {
      error('Only Markdown files (.md, .markdown) can be opened');
      return;
    }

    let filePath = file.path;
    
    if (!filePath && file.webkitRelativePath) {
      filePath = file.webkitRelativePath;
    }
    
    if (!filePath) {
      const pathProp = Object.getOwnPropertyNames(file).find(prop => 
        prop.toLowerCase().includes('path')
      );
      if (pathProp) {
        filePath = file[pathProp];
      }
    }

    if (!filePath) {
      error('Could not get file path. Please use File > Open instead.');
      return;
    }

    try {
      if (currentDocument?.filePath) {
        await invoke('unwatch_file', { path: currentDocument.filePath });
      }

      const doc = await invoke('open_document', { path: filePath });
      await invoke('watch_file', { path: filePath });
      
      window.dispatchEvent(new CustomEvent('file-dropped', { 
        detail: { doc, filePath } 
      }));
    } catch (e) {
      console.error('Drop open error:', e);
      error(`Failed to open file: ${e}`);
    }
  }, [currentDocument, error]);

  useEffect(() => {
    const dropZone = document.getElementById('drop-zone-overlay');
    if (!dropZone) return;

    dropZone.addEventListener('dragenter', handleDragEnter);
    dropZone.addEventListener('dragover', handleDragOver);
    dropZone.addEventListener('dragleave', handleDragLeave);
    dropZone.addEventListener('drop', handleDrop);

    return () => {
      dropZone.removeEventListener('dragenter', handleDragEnter);
      dropZone.removeEventListener('dragover', handleDragOver);
      dropZone.removeEventListener('dragleave', handleDragLeave);
      dropZone.removeEventListener('drop', handleDrop);
    };
  }, [handleDragEnter, handleDragOver, handleDragLeave, handleDrop]);

  return (
    <>
      {children}
      <div
        id="drop-zone-overlay"
        className={`fixed inset-0 z-50 flex items-center justify-center transition-all duration-200 ${
          isDragging ? 'opacity-100' : 'opacity-0 pointer-events-none'
        }`}
        style={{
          backgroundColor: isDragging 
            ? (dragStatus === 'invalid' 
                ? 'rgba(220, 38, 38, 0.1)' 
                : 'rgba(0, 102, 204, 0.1)')
            : 'transparent',
        }}
      >
        <div
          className={`rounded-2xl p-12 border-4 border-dashed transition-all duration-200 ${
            dragStatus === 'invalid' ? 'border-red-500' : 'border-[var(--accent-color)]'
          }`}
          style={{
            backgroundColor: isDragging 
              ? (dragStatus === 'invalid'
                  ? 'rgba(220, 38, 38, 0.05)'
                  : 'rgba(0, 102, 204, 0.05)')
              : 'transparent',
          }}
        >
          <div className="flex flex-col items-center gap-4">
            <div 
              className={`text-6xl ${dragStatus === 'invalid' ? 'opacity-50' : ''}`}
            >
              {dragStatus === 'invalid' ? '📄' : '📥'}
            </div>
            <p 
              className={`text-xl font-semibold ${
                dragStatus === 'invalid' ? 'text-red-500' : 'text-[var(--accent-color)]'
              }`}
            >
              {dragStatus === 'invalid' 
                ? 'Only Markdown files (.md, .markdown) are supported'
                : 'Drop file here'
              }
            </p>
            {dragStatus === 'valid' && (
              <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                Release to open
              </p>
            )}
          </div>
        </div>
      </div>
    </>
  );
}
