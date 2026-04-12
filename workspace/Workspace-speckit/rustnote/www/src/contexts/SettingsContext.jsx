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
      setSettings(prev => ({ ...prev, ...result }));
    } catch (e) {
      console.log('Using default settings');
    }
  }

  async function saveSettings() {
    try {
      await invoke('write_settings', { settings });
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

  return (
    <SettingsContext.Provider value={{
      settings,
      toggleTheme,
      toggleFocusMode,
      toggleTypewriterMode,
      setTypewriterMode,
      toggleOutline,
      setSettings,
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