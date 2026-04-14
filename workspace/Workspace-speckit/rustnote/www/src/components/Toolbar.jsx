import { useTranslation } from 'react-i18next';
import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';

export default function Toolbar({ onExportClick, onPreferencesClick }) {
  const { t } = useTranslation();
  const { createNewDocument, openDocument, saveDocument, insertImage, isSaving, currentDocument } = useDocument();
  const { settings, toggleTheme, toggleFocusMode, toggleTypewriterMode, toggleOutline } = useSettings();

  const documentTitle = currentDocument?.title || t('app.untitled');
  const isDirty = currentDocument?.isDirty || false;

  return (
    <div
      id="toolbar"
      className="flex items-center gap-2 px-4 py-2 border-b"
      style={{
        backgroundColor: 'var(--bg-secondary)',
        borderColor: 'var(--border-color)',
      }}
    >
      <button
        id="btn-new"
        onClick={createNewDocument}
        className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title={t('toolbar.newTitle')}
      >
        {t('toolbar.new')}
      </button>

      <button
        id="btn-open"
        onClick={openDocument}
        className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title={t('toolbar.openTitle')}
      >
        {t('toolbar.open')}
      </button>

      <button
        id="btn-save"
        onClick={saveDocument}
        disabled={isSaving}
        className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
          opacity: isSaving ? 0.6 : 1,
        }}
        title={t('toolbar.saveTitle')}
      >
        {isSaving ? t('toolbar.saving') : t('toolbar.save')}
      </button>

      <div className="flex-1" />

      <div
        id="document-title"
        className="px-3 py-1.5 text-sm font-medium"
        style={{ color: 'var(--text-primary)' }}
        title={isDirty ? t('app.unsavedChanges') : t('app.documentTitle')}
      >
        {isDirty && <span style={{ color: 'var(--accent-color)' }}>*</span>} {documentTitle}
      </div>

      <div className="flex-1" />

      <button
        id="btn-export"
        onClick={onExportClick}
        className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title={t('toolbar.exportTitle')}
      >
        {t('toolbar.export')}
      </button>

      <button
        id="btn-preferences"
        onClick={onPreferencesClick}
        className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title={t('toolbar.preferencesTitle')}
      >
        ⚙️
      </button>

      <button
        id="btn-image"
        onClick={insertImage}
        className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title={t('toolbar.imageTitle')}
      >
        {t('toolbar.image')}
      </button>

      <button
        id="btn-outline"
        onClick={toggleOutline}
        className={`px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors ${
          settings.outlineVisible ? 'text-white' : ''
        }`}
        style={{
          backgroundColor: settings.outlineVisible ? 'var(--accent-color)' : 'var(--bg-primary)',
          borderColor: settings.outlineVisible ? 'var(--accent-color)' : 'var(--border-color)',
          color: settings.outlineVisible ? 'white' : 'var(--text-primary)',
        }}
        title={t('toolbar.outlineTitle')}
      >
        {t('toolbar.outline')}
      </button>

      <button
        id="btn-focus"
        onClick={toggleFocusMode}
        className={`px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors ${
          settings.focusMode ? 'text-white' : ''
        }`}
        style={{
          backgroundColor: settings.focusMode ? 'var(--accent-color)' : 'var(--bg-primary)',
          borderColor: settings.focusMode ? 'var(--accent-color)' : 'var(--border-color)',
          color: settings.focusMode ? 'white' : 'var(--text-primary)',
        }}
        title={t('toolbar.focusTitle')}
      >
        {t('toolbar.focus')}
      </button>

      <button
        id="btn-typewriter"
        onClick={toggleTypewriterMode}
        className={`px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors ${
          settings.typewriterMode ? 'text-white' : ''
        }`}
        style={{
          backgroundColor: settings.typewriterMode ? 'var(--accent-color)' : 'var(--bg-primary)',
          borderColor: settings.typewriterMode ? 'var(--accent-color)' : 'var(--border-color)',
          color: settings.typewriterMode ? 'white' : 'var(--text-primary)',
        }}
        title={t('toolbar.typewriterTitle')}
      >
        {t('toolbar.typewriter')}
      </button>

      <button
        id="btn-theme"
        onClick={toggleTheme}
        className="px-3 py-1.5 text-sm border rounded cursor-pointer transition-colors"
        style={{
          backgroundColor: 'var(--bg-primary)',
          borderColor: 'var(--border-color)',
          color: 'var(--text-primary)',
        }}
        title={t('toolbar.toggleTheme')}
      >
        {settings.theme === 'light' ? '🌙' : '☀️'}
      </button>
    </div>
  );
}
