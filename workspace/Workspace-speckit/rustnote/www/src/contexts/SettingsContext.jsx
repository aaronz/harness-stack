import { createContext, useContext, useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';

const SettingsContext = createContext();

export function SettingsProvider({ children }) {
  const [settings, setSettingsState] = useState({
    theme: 'light',
    autoSave: true,
    autoSaveInterval: 30000,
    focusMode: false,
    typewriterMode: false,
    outlineVisible: false,
    fontSize: 16,
    fontFamily: 'System',
    lineHeight: 1.6,
    contentWidth: 720,
    recentFiles: [],
    recentFolders: [],
  });
  
  const isDirtyRef = useRef(false);

  useEffect(() => {
    loadSettings();
  }, []);

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', settings.theme);
  }, [settings.theme]);

  async function loadSettings() {
    try {
      const result = await invoke('read_settings');
      setSettingsState(prev => ({
        ...prev,
        theme: result.theme,
        autoSave: result.autoSave,
        autoSaveInterval: result.autoSaveInterval,
        focusMode: result.focusMode,
        typewriterMode: result.typewriterMode,
        outlineVisible: result.outlineVisible,
        fontSize: result.fontSize ?? result.editor?.font_size ?? prev.fontSize,
        fontFamily: result.fontFamily ?? result.editor?.fontFamily ?? prev.fontFamily,
        lineHeight: result.lineHeight ?? result.editor?.line_height ?? prev.lineHeight,
        contentWidth: result.contentWidth ?? result.editor?.content_width ?? prev.contentWidth,
        recentFiles: result.recentFiles ?? result.recent_files ?? prev.recentFiles,
        recentFolders: result.recentFolders ?? result.recent_folders ?? prev.recentFolders,
      }));
    } catch (e) {
      console.log('Using default settings');
    }
  }

  async function saveSettings() {
    if (!isDirtyRef.current) return;
    
    try {
      const settingsToSave = {
        theme: settings.theme,
        autoSave: settings.autoSave,
        autoSaveInterval: settings.autoSaveInterval,
        focusMode: settings.focusMode,
        typewriterMode: settings.typewriterMode,
        outlineVisible: settings.outlineVisible,
        editor: {
          fontFamily: settings.fontFamily,
          fontSize: settings.fontSize,
          lineHeight: settings.lineHeight,
          tabSize: 4,
          contentWidth: settings.contentWidth,
        },
        recentFiles: settings.recentFiles,
        recentFolders: settings.recentFolders,
      };
      await invoke('write_settings', { settings: settingsToSave });
      isDirtyRef.current = false;
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  }

  const setSettings = useCallback((updater) => {
    setSettingsState(prev => {
      const newSettings = typeof updater === 'function' ? updater(prev) : updater;
      isDirtyRef.current = true;
      return newSettings;
    });
    saveSettings();
  }, []);

  const toggleTheme = useCallback(() => {
    setSettings(prev => ({ ...prev, theme: prev.theme === 'light' ? 'dark' : 'light' }));
  }, [setSettings]);

  const toggleFocusMode = useCallback(() => {
    setSettings(prev => ({ ...prev, focusMode: !prev.focusMode }));
  }, [setSettings]);

  const toggleTypewriterMode = useCallback(() => {
    setSettings(prev => ({ ...prev, typewriterMode: !prev.typewriterMode }));
  }, [setSettings]);

  const setTypewriterMode = useCallback((value) => {
    setSettings(prev => ({ ...prev, typewriterMode: value }));
  }, [setSettings]);

  const toggleOutline = useCallback(() => {
    setSettings(prev => ({ ...prev, outlineVisible: !prev.outlineVisible }));
  }, [setSettings]);

  const addToRecentFiles = useCallback((filePath) => {
    if (!filePath) return;
    setSettings(prev => ({
      ...prev,
      recentFiles: [filePath, ...prev.recentFiles.filter(p => p !== filePath)].slice(0, 10),
    }));
  }, [setSettings]);

  const clearRecentFiles = useCallback(() => {
    setSettings(prev => ({ ...prev, recentFiles: [] }));
  }, [setSettings]);

  const addToRecentFolders = useCallback((folderPath) => {
    if (!folderPath) return;
    setSettings(prev => ({
      ...prev,
      recentFolders: [folderPath, ...prev.recentFolders.filter(p => p !== folderPath)].slice(0, 10),
    }));
  }, [setSettings]);

  const clearRecentFolders = useCallback(() => {
    setSettings(prev => ({ ...prev, recentFolders: [] }));
  }, [setSettings]);

  return (
    <SettingsContext.Provider value={{
      settings,
      toggleTheme,
      toggleFocusMode,
      toggleTypewriterMode,
      setTypewriterMode,
      toggleOutline,
      setSettings,
      addToRecentFiles,
      clearRecentFiles,
      addToRecentFolders,
      clearRecentFolders,
    }}>
      {children}
    </SettingsContext.Provider>
  );
}

export function useSettings() {
  const context = useContext(SettingsContext);
  if (!context) {
    throw new Error('useSettings must be used within SettingsProvider');
  }
  return context;
}