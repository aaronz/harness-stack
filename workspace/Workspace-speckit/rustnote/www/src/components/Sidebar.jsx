import { useState, useCallback } from 'react';
import { useDocument } from '../contexts/DocumentContext';
import { useToast } from '../contexts/ToastContext';
import { useSettings } from '../contexts/SettingsContext';
import { invoke } from '@tauri-apps/api/core';

function FileItem({ file, level = 0 }) {
  const { currentDocument, openDocument, createFile, createFolder, renameItem, deleteItem } = useDocument();
  const { success, error } = useToast();
  const [isExpanded, setIsExpanded] = useState(true);
  const [contextMenu, setContextMenu] = useState(null);
  const [isRenaming, setIsRenaming] = useState(false);
  const [newName, setNewName] = useState(file.name);

  const handleClick = useCallback(async (e) => {
    e.stopPropagation();
    if (file.is_directory) {
      setIsExpanded(!isExpanded);
    } else {
      try {
        await openDocument(file.path);
      } catch (e) {
        error(`Failed to open file: ${e}`);
      }
    }
  }, [file, isExpanded, openDocument, error]);

  const handleContextMenu = useCallback((e) => {
    e.preventDefault();
    e.stopPropagation();
    setContextMenu({
      x: e.clientX,
      y: e.clientY,
      file,
    });
  }, [file]);

  const closeContextMenu = useCallback(() => {
    setContextMenu(null);
  }, []);

  const handleNewFile = useCallback(async () => {
    closeContextMenu();
    const name = prompt('Enter file name:', 'untitled.md');
    if (!name) return;
    
    try {
      await createFile(file.path, name);
      success('File created successfully');
    } catch (e) {
      error(`Failed to create file: ${e}`);
    }
  }, [file.path, createFile, success, error, closeContextMenu]);

  const handleNewFolder = useCallback(async () => {
    closeContextMenu();
    const name = prompt('Enter folder name:', 'new-folder');
    if (!name) return;
    
    try {
      await createFolder(file.path, name);
      success('Folder created successfully');
    } catch (e) {
      error(`Failed to create folder: ${e}`);
    }
  }, [file.path, createFolder, success, error, closeContextMenu]);

  const handleRename = useCallback(() => {
    closeContextMenu();
    setIsRenaming(true);
    setNewName(file.name);
  }, [file.name, closeContextMenu]);

  const handleRenameSubmit = useCallback(async () => {
    if (!newName.trim()) {
      setIsRenaming(false);
      return;
    }
    
    if (newName === file.name) {
      setIsRenaming(false);
      return;
    }
    
    try {
      await renameItem(file.path, newName);
      success('Renamed successfully');
    } catch (e) {
      error(`Failed to rename: ${e}`);
    }
    setIsRenaming(false);
  }, [newName, file.path, file.name, renameItem, success, error]);

  const handleDelete = useCallback(async () => {
    closeContextMenu();
    const confirmed = confirm(`Are you sure you want to delete "${file.name}"?`);
    if (!confirmed) return;
    
    try {
      await deleteItem(file.path);
      success('Deleted successfully');
    } catch (e) {
      error(`Failed to delete: ${e}`);
    }
  }, [file.path, file.name, deleteItem, success, error, closeContextMenu]);

  const handleKeyDown = useCallback((e) => {
    if (e.key === 'Enter') {
      handleRenameSubmit();
    } else if (e.key === 'Escape') {
      setIsRenaming(false);
      setNewName(file.name);
    }
  }, [handleRenameSubmit, file.name]);

  const isActive = currentDocument?.filePath === file.path;
  const hasChildren = file.is_directory && file.children && file.children.length > 0;

  return (
    <>
      <div
        className="select-none"
        onContextMenu={handleContextMenu}
      >
        <div
          onClick={handleClick}
          className="px-3 py-2 rounded cursor-pointer text-sm flex items-center gap-2"
          style={{
            backgroundColor: isActive ? 'var(--accent-color)' : 'transparent',
            color: isActive ? 'white' : 'var(--text-primary)',
            paddingLeft: `${12 + level * 16}px`,
          }}
        >
          {file.is_directory && (
            <span className="text-xs opacity-60">
              {isExpanded ? '▼' : '▶'}
            </span>
          )}
          <span>{file.is_directory ? '📁' : '📄'}</span>
          {isRenaming ? (
            <input
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              onBlur={handleRenameSubmit}
              onKeyDown={handleKeyDown}
              className="flex-1 px-1 py-0 text-sm bg-transparent border border-[var(--accent-color)] rounded outline-none"
              autoFocus
              onClick={(e) => e.stopPropagation()}
            />
          ) : (
            <span className="truncate">{file.name}</span>
          )}
        </div>
      </div>
      
      {hasChildren && isExpanded && (
        <div>
          {file.children.map((child, index) => (
            <FileItem key={`${child.path}-${index}`} file={child} level={level + 1} />
          ))}
        </div>
      )}
      
      {contextMenu && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          file={contextMenu.file}
          onClose={closeContextMenu}
          onNewFile={handleNewFile}
          onNewFolder={handleNewFolder}
          onRename={handleRename}
          onDelete={handleDelete}
        />
      )}
    </>
  );
}

