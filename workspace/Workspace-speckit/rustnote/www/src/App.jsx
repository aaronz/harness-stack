import { useEffect } from 'react';
import { SettingsProvider, useSettings } from './contexts/SettingsContext';
import { DocumentProvider, useDocument } from './contexts/DocumentContext';
import Toolbar from './components/Toolbar';
import Sidebar from './components/Sidebar';
import Editor from './components/Editor';
import OutlinePanel from './components/OutlinePanel';

function AppContent() {
  const { settings } = useSettings();
  const { createNewDocument, currentDocument } = useDocument();

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', settings.theme);
  }, [settings.theme]);

  useEffect(() => {
    if (!currentDocument) {
      createNewDocument();
    }
  }, []);

  return (
    <div id="app" className="flex h-screen w-screen">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Toolbar />
        <Editor />
      </div>
      <OutlinePanel />
    </div>
  );
}

export default function App() {
  return (
    <SettingsProvider>
      <DocumentProvider>
        <AppContent />
      </DocumentProvider>
    </SettingsProvider>
  );
}