import { useEffect, useRef, useState } from 'react';
import { SettingsProvider, useSettings } from './contexts/SettingsContext';
import { DocumentProvider, useDocument } from './contexts/DocumentContext';
import { SearchProvider, useSearch } from './contexts/SearchContext';
import { ToastProvider } from './contexts/ToastContext';
import Toolbar from './components/Toolbar';
import Sidebar from './components/Sidebar';
import TipTapEditor from './components/TipTapEditor';
import OutlinePanel from './components/OutlinePanel';
import SearchPanel from './components/SearchPanel';
import ExportModal from './components/ExportModal';
import Toast from './components/Toast';
import DropZone from './components/DropZone';

function AppContent() {
  const { settings } = useSettings();
  const { createNewDocument, currentDocument, setCurrentDocument } = useDocument();
  const { isSearchVisible, showSearch, hideSearch, findNext, findPrev } = useSearch();
  const [isExportVisible, setIsExportVisible] = useState(false);
  const editorRef = useRef(null);

  const handleHeadingClick = (heading) => {
    editorRef.current?.scrollToHeading(heading);
  };

  const handleExportClick = () => {
    setIsExportVisible(true);
  };

  const handleExportClose = () => {
    setIsExportVisible(false);
  };

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', settings.theme);
  }, [settings.theme]);

  useEffect(() => {
    if (!currentDocument) {
      createNewDocument();
    }
  }, []);

  useEffect(() => {
    const handleFileDropped = (e) => {
      const { doc, filePath } = e.detail;
      if (doc) {
        setCurrentDocument({
          id: doc.id,
          title: doc.title,
          content: doc.content,
          filePath: doc.file_path || filePath || null,
          isDirty: doc.is_dirty,
        });
      }
    };

    window.addEventListener('file-dropped', handleFileDropped);
    return () => window.removeEventListener('file-dropped', handleFileDropped);
  }, [setCurrentDocument]);

  useEffect(() => {
    const handleKeyDown = (e) => {
      const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
      const modifier = isMac ? e.metaKey : e.ctrlKey;

      if (modifier && e.key === 'f') {
        e.preventDefault();
        showSearch();
        return;
      }

      if (modifier && e.key === 'h') {
        e.preventDefault();
        showSearch();
        return;
      }

      if (e.key === 'F3' && !modifier) {
        e.preventDefault();
        if (isSearchVisible) {
          findNext();
        }
        return;
      }

      if (e.key === 'F3' && e.shiftKey) {
        e.preventDefault();
        if (isSearchVisible) {
          findPrev();
        }
        return;
      }

      if (modifier && e.key === 'g') {
        e.preventDefault();
        if (isSearchVisible) {
          findNext();
        }
        return;
      }

      if (e.key === 'Escape' && isSearchVisible) {
        e.preventDefault();
        hideSearch();
        return;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [showSearch, hideSearch, findNext, findPrev, isSearchVisible]);

  return (
    <DropZone>
      <div id="app" className="flex h-screen w-screen">
        <Sidebar />
        <div className="flex-1 flex flex-col overflow-hidden">
          <Toolbar onExportClick={handleExportClick} />
          <SearchPanel isVisible={isSearchVisible} onClose={hideSearch} />
          <ExportModal isVisible={isExportVisible} onClose={handleExportClose} />
          <TipTapEditor ref={editorRef} />
        </div>
        <OutlinePanel onHeadingClick={handleHeadingClick} />
      </div>
    </DropZone>
  );
}

export default function App() {
  return (
    <SettingsProvider>
      <DocumentProvider>
        <SearchProvider>
          <ToastProvider>
            <AppContent />
            <Toast />
          </ToastProvider>
        </SearchProvider>
      </DocumentProvider>
    </SettingsProvider>
  );
}