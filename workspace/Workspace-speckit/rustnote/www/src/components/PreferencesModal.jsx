import { useState, useEffect } from 'react';
import { useSettings } from '../contexts/SettingsContext';

const FONT_FAMILIES = ['System', 'Serif', 'Monospace'];
const MIN_FONT_SIZE = 12;
const MAX_FONT_SIZE = 32;
const MIN_LINE_HEIGHT = 1.0;
const MAX_LINE_HEIGHT = 2.5;
const MIN_CONTENT_WIDTH = 500;
const MAX_CONTENT_WIDTH = 1400;
const MIN_TAB_SIZE = 2;
const MAX_TAB_SIZE = 8;

export default function PreferencesModal({ isVisible, onClose }) {
  const { settings, setSettings } = useSettings();
  
  const [localSettings, setLocalSettings] = useState({
    theme: 'light',
    autoSave: true,
    autoSaveInterval: 30000,
    focusMode: false,
    typewriterMode: false,
    outlineVisible: false,
    fontSize: 16,
    fontFamily: 'System',
    lineHeight: 1.6,
    tabSize: 4,
    contentWidth: 720,
  });

  useEffect(() => {
    if (isVisible) {
      setLocalSettings({
        theme: settings.theme,
        autoSave: settings.autoSave,
        autoSaveInterval: settings.autoSaveInterval,
        focusMode: settings.focusMode,
        typewriterMode: settings.typewriterMode,
        outlineVisible: settings.outlineVisible,
        fontSize: settings.fontSize,
        fontFamily: settings.fontFamily,
        lineHeight: settings.lineHeight,
        tabSize: settings.tabSize || 4,
        contentWidth: settings.contentWidth,
      });
    }
  }, [isVisible, settings]);

  if (!isVisible) {
    return null;
  }

  const handleSave = () => {
    setSettings(prev => ({
      ...prev,
      ...localSettings,
    }));
    onClose();
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Escape') {
      onClose();
    }
  };

  const autoSaveIntervalSeconds = localSettings.autoSaveInterval / 1000;

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
        className="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-hidden flex flex-col"
        style={{
          backgroundColor: 'var(--bg-primary)',
          color: 'var(--text-primary)',
        }}
      >
        <div
          className="flex items-center justify-between px-6 py-4 border-b shrink-0"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <h2 id="preferences-title" className="text-lg font-semibold">Preferences</h2>
          <button
            id="btn-preferences-close"
            onClick={onClose}
            className="text-2xl leading-none opacity-60 hover:opacity-100"
            style={{ color: 'var(--text-primary)' }}
          >
            ×
          </button>
        </div>

        <div className="flex-1 overflow-y-auto px-6 py-4">
          <section className="mb-6">
            <h3 className="text-sm font-semibold mb-3 uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>
              Appearance
            </h3>
            
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2" style={{ color: 'var(--text-primary)' }}>
                Theme
              </label>
              <div className="flex gap-2">
                <button
                  id="btn-theme-light"
                  onClick={() => setLocalSettings(s => ({ ...s, theme: 'light' }))}
                  className={`px-4 py-2 text-sm border rounded cursor-pointer transition-colors ${
                    localSettings.theme === 'light' ? 'ring-2' : ''
                  }`}
                  style={{
                    backgroundColor: localSettings.theme === 'light' ? 'var(--accent-color)' : 'var(--bg-primary)',
                    borderColor: localSettings.theme === 'light' ? 'var(--accent-color)' : 'var(--border-color)',
                    color: localSettings.theme === 'light' ? 'white' : 'var(--text-primary)',
                    ringColor: 'var(--accent-color)',
                  }}
                >
                  Light
                </button>
                <button
                  id="btn-theme-dark"
                  onClick={() => setLocalSettings(s => ({ ...s, theme: 'dark' }))}
                  className={`px-4 py-2 text-sm border rounded cursor-pointer transition-colors ${
                    localSettings.theme === 'dark' ? 'ring-2' : ''
                  }`}
                  style={{
                    backgroundColor: localSettings.theme === 'dark' ? 'var(--accent-color)' : 'var(--bg-primary)',
                    borderColor: localSettings.theme === 'dark' ? 'var(--accent-color)' : 'var(--border-color)',
                    color: localSettings.theme === 'dark' ? 'white' : 'var(--text-primary)',
                    ringColor: 'var(--accent-color)',
                  }}
                >
                  Dark
                </button>
              </div>
            </div>

            <div className="mb-4">
              <label 
                htmlFor="font-family-select"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                Font Family
              </label>
              <select
                id="font-family-select"
                value={localSettings.fontFamily}
                onChange={(e) => setLocalSettings(s => ({ ...s, fontFamily: e.target.value }))}
                className="w-full px-3 py-2 text-sm border rounded cursor-pointer"
                style={{
                  backgroundColor: 'var(--bg-primary)',
                  borderColor: 'var(--border-color)',
                  color: 'var(--text-primary)',
                }}
              >
                {FONT_FAMILIES.map(font => (
                  <option key={font} value={font}>{font}</option>
                ))}
              </select>
            </div>

            <div className="mb-4">
              <label 
                htmlFor="font-size-slider"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                Font Size: {localSettings.fontSize}px
              </label>
              <input
                id="font-size-slider"
                type="range"
                min={MIN_FONT_SIZE}
                max={MAX_FONT_SIZE}
                value={localSettings.fontSize}
                onChange={(e) => setLocalSettings(s => ({ ...s, fontSize: parseInt(e.target.value, 10) }))}
                className="w-full h-2 rounded-full appearance-none cursor-pointer"
                style={{
                  backgroundColor: 'var(--bg-secondary)',
                  accentColor: 'var(--accent-color)',
                }}
              />
              <div className="flex justify-between text-xs mt-1" style={{ color: 'var(--text-secondary)' }}>
                <span>{MIN_FONT_SIZE}px</span>
                <span>{MAX_FONT_SIZE}px</span>
              </div>
            </div>

            <div className="mb-4">
              <label 
                htmlFor="line-height-slider"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                Line Height: {localSettings.lineHeight.toFixed(1)}
              </label>
              <input
                id="line-height-slider"
                type="range"
                min={MIN_LINE_HEIGHT * 10}
                max={MAX_LINE_HEIGHT * 10}
                step={1}
                value={localSettings.lineHeight * 10}
                onChange={(e) => setLocalSettings(s => ({ ...s, lineHeight: parseInt(e.target.value, 10) / 10 }))}
                className="w-full h-2 rounded-full appearance-none cursor-pointer"
                style={{
                  backgroundColor: 'var(--bg-secondary)',
                  accentColor: 'var(--accent-color)',
                }}
              />
              <div className="flex justify-between text-xs mt-1" style={{ color: 'var(--text-secondary)' }}>
                <span>{MIN_LINE_HEIGHT}</span>
                <span>{MAX_LINE_HEIGHT}</span>
              </div>
            </div>

            <div className="mb-4">
              <label 
                htmlFor="content-width-slider"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                Content Width: {localSettings.contentWidth}px
              </label>
              <input
                id="content-width-slider"
                type="range"
                min={MIN_CONTENT_WIDTH}
                max={MAX_CONTENT_WIDTH}
                step={10}
                value={localSettings.contentWidth}
                onChange={(e) => setLocalSettings(s => ({ ...s, contentWidth: parseInt(e.target.value, 10) }))}
                className="w-full h-2 rounded-full appearance-none cursor-pointer"
                style={{
                  backgroundColor: 'var(--bg-secondary)',
                  accentColor: 'var(--accent-color)',
                }}
              />
              <div className="flex justify-between text-xs mt-1" style={{ color: 'var(--text-secondary)' }}>
                <span>{MIN_CONTENT_WIDTH}px</span>
                <span>{MAX_CONTENT_WIDTH}px</span>
              </div>
            </div>
          </section>

          <section className="mb-6">
            <h3 className="text-sm font-semibold mb-3 uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>
              Editor
            </h3>

            <div className="mb-4">
              <label 
                htmlFor="tab-size-slider"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                Tab Size: {localSettings.tabSize} spaces
              </label>
              <input
                id="tab-size-slider"
                type="range"
                min={MIN_TAB_SIZE}
                max={MAX_TAB_SIZE}
                value={localSettings.tabSize}
                onChange={(e) => setLocalSettings(s => ({ ...s, tabSize: parseInt(e.target.value, 10) }))}
                className="w-full h-2 rounded-full appearance-none cursor-pointer"
                style={{
                  backgroundColor: 'var(--bg-secondary)',
                  accentColor: 'var(--accent-color)',
                }}
              />
              <div className="flex justify-between text-xs mt-1" style={{ color: 'var(--text-secondary)' }}>
                <span>{MIN_TAB_SIZE}</span>
                <span>{MAX_TAB_SIZE}</span>
              </div>
            </div>

            <div className="mb-4">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  id="focus-mode-toggle"
                  type="checkbox"
                  checked={localSettings.focusMode}
                  onChange={(e) => setLocalSettings(s => ({ ...s, focusMode: e.target.checked }))}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  Focus Mode (dims non-active paragraphs)
                </span>
              </label>
            </div>

            <div className="mb-4">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  id="typewriter-mode-toggle"
                  type="checkbox"
                  checked={localSettings.typewriterMode}
                  onChange={(e) => setLocalSettings(s => ({ ...s, typewriterMode: e.target.checked }))}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  Typewriter Mode (keeps cursor centered)
                </span>
              </label>
            </div>

            <div className="mb-4">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  id="outline-toggle"
                  type="checkbox"
                  checked={localSettings.outlineVisible}
                  onChange={(e) => setLocalSettings(s => ({ ...s, outlineVisible: e.target.checked }))}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  Show Outline Panel
                </span>
              </label>
            </div>
          </section>

          <section className="mb-6">
            <h3 className="text-sm font-semibold mb-3 uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>
              Auto-save
            </h3>

            <div className="mb-4">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  id="auto-save-toggle"
                  type="checkbox"
                  checked={localSettings.autoSave}
                  onChange={(e) => setLocalSettings(s => ({ ...s, autoSave: e.target.checked }))}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  Enable Auto-save
                </span>
              </label>
            </div>

            <div className="mb-4">
              <label 
                htmlFor="auto-save-interval-slider"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                Auto-save Interval: {autoSaveIntervalSeconds} seconds
              </label>
              <input
                id="auto-save-interval-slider"
                type="range"
                min={10}
                max={300}
                value={autoSaveIntervalSeconds}
                onChange={(e) => setLocalSettings(s => ({ ...s, autoSaveInterval: parseInt(e.target.value, 10) * 1000 }))}
                className="w-full h-2 rounded-full appearance-none cursor-pointer"
                style={{
                  backgroundColor: 'var(--bg-secondary)',
                  accentColor: 'var(--accent-color)',
                }}
                disabled={!localSettings.autoSave}
              />
              <div className="flex justify-between text-xs mt-1" style={{ color: 'var(--text-secondary)' }}>
                <span>10s</span>
                <span>300s (5min)</span>
              </div>
            </div>
          </section>

          <section className="mb-6">
            <h3 className="text-sm font-semibold mb-3 uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>
              Recent Files
            </h3>
            <div 
              id="recent-files-list"
              className="border rounded p-3 text-sm"
              style={{ 
                borderColor: 'var(--border-color)',
                backgroundColor: 'var(--bg-secondary)',
                color: 'var(--text-primary)',
              }}
            >
              {settings.recentFiles && settings.recentFiles.length > 0 ? (
                <ul className="space-y-1">
                  {settings.recentFiles.slice(0, 10).map((file, index) => (
                    <li key={index} className="truncate text-xs" style={{ color: 'var(--text-secondary)' }}>
                      {file}
                    </li>
                  ))}
                </ul>
              ) : (
                <span style={{ color: 'var(--text-secondary)' }}>No recent files</span>
              )}
            </div>
          </section>
        </div>

        <div
          className="flex justify-end gap-2 px-6 py-4 border-t shrink-0"
          style={{ borderColor: 'var(--border-color)' }}
        >
          <button
            id="btn-preferences-cancel"
            onClick={onClose}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          >
            Cancel
          </button>
          <button
            id="btn-preferences-save"
            onClick={handleSave}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors"
            style={{
              backgroundColor: 'var(--accent-color)',
              borderColor: 'var(--accent-color)',
              color: 'white',
            }}
          >
            Save
          </button>
        </div>
      </div>
    </div>
  );
}