function ContextMenu({ x, y, file, onClose, onNewFile, onNewFolder, onRename, onDelete }) {
  const { success, error } = useToast();
  const [inputName, setInputName] = useState('');
  const [inputMode, setInputMode] = useState(null);
  const { createFile, createFolder, renameItem, deleteItem } = useDocument();
  const { refreshWorkspace } = useDocument();

  const handleNewFile = useCallback(async () => {
    const name = prompt('Enter file name:', 'untitled.md');
    if (!name) return;
    
    try {
      await createFile(file.path, name);
      success('File created successfully');
    } catch (e) {
      error(`Failed to create file: ${e}`);
    }
    onClose();
  }, [file.path, createFile, success, error, onClose]);

  const handleNewFolder = useCallback(async () => {
    const name = prompt('Enter folder name:', 'new-folder');
    if (!name) return;
    
    try {
      await createFolder(file.path, name);
      success('Folder created successfully');
    } catch (e) {
      error(`Failed to create folder: ${e}`);
    }
    onClose();
  }, [file.path, createFolder, success, error, onClose]);

  const handleRename = useCallback(async () => {
    const name = prompt('Enter new name:', file.name);
    if (!name || name === file.name) {
      onClose();
      return;
    }
    
    try {
      await renameItem(file.path, name);
      success('Renamed successfully');
    } catch (e) {
      error(`Failed to rename: ${e}`);
    }
    onClose();
  }, [file.path, file.name, renameItem, success, error, onClose]);

  const handleDelete = useCallback(async () => {
    const confirmed = confirm(`Are you sure you want to delete "${file.name}"?`);
    if (!confirmed) {
      onClose();
      return;
    }
    
    try {
      await deleteItem(file.path);
      success('Deleted successfully');
    } catch (e) {
      error(`Failed to delete: ${e}`);
    }
    onClose();
  }, [file.path, file.name, deleteItem, success, error, onClose]);

  return (
    <>
      <div
        className="fixed inset-0 z-40"
        onClick={onClose}
        onContextMenu={(e) => { e.preventDefault(); onClose(); }}
      />
      <div
        className="fixed z-50 py-1 rounded-lg shadow-lg min-w-[160px]"
        style={{
          left: x,
          top: y,
          backgroundColor: 'var(--bg-secondary)',
          border: '1px solid var(--border-color)',
        }}
      >
        {file.is_directory && (
          <>
            <button
              onClick={handleNewFile}
              className="w-full px-4 py-2 text-sm text-left hover:bg-[var(--bg-hover)]"
              style={{ color: 'var(--text-primary)' }}
            >
              📄 New File
            </button>
            <button
              onClick={handleNewFolder}
              className="w-full px-4 py-2 text-sm text-left hover:bg-[var(--bg-hover)]"
              style={{ color: 'var(--text-primary)' }}
            >
              📁 New Folder
            </button>
            <div className="my-1 border-t" style={{ borderColor: 'var(--border-color)' }} />
          </>
        )}
        <button
          onClick={handleRename}
          className="w-full px-4 py-2 text-sm text-left hover:bg-[var(--bg-hover)]"
          style={{ color: 'var(--text-primary)' }}
        >
          ✏️ Rename
        </button>
        <button
          onClick={handleDelete}
          className="w-full px-4 py-2 text-sm text-left hover:bg-[var(--bg-hover)] text-red-500"
        >
          🗑️ Delete
        </button>
      </div>
    </>
  );
}

function WorkspaceContextMenu({ x, y, rootPath, onClose }) {
  const { success, error } = useToast();
  const { createFile, createFolder, refreshWorkspace } = useDocument();

  const handleNewFile = useCallback(async () => {
    const name = prompt('Enter file name:', 'untitled.md');
    if (!name) return;
    
    try {
      await createFile(rootPath, name);
      success('File created successfully');
    } catch (e) {
      error(`Failed to create file: ${e}`);
    }
    onClose();
  }, [rootPath, createFile, success, error, onClose]);

  const handleNewFolder = useCallback(async () => {
    const name = prompt('Enter folder name:', 'new-folder');
    if (!name) return;
    
    try {
      await createFolder(rootPath, name);
      success('Folder created successfully');
    } catch (e) {
      error(`Failed to create folder: ${e}`);
    }
    onClose();
  }, [rootPath, createFolder, success, error, onClose]);

  return (
    <>
      <div
        className="fixed inset-0 z-40"
        onClick={onClose}
        onContextMenu={(e) => { e.preventDefault(); onClose(); }}
      />
      <div
        className="fixed z-50 py-1 rounded-lg shadow-lg min-w-[160px]"
        style={{
          left: x,
          top: y,
          backgroundColor: 'var(--bg-secondary)',
          border: '1px solid var(--border-color)',
        }}
      >
        <button
          onClick={handleNewFile}
          className="w-full px-4 py-2 text-sm text-left hover:bg-[var(--bg-hover)]"
          style={{ color: 'var(--text-primary)' }}
        >
          📄 New File
        </button>
        <button
          onClick={handleNewFolder}
          className="w-full px-4 py-2 text-sm text-left hover:bg-[var(--bg-hover)]"
          style={{ color: 'var(--text-primary)' }}
        >
          📁 New Folder
        </button>
      </div>
    </>
  );
}

