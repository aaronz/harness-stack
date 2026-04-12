import { createContext, useContext, useState, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { useSettings } from './SettingsContext';

const DocumentContext = createContext();

export function DocumentProvider({ children }) {
  const [currentDocument, setCurrentDocument] = useState(null);
  const [workspace, setWorkspace] = useState(null);
  const [isSaving, setIsSaving] = useState(false);
  const insertTextRef = useRef(null);
  const { addToRecentFiles } = useSettings();

  const setInsertTextCallback = useCallback((callback) => {
    insertTextRef.current = callback;
  }, []);

  const triggerInsertText = useCallback((text) => {
    if (insertTextRef.current) {
      insertTextRef.current(text);
    }
  }, []);

  async function createNewDocument() {
    try {
      const doc = await invoke('create_document', { title: 'Untitled' });
      setCurrentDocument({
        id: doc.id,
        title: doc.title,
        content: doc.content,
        filePath: doc.file_path || null,
        isDirty: doc.is_dirty,
      });
    } catch (e) {
      console.error('Create document error:', e);
      setCurrentDocument({
        title: 'Untitled',
        content: '',
        filePath: null,
        isDirty: false,
      });
    }
  }

  async function openDocument() {
    try {
      const result = await open({
        filters: [{ name: 'Markdown', extensions: ['md', 'markdown'] }],
      });

      if (result) {
        if (currentDocument?.filePath) {
          await invoke('unwatch_file', { path: currentDocument.filePath });
        }

        const doc = await invoke('open_document', { path: result });
        setCurrentDocument({
          id: doc.id,
          title: doc.title,
          content: doc.content,
          filePath: doc.file_path || null,
          isDirty: doc.is_dirty,
        });

        await invoke('watch_file', { path: result });
        addToRecentFiles(result);
      }
    } catch (e) {
      console.error('Open error:', e);
    }
  }

  const saveDocument = useCallback(async function() {
    if (!currentDocument) return;

    setIsSaving(true);
    try {
      if (currentDocument.filePath) {
        await invoke('save_document', {
          path: currentDocument.filePath,
          content: currentDocument.content,
        });
        await invoke('update_watched_file_state', {
          path: currentDocument.filePath,
        });
        setCurrentDocument(prev => ({ ...prev, isDirty: false }));
      } else {
        await saveDocumentAs();
      }
    } catch (e) {
      console.error('Save error:', e);
    } finally {
      setIsSaving(false);
    }
  }, [currentDocument]);

  async function saveDocumentAs() {
    if (!currentDocument) return;

    try {
      const result = await save({
        filters: [{ name: 'Markdown', extensions: ['md'] }],
        defaultPath: currentDocument.title + '.md',
      });

      if (result) {
        setCurrentDocument(prev => ({ ...prev, filePath: result }));
        await saveDocument();
      }
    } catch (e) {
      console.error('Save as error:', e);
    }
  }

  function updateContent(content) {
    setCurrentDocument(prev => ({
      ...prev,
      content,
      isDirty: true,
    }));
  }

  async function openWorkspace() {
    try {
      const result = await open({
        directory: true,
        title: 'Open Folder as Workspace',
      });

      if (result) {
        const ws = await invoke('list_workspace', { path: result });
        setWorkspace(ws);
      }
    } catch (e) {
      console.error('Workspace error:', e);
    }
  }

  async function refreshWorkspace() {
    if (!workspace?.root_path) return;
    try {
      const ws = await invoke('list_workspace', { path: workspace.root_path });
      setWorkspace(ws);
    } catch (e) {
      console.error('Refresh workspace error:', e);
    }
  }

  async function createFile(parentPath, name) {
    try {
      await invoke('create_file', { parentPath, name });
      await refreshWorkspace();
      return true;
    } catch (e) {
      console.error('Create file error:', e);
      throw e;
    }
  }

  async function createFolder(parentPath, name) {
    try {
      await invoke('create_folder', { parentPath, name });
      await refreshWorkspace();
      return true;
    } catch (e) {
      console.error('Create folder error:', e);
      throw e;
    }
  }

  async function renameItem(oldPath, newName) {
    try {
      await invoke('rename_item', { oldPath, newName });
      await refreshWorkspace();
      return true;
    } catch (e) {
      console.error('Rename item error:', e);
      throw e;
    }
  }

  async function deleteItem(path) {
    try {
      await invoke('delete_item', { path });
      await refreshWorkspace();
      return true;
    } catch (e) {
      console.error('Delete item error:', e);
      throw e;
    }
  }

  async function insertImage() {
    try {
      const result = await open({
        filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'] }],
        multiple: false,
      });

      if (result) {
        const workspacePath = currentDocument?.filePath
          ? currentDocument.filePath.substring(0, currentDocument.filePath.lastIndexOf('/'))
          : null;

        const imageInfo = await invoke('insert_image', {
          imagePath: result,
          workspacePath,
        });

        if (imageInfo) {
          const markdown = `![${imageInfo.file_name}](${imageInfo.relative_path})`;
          triggerInsertText(markdown);
        }
      }
    } catch (e) {
      console.error('Insert image error:', e);
    }
  }

  return (
    <DocumentContext.Provider value={{
      currentDocument,
      setCurrentDocument,
      createNewDocument,
      openDocument,
      saveDocument,
      saveDocumentAs,
      updateContent,
      workspace,
      openWorkspace,
      refreshWorkspace,
      createFile,
      createFolder,
      renameItem,
      deleteItem,
      insertImage,
      setInsertTextCallback,
      isSaving,
    }}>
      {children}
    </DocumentContext.Provider>
  );
}

export function useDocument() {
  const context = useContext(DocumentContext);
  if (!context) {
    throw new Error('useDocument must be used within DocumentProvider');
  }
  return context;
}