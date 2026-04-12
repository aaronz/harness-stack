import { createContext, useContext, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const SettingsContext = createContext();

export function SettingsProvider({ children }) {
  const [settings, setSettings] = useState({
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
  });

  useEffect(() => {
    loadSettings();
  }, []);

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', settings.theme);
  }, [settings.theme]);

  async function loadSettings() {
    try {
      const result = await invoke('read_settings');
      // Map nested editor settings from Rust to flat frontend structure
      setSettings(prev => ({
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
      }));
    } catch (e) {
      console.log('Using default settings');
    }
  }

  async function saveSettings() {
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
      };
      await invoke('write_settings', { settings: settingsToSave });
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  }

  function toggleTheme() {
    setSettings(prev => {
      const newSettings = { ...prev, theme: prev.theme === 'light' ? 'dark' : 'light' };
      return newSettings;
    });
    saveSettings();
  }

  function toggleFocusMode() {
    setSettings(prev => {
      const newSettings = { ...prev, focusMode: !prev.focusMode };
      return newSettings;
    });
    saveSettings();
  }

  function toggleTypewriterMode() {
    setSettings(prev => {
      const newSettings = { ...prev, typewriterMode: !prev.typewriterMode };
      return newSettings;
    });
    saveSettings();
  }

  function setTypewriterMode(value) {
    setSettings(prev => {
      const newSettings = { ...prev, typewriterMode: value };
      return newSettings;
    });
    saveSettings();
  }

  function toggleOutline() {
    setSettings(prev => {
      const newSettings = { ...prev, outlineVisible: !prev.outlineVisible };
      return newSettings;
    });
    saveSettings();
  }

  function addToRecentFiles(filePath) {
    if (!filePath) return;
    setSettings(prev => {
      const newRecentFiles = [filePath, ...prev.recentFiles.filter(p => p !== filePath)].slice(0, 10);
      const newSettings = { ...prev, recentFiles: newRecentFiles };
      return newSettings;
    });
    saveSettings();
  }

  function clearRecentFiles() {
    setSettings(prev => {
      const newSettings = { ...prev, recentFiles: [] };
      return newSettings;
    });
    saveSettings();
  }

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