export default function Sidebar() {
  const { workspace, openWorkspace, currentDocument, setCurrentDocument } = useDocument();
  const { settings, clearRecentFiles } = useSettings();
  const { success, error } = useToast();
  const [rootContextMenu, setRootContextMenu] = useState(null);

  const handleRootContextMenu = useCallback((e) => {
    e.preventDefault();
    setRootContextMenu({
      x: e.clientX,
      y: e.clientY,
    });
  }, []);

  const closeRootContextMenu = useCallback(() => {
    setRootContextMenu(null);
  }, []);

  const handleOpenRecentFile = useCallback(async (filePath) => {
    try {
      const doc = await invoke('open_document', { path: filePath });
      setCurrentDocument({
        id: doc.id,
        title: doc.title,
        content: doc.content,
        filePath: doc.file_path || null,
        isDirty: doc.is_dirty,
      });
      success('Opened recent file');
    } catch (e) {
      error(`Failed to open file: ${e}`);
    }
  }, [setCurrentDocument, success, error]);

  const handleClearRecentFiles = useCallback(() => {
    clearRecentFiles();
    success('Cleared recent files');
  }, [clearRecentFiles, success]);

  const getFileName = (filePath) => {
    if (!filePath) return 'Unknown';
    const parts = filePath.split('/');
    return parts[parts.length - 1];
  };

  const getFileDir = (filePath) => {
    if (!filePath) return '';
    const parts = filePath.split('/');
    parts.pop();
    if (parts.length <= 2) return parts.join('/');
    return '.../' + parts.slice(-2).join('/');
  };

  return (
    <div
      id="sidebar"
      className="w-64 flex flex-col border-r"
      style={{
        backgroundColor: 'var(--bg-secondary)',
        borderColor: 'var(--border-color)',
      }}
    >
      {/* Recent Files Section */}
      {settings.recentFiles && settings.recentFiles.length > 0 && (
        <div className="border-b" style={{ borderColor: 'var(--border-color)' }}>
          <div
            className="px-4 py-3 font-semibold flex items-center justify-between"
          >
            <span className="text-xs uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>Recent</span>
            <button
              onClick={handleClearRecentFiles}
              className="text-xs px-2 py-0.5 rounded hover:bg-[var(--bg-hover)]"
              style={{ color: 'var(--text-secondary)' }}
              title="Clear recent files"
            >
              Clear
            </button>
          </div>
          <div className="max-h-48 overflow-y-auto">
            {settings.recentFiles.map((filePath, index) => (
              <div
                key={`recent-${filePath}-${index}`}
                onClick={() => handleOpenRecentFile(filePath)}
                className="px-4 py-2 cursor-pointer text-sm hover:bg-[var(--bg-hover)] truncate"
                style={{ color: 'var(--text-primary)' }}
                title={filePath}
              >
                <div className="truncate font-medium">{getFileName(filePath)}</div>
                <div className="text-xs truncate" style={{ color: 'var(--text-secondary)' }}>
                  {getFileDir(filePath)}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      <div
        className="px-4 py-4 font-semibold border-b flex items-center justify-between"
        style={{ borderColor: 'var(--border-color)' }}
      >
        <span>Files</span>
        {workspace && (
          <button
            onClick={handleRootContextMenu}
            className="text-xs px-2 py-1 rounded hover:bg-[var(--bg-hover)]"
            style={{ color: 'var(--text-secondary)' }}
            title="New File/Folder"
          >
            +
          </button>
        )}
      </div>

      <div className="flex-1 overflow-y-auto p-2">
        {!workspace ? (
          <div className="text-sm p-2" style={{ color: 'var(--text-secondary)' }}>
            <p className="mb-2">No workspace open</p>
            <button
              onClick={openWorkspace}
              className="px-3 py-1.5 text-sm border rounded cursor-pointer w-full"
              style={{
                backgroundColor: 'var(--bg-primary)',
                borderColor: 'var(--border-color)',
                color: 'var(--text-primary)',
              }}
            >
              Open Workspace
            </button>
          </div>
        ) : (
          <div className="space-y-1">
            {workspace.files?.map((file, index) => (
              <FileItem key={`${file.path}-${index}`} file={file} />
            ))}
          </div>
        )}
      </div>

      {rootContextMenu && (
        <WorkspaceContextMenu
          x={rootContextMenu.x}
          y={rootContextMenu.y}
          rootPath={workspace.root_path}
          onClose={closeRootContextMenu}
        />
      )}
    </div>
  );
}