import { useEffect, useRef, useCallback, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useDocument } from '../contexts/DocumentContext';

export function useFileWatcher(pollInterval = 2000) {
  const { currentDocument, setCurrentDocument } = useDocument();
  const [externalChange, setExternalChange] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const pollTimerRef = useRef(null);

  const checkForChanges = useCallback(async () => {
    if (!currentDocument?.filePath || currentDocument.isDirty) {
      return;
    }

    try {
      const result = await invoke('check_external_change', { 
        path: currentDocument.filePath 
      });

      if (result.has_changed) {
        const fileName = currentDocument.filePath.split('/').pop();
        
        setExternalChange({
          path: currentDocument.filePath,
          fileName: fileName,
          newContent: result.content,
          timestamp: Date.now(),
        });
      }
    } catch (e) {
      console.debug('File watcher check error:', e);
    }
  }, [currentDocument]);

  useEffect(() => {
    if (pollTimerRef.current) {
      clearInterval(pollTimerRef.current);
    }

    if (currentDocument?.filePath && !currentDocument.isDirty) {
      pollTimerRef.current = setInterval(checkForChanges, pollInterval);
    }

    return () => {
      if (pollTimerRef.current) {
        clearInterval(pollTimerRef.current);
        pollTimerRef.current = null;
      }
    };
  }, [currentDocument?.filePath, currentDocument?.isDirty, pollInterval, checkForChanges]);

  const handleReload = useCallback(async () => {
    if (!externalChange) return;

    setIsLoading(true);
    try {
      const doc = await invoke('open_document', { path: externalChange.path });
      
      setCurrentDocument({
        id: doc.id,
        title: doc.title,
        content: doc.content,
        filePath: doc.file_path || externalChange.path,
        isDirty: false,
      });

      await invoke('update_watched_file_state', { path: externalChange.path });
      setExternalChange(null);
    } catch (e) {
      console.error('Failed to reload document:', e);
    } finally {
      setIsLoading(false);
    }
  }, [externalChange, setCurrentDocument]);

  const handleKeepCurrent = useCallback(async () => {
    if (!externalChange) return;

    try {
      await invoke('update_watched_file_state', { path: externalChange.path });
      setExternalChange(null);
    } catch (e) {
      console.error('Failed to update file state:', e);
    }
  }, [externalChange]);

  const handleCompareLater = useCallback(() => {
    setExternalChange(null);
  }, []);

  const dismissChange = useCallback(() => {
    setExternalChange(null);
  }, []);

  return {
    externalChange,
    isLoading,
    handleReload,
    handleKeepCurrent,
    handleCompareLater,
    dismissChange,
  };
}