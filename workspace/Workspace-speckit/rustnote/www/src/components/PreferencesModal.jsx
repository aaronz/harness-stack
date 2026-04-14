import { useState, useEffect, useRef, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
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
  const { t } = useTranslation();
  const { settings, setSettings } = useSettings();
  const modalRef = useRef(null);
  const closeButtonRef = useRef(null);
  const previousActiveElement = useRef(null);
  const hasChangesRef = useRef(false);
  
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

  const handleChange = useCallback((key, value) => {
    hasChangesRef.current = true;
    setLocalSettings(prev => ({ ...prev, [key]: value }));
    setSettings(prev => ({ ...prev, [key]: value }));
  }, [setSettings]);

  useEffect(() => {
    if (isVisible) {
      previousActiveElement.current = document.activeElement;
      hasChangesRef.current = false;
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
      setTimeout(() => {
        closeButtonRef.current?.focus();
      }, 0);
    } else if (previousActiveElement.current) {
      previousActiveElement.current.focus();
    }
  }, [isVisible, settings]);

  if (!isVisible) {
    return null;
  }

  const handleSave = useCallback(() => {
    setSettings(localSettings);
    hasChangesRef.current = false;
    onClose();
  }, [localSettings, setSettings, onClose]);

  const handleClose = useCallback(() => {
    if (hasChangesRef.current) {
      setSettings(localSettings);
      hasChangesRef.current = false;
    }
    onClose();
  }, [localSettings, setSettings, onClose]);

  const handleKeyDown = useCallback((e) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      handleClose();
    }
    if (e.key === 'Tab') {
      const focusableElements = modalRef.current?.querySelectorAll(
        'button:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])'
      );
      if (!focusableElements || focusableElements.length === 0) return;
      
      const firstElement = focusableElements[0];
      const lastElement = focusableElements[focusableElements.length - 1];
      
      if (e.shiftKey && document.activeElement === firstElement) {
        e.preventDefault();
        lastElement.focus();
      } else if (!e.shiftKey && document.activeElement === lastElement) {
        e.preventDefault();
        firstElement.focus();
      }
    }
  }, [handleClose]);

  const autoSaveIntervalSeconds = localSettings.autoSaveInterval / 1000;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      onKeyDown={handleKeyDown}
      role="dialog"
      aria-modal="true"
      aria-labelledby="preferences-title"
    >
      <div
        className="absolute inset-0 bg-black/50"
        onClick={handleClose}
      />

      <div
        ref={modalRef}
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
          <h2 id="preferences-title" className="text-lg font-semibold">{t('preferences.title')}</h2>
          <button
            ref={closeButtonRef}
            id="btn-preferences-close"
            onClick={handleClose}
            className="text-2xl leading-none opacity-60 hover:opacity-100 focus:outline-none focus:ring-2"
            style={{ color: 'var(--text-primary)' }}
            aria-label={t('preferences.close')}
          >
            ×
          </button>
        </div>

        <div className="flex-1 overflow-y-auto px-6 py-4">
          <section className="mb-6">
            <h3 className="text-sm font-semibold mb-3 uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>
              {t('preferences.appearance')}
            </h3>
            
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2" style={{ color: 'var(--text-primary)' }}>
                {t('preferences.theme')}
              </label>
              <div className="flex gap-2">
                <button
                  id="btn-theme-light"
                  onClick={() => handleChange('theme', 'light')}
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
                  {t('preferences.light')}
                </button>
                <button
                  id="btn-theme-dark"
                  onClick={() => handleChange('theme', 'dark')}
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
                  {t('preferences.dark')}
                </button>
              </div>
            </div>

            <div className="mb-4">
              <label 
                htmlFor="font-family-select"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                {t('preferences.fontFamily')}
              </label>
              <select
                id="font-family-select"
                value={localSettings.fontFamily}
                onChange={(e) => handleChange('fontFamily', e.target.value)}
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
                {t('preferences.fontSize')}: {localSettings.fontSize}px
              </label>
              <input
                id="font-size-slider"
                type="range"
                min={MIN_FONT_SIZE}
                max={MAX_FONT_SIZE}
                value={localSettings.fontSize}
                onChange={(e) => handleChange('fontSize', parseInt(e.target.value, 10))}
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
                {t('preferences.lineHeight')}: {localSettings.lineHeight.toFixed(1)}
              </label>
              <input
                id="line-height-slider"
                type="range"
                min={MIN_LINE_HEIGHT * 10}
                max={MAX_LINE_HEIGHT * 10}
                step={1}
                value={localSettings.lineHeight * 10}
                onChange={(e) => handleChange('lineHeight', parseInt(e.target.value, 10) / 10)}
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
                {t('preferences.contentWidth')}: {localSettings.contentWidth}px
              </label>
              <input
                id="content-width-slider"
                type="range"
                min={MIN_CONTENT_WIDTH}
                max={MAX_CONTENT_WIDTH}
                step={10}
                value={localSettings.contentWidth}
                onChange={(e) => handleChange('contentWidth', parseInt(e.target.value, 10))}
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
              {t('preferences.editor')}
            </h3>

            <div className="mb-4">
              <label 
                htmlFor="tab-size-slider"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                {t('preferences.tabSize')}: {localSettings.tabSize} {t('preferences.spaces')}
              </label>
              <input
                id="tab-size-slider"
                type="range"
                min={MIN_TAB_SIZE}
                max={MAX_TAB_SIZE}
                value={localSettings.tabSize}
                onChange={(e) => handleChange('tabSize', parseInt(e.target.value, 10))}
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
                  onChange={(e) => handleChange('focusMode', e.target.checked)}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  {t('preferences.focusMode')}
                </span>
              </label>
            </div>

            <div className="mb-4">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  id="typewriter-mode-toggle"
                  type="checkbox"
                  checked={localSettings.typewriterMode}
                  onChange={(e) => handleChange('typewriterMode', e.target.checked)}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  {t('preferences.typewriterMode')}
                </span>
              </label>
            </div>

            <div className="mb-4">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  id="outline-toggle"
                  type="checkbox"
                  checked={localSettings.outlineVisible}
                  onChange={(e) => handleChange('outlineVisible', e.target.checked)}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  {t('preferences.showOutlinePanel')}
                </span>
              </label>
            </div>
          </section>

          <section className="mb-6">
            <h3 className="text-sm font-semibold mb-3 uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>
              {t('preferences.autosave')}
            </h3>

            <div className="mb-4">
              <label className="flex items-center gap-3 cursor-pointer">
                <input
                  id="auto-save-toggle"
                  type="checkbox"
                  checked={localSettings.autoSave}
                  onChange={(e) => handleChange('autoSave', e.target.checked)}
                  className="w-4 h-4 rounded"
                  style={{ accentColor: 'var(--accent-color)' }}
                />
                <span className="text-sm" style={{ color: 'var(--text-primary)' }}>
                  {t('preferences.enableAutosave')}
                </span>
              </label>
            </div>

            <div className="mb-4">
              <label 
                htmlFor="auto-save-interval-slider"
                className="block text-sm font-medium mb-2" 
                style={{ color: 'var(--text-primary)' }}
              >
                {t('preferences.autosaveInterval')}: {autoSaveIntervalSeconds} seconds
              </label>
              <input
                id="auto-save-interval-slider"
                type="range"
                min={10}
                max={300}
                value={autoSaveIntervalSeconds}
                onChange={(e) => handleChange('autoSaveInterval', parseInt(e.target.value, 10) * 1000)}
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
              {t('preferences.recentFiles')}
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
                <span style={{ color: 'var(--text-secondary)' }}>{t('preferences.noRecentFiles')}</span>
              )}
            </div>
          </section>

          <section className="mb-6">
            <h3 className="text-sm font-semibold mb-3 uppercase tracking-wide" style={{ color: 'var(--text-secondary)' }}>
              {t('preferences.recentFolders')}
            </h3>
            <div 
              id="recent-folders-list"
              className="border rounded p-3 text-sm"
              style={{ 
                borderColor: 'var(--border-color)',
                backgroundColor: 'var(--bg-secondary)',
                color: 'var(--text-primary)',
              }}
            >
              {settings.recentFolders && settings.recentFolders.length > 0 ? (
                <ul className="space-y-1">
                  {settings.recentFolders.slice(0, 10).map((folder, index) => (
                    <li key={index} className="truncate text-xs" style={{ color: 'var(--text-secondary)' }}>
                      {folder}
                    </li>
                  ))}
                </ul>
              ) : (
                <span style={{ color: 'var(--text-secondary)' }}>{t('preferences.noRecentFolders')}</span>
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
            onClick={handleClose}
            className="px-4 py-2 text-sm border rounded cursor-pointer transition-colors"
            style={{
              backgroundColor: 'var(--bg-primary)',
              borderColor: 'var(--border-color)',
              color: 'var(--text-primary)',
            }}
          >
            {t('preferences.cancel')}
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
            {t('preferences.save')}
          </button>
        </div>
      </div>
    </div>
  );
}
