import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
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
import PreferencesModal from './components/PreferencesModal';
import ExternalChangeModal from './components/ExternalChangeModal';
import RecoveryModal from './components/RecoveryModal';
import Toast from './components/Toast';
import DropZone from './components/DropZone';
import { useFileWatcher } from './hooks/useFileWatcher';

function AppContent() {
  const { settings } = useSettings();
  const { createNewDocument, currentDocument, setCurrentDocument, saveDocument, openDocument, openWorkspace } = useDocument();
  const { isSearchVisible, showSearch, hideSearch, findNext, findPrev } = useSearch();
  const [isExportVisible, setIsExportVisible] = useState(false);
  const [isPreferencesVisible, setIsPreferencesVisible] = useState(false);
  const [recoverySnapshots, setRecoverySnapshots] = useState([]);
  const [isRecoveryVisible, setIsRecoveryVisible] = useState(false);
  const [isRecovering, setIsRecovering] = useState(false);
  const editorRef = useRef(null);

  const {
    externalChange,
    isLoading,
    handleReload,
    handleKeepCurrent,
    handleCompareLater,
  } = useFileWatcher();

  useEffect(() => {
    async function checkRecoverySnapshots() {
      try {
        const snapshots = await invoke('list_recovery_snapshots');
        if (snapshots && snapshots.length > 0) {
          setRecoverySnapshots(snapshots);
          setIsRecoveryVisible(true);
        }
      } catch (e) {
        console.error('Failed to check recovery snapshots:', e);
      }
    }
    checkRecoverySnapshots();
  }, []);

  const handleRecover = async (snapshotId) => {
    setIsRecovering(true);
    try {
      const recoveryData = await invoke('restore_recovery_snapshot', { id: snapshotId });
      if (recoveryData) {
        setCurrentDocument({
          id: crypto.randomUUID(),
          title: recoveryData.title || 'Recovered',
          content: recoveryData.content,
          filePath: recoveryData.file_path || null,
          isDirty: true,
        });
        await invoke('delete_recovery_snapshot', { id: snapshotId });
        const snapshots = await invoke('list_recovery_snapshots');
        setRecoverySnapshots(snapshots);
        if (snapshots.length === 0) {
          setIsRecoveryVisible(false);
        }
      }
    } catch (e) {
      console.error('Failed to recover snapshot:', e);
    } finally {
      setIsRecovering(false);
    }
  };

  const handleDeleteSnapshot = async (snapshotId) => {
    try {
      await invoke('delete_recovery_snapshot', { id: snapshotId });
      const snapshots = await invoke('list_recovery_snapshots');
      setRecoverySnapshots(snapshots);
      if (snapshots.length === 0) {
        setIsRecoveryVisible(false);
      }
    } catch (e) {
      console.error('Failed to delete snapshot:', e);
    }
  };

  const handleStartFresh = async () => {
    try {
      for (const snapshot of recoverySnapshots) {
        await invoke('delete_recovery_snapshot', { id: snapshot.id });
      }
      setRecoverySnapshots([]);
      setIsRecoveryVisible(false);
      createNewDocument();
    } catch (e) {
      console.error('Failed to start fresh:', e);
      setIsRecoveryVisible(false);
      createNewDocument();
    }
  };

  const handleHeadingClick = (heading) => {
    editorRef.current?.scrollToHeading(heading);
  };

  const handleExportClick = () => {
    setIsExportVisible(true);
  };

  const handleExportClose = () => {
    setIsExportVisible(false);
  };

  const handlePreferencesClick = () => {
    setIsPreferencesVisible(true);
  };

  const handlePreferencesClose = () => {
    setIsPreferencesVisible(false);
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

      if (modifier && e.key === 'n') {
        e.preventDefault();
        if (currentDocument?.isDirty) {
          const shouldSave = window.confirm('Do you want to save changes before creating a new document?');
          if (shouldSave) {
            saveDocument();
          }
          if (!shouldSave) {
            return;
          }
        }
        createNewDocument();
        return;
      }

      if (modifier && e.key === 'o') {
        e.preventDefault();
        if (currentDocument?.isDirty) {
          const shouldSave = window.confirm('Do you want to save changes before opening a new file?');
          if (shouldSave) {
            saveDocument();
          }
          if (!shouldSave) {
            return;
          }
        }
        openDocument();
        return;
      }

      if (e.ctrlKey && e.shiftKey && e.key === 'O') {
        e.preventDefault();
        openWorkspace();
        return;
      }

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
  }, [showSearch, hideSearch, findNext, findPrev, isSearchVisible, openDocument, openWorkspace, currentDocument, saveDocument]);

  return (
    <DropZone>
      <div id="app" className="flex h-screen w-screen">
        <Sidebar />
        <div className="flex-1 flex flex-col overflow-hidden">
          <Toolbar onExportClick={handleExportClick} onPreferencesClick={handlePreferencesClick} />
          <SearchPanel isVisible={isSearchVisible} onClose={hideSearch} />
          <ExportModal isVisible={isExportVisible} onClose={handleExportClose} />
          <PreferencesModal isVisible={isPreferencesVisible} onClose={handlePreferencesClose} />
          <ExternalChangeModal
            isVisible={!!externalChange}
            fileName={externalChange?.fileName}
            onReload={handleReload}
            onKeepCurrent={handleKeepCurrent}
            onCompareLater={handleCompareLater}
            isLoading={isLoading}
          />
          <RecoveryModal
            isVisible={isRecoveryVisible}
            snapshots={recoverySnapshots}
            onRecover={handleRecover}
            onStartFresh={handleStartFresh}
            onDeleteSnapshot={handleDeleteSnapshot}
            isLoading={isRecovering}
          />
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