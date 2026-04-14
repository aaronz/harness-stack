import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { useDocument } from '../contexts/DocumentContext';
import { useToast } from '../contexts/ToastContext';

const EXPORT_FORMATS = {
  HTML_STANDALONE: 'html_standalone',
  HTML_LINKED: 'html_linked',
  PDF_A4: 'pdf_a4',
  PDF_LETTER: 'pdf_letter',
  PDF_LEGAL: 'pdf_legal',
};

const FORMAT_LABELS = {
  [EXPORT_FORMATS.HTML_STANDALONE]: 'HTML (Standalone)',
  [EXPORT_FORMATS.HTML_LINKED]: 'HTML (Linked Assets)',
  [EXPORT_FORMATS.PDF_A4]: 'PDF (A4)',
  [EXPORT_FORMATS.PDF_LETTER]: 'PDF (Letter)',
  [EXPORT_FORMATS.PDF_LEGAL]: 'PDF (Legal)',
};

export default function ExportModal({ isVisible, onClose }) {
  const { currentDocument } = useDocument();
  const { success, error } = useToast();
  const [selectedFormat, setSelectedFormat] = useState(EXPORT_FORMATS.HTML_STANDALONE);
  const [isExporting, setIsExporting] = useState(false);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    if (isVisible) {
      setSelectedFormat(EXPORT_FORMATS.HTML_STANDALONE);
      setProgress(0);
    }
  }, [isVisible]);

  if (!isVisible) {
    return null;
  }

  const getFileExtension = (format) => {
    switch (format) {
      case EXPORT_FORMATS.HTML_STANDALONE:
      case EXPORT_FORMATS.HTML_LINKED:
        return 'html';
      case EXPORT_FORMATS.PDF_A4:
      case EXPORT_FORMATS.PDF_LETTER:
      case EXPORT_FORMATS.PDF_LEGAL:
        return 'pdf';
      default:
        return 'html';
    }
  };

  const getDefaultFileName = () => {
    const title = currentDocument?.title || 'Untitled';
    const safeName = title.replace(/[^a-zA-Z0-9_-]/g, '_');
    return `${safeName}_export.${getFileExtension(selectedFormat)}`;
  };

  const handleExport = async () => {
    if (!currentDocument?.content) {
      error('No document content to export');
      return;
    }

    const markdown = currentDocument.content;
    const ext = getFileExtension(selectedFormat);

    try {
      const outputPath = await save({
        filters: [{ name: ext.toUpperCase(), extensions: [ext] }],
        defaultPath: getDefaultFileName(),
      });

      if (!outputPath) {
        return;
      }

      setIsExporting(true);
      setProgress(10);

      if (selectedFormat === EXPORT_FORMATS.HTML_STANDALONE) {
        setProgress(30);
        const inlineOptions = {
          mode: { type: 'Inline' },
          embed_css: true,
        };
        await invoke('export_to_html', { markdown, outputPath, options: inlineOptions });
        setProgress(100);
        success('HTML exported successfully');
      } else if (selectedFormat === EXPORT_FORMATS.HTML_LINKED) {
        setProgress(30);
        const linkedOptions = {
          mode: { type: 'Linked', value: { assets_dir: 'assets' } },
          embed_css: true,
        };
        await invoke('export_to_html', { markdown, outputPath, options: linkedOptions });
        setProgress(100);
        success('HTML exported successfully');
      } else if (selectedFormat.startsWith('pdf_')) {
        setProgress(30);
        let pageSize;
        switch (selectedFormat) {
          case EXPORT_FORMATS.PDF_LETTER:
            pageSize = { type: 'Letter' };
            break;
          case EXPORT_FORMATS.PDF_LEGAL:
            pageSize = { type: 'Legal' };
            break;
          case EXPORT_FORMATS.PDF_A4:
          default:
            pageSize = { type: 'A4' };
            break;
        }

        const options = {
          page_size: pageSize,
          margins: { top_mm: 20, right_mm: 20, bottom_mm: 20, left_mm: 20 },
          embed_images: true,
        };

        await invoke('export_to_pdf_native', { markdown, outputPath, options });
        setProgress(100);
        success('PDF exported successfully');
      }

      onClose();
    } catch (e) {
      console.error('Export error:', e);
      error(`Export failed: ${e}`);
    } finally {
      setIsExporting(false);
      setProgress(0);
    }
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Escape') {
      onClose();
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      onKeyDown={handleKeyDown}
    >
      <div
        className="absolute inset-0 bg-black/50"
        onClick={onClose}
      />

      <div
        className="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md mx-4"
        style={{
          backgroundColor: 'var(--bg-primary)',
          color: 'var(--text-primary)',
        }}
      >
        <div
          className="flex items-center justify-between px-6 py-4 border-b"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <h2 className="text-lg font-semibold">Export Document</h2>
          <button
            id="btn-export-close"
            onClick={onClose}
            className="text-2xl leading-none opacity-60 hover:opacity-100"
            style={{ color: 'var(--text-primary)' }}
          >
            ×
          </button>
        </div>

        <div className="px-6 py-4">
          <div className="mb-4">
            <label
              className="block text-sm font-medium mb-2"
              style={{ color: 'var(--text-primary)' }}
            >
              Export Format
            </label>
            <div className="space-y-2">
              {Object.entries(FORMAT_LABELS).map(([value, label]) => (
                <label
                  key={value}
                  className="flex items-center gap-3 p-3 rounded border cursor-pointer transition-colors"
                  style={{
                    borderColor: selectedFormat === value ? 'var(--accent-color)' : 'var(--border-color)',
                    backgroundColor: selectedFormat === value ? 'var(--bg-secondary)' : 'transparent',
                  }}
                >
                  <input
                    type="radio"
                    name="export-format"
                    value={value}
                    checked={selectedFormat === value}
                    onChange={(e) => setSelectedFormat(e.target.value)}
                    className="w-4 h-4"
                  />
                  <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                    {label}
                  </span>
                </label>
              ))}
            </div>
          </div>

          {isExporting && (
            <div className="mb-4">
              <div
                className="h-2 rounded-full overflow-hidden"
                style={{ backgroundColor: 'var(--bg-secondary)' }}
              >
                <div
                  className="h-full transition-all duration-300 ease-out"
                  style={{
                    width: `${progress}%`,
                    backgroundColor: 'var(--accent-color)',
                  }}
                />
              </div>
              <p className="text-xs mt-1" style={{ color: 'var(--text-secondary)' }}>
                Exporting... {progress}%
              </p>
            </div>
          )}
        </div>

        <div
          className="flex justify-end gap-2 px-6 py-4 border-t"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <button
            id="btn-export-cancel"
            onClick={onClose}
            disabled={isExporting}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          >
            Cancel
          </button>
          <button
            id="btn-export-confirm"
            onClick={handleExport}
            disabled={isExporting}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors disabled:opacity-50"
            style={{
              backgroundColor: 'var(--accent-color)',
              borderColor: 'var(--accent-color)',
              color: 'white',
            }}
          >
            {isExporting ? 'Exporting...' : 'Export'}
          </button>
        </div>
      </div>
    </div>
  );
}

export { EXPORT_FORMATS };