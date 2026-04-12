import { useDocument } from '../contexts/DocumentContext';
import { useSettings } from '../contexts/SettingsContext';

export default function Toolbar({ onExportClick }) {
  const { createNewDocument, openDocument, saveDocument, insertImage, isSaving } = useDocument();
  const { settings, toggleTheme, toggleFocusMode, toggleTypewriterMode, toggleOutline } = useSettings();

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
        title="New (Ctrl+N)"
      >
        New
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
        title="Open (Ctrl+O)"
      >
        Open
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
        title="Save (Ctrl+S)"
      >
        {isSaving ? 'Saving...' : 'Save'}
      </button>

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
        title="Export Document"
      >
        Export
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
        title="Insert Image"
      >
        Image
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
        title="Toggle Outline (Ctrl+Shift+O)"
      >
        Outline
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
        title="Focus Mode (Ctrl+Shift+F)"
      >
        Focus
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
        title="Typewriter Mode (Ctrl+Shift+T)"
      >
        Typewriter
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
        title="Toggle Theme"
      >
        {settings.theme === 'light' ? '🌙' : '☀️'}
      </button>
    </div>
  );
}