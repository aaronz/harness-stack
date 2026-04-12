import { useEffect, useRef } from 'react';

/**
 * Auto-save timer hook that triggers save after a configurable interval
 * when the document is dirty and auto-save is enabled.
 * 
 * @param {boolean} isDirty - Whether the document has unsaved changes
 * @param {boolean} autoSave - Whether auto-save is enabled
 * @param {number} autoSaveInterval - Interval in milliseconds
 * @param {string} content - The document content (used to reset timer on edits)
 * @param {Function} saveDocument - Function to call to save the document
 */
export function useAutoSaveTimer(isDirty, autoSave, autoSaveInterval, content, saveDocument) {
  const timerRef = useRef(null);

  useEffect(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }

    if (isDirty && autoSave) {
      timerRef.current = setTimeout(() => {
        saveDocument();
      }, autoSaveInterval);
    }

    return () => {
      if (timerRef.current) {
        clearTimeout(timerRef.current);
        timerRef.current = null;
      }
    };
  }, [isDirty, autoSave, autoSaveInterval, content, saveDocument]);
}