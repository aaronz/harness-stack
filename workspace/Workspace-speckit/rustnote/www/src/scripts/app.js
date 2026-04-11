// Global state
let currentDocument = null;
let workspace = null;
let settings = {
    theme: 'light',
    autoSave: true,
    autoSaveInterval: 30000,
    focusMode: false,
    typewriterMode: false,
    outlineVisible: false
};

async function init() {
    await loadSettings();
    applyTheme();
    setupEventListeners();
    
    const hasRecovery = await checkForRecovery();
    if (!hasRecovery) {
        createNewDocument();
    }
    
    setInterval(saveSnapshot, 60000);
    setInterval(checkForExternalChanges, 10000);
    await cleanupOldSnapshots();
}

async function checkForRecovery() {
    try {
        const snapshots = await window.__TAURI__.core.invoke('list_recovery_snapshots');
        if (snapshots && snapshots.length > 0) {
            const choice = confirm(`Found ${snapshots.length} recovery snapshot(s). Would you like to restore the most recent?`);
            if (choice) {
                await restoreSnapshot(snapshots[0].id);
                return true;
            }
        }
    } catch (e) {
        console.log('No recovery snapshots found');
    }
    return false;
}

async function saveSnapshot() {
    if (!currentDocument || !currentDocument.content) return;
    
    try {
        await window.__TAURI__.core.invoke('save_recovery_snapshot', {
            filePath: currentDocument.filePath,
            content: currentDocument.content,
            cursorOffset: editor.cursorOffset || 0,
            title: currentDocument.title || 'Untitled'
        });
    } catch (e) {
        console.error('Failed to save snapshot:', e);
    }
}

async function restoreSnapshot(id) {
    try {
        const data = await window.__TAURI__.core.invoke('restore_recovery_snapshot', { id });
        if (data) {
            currentDocument = {
                id: Date.now().toString(),
                title: data.title || 'Recovered',
                content: data.content,
                filePath: data.file_path || null,
                isDirty: false
            };
            editor.setContent(data.content);
            if (data.cursor_offset) {
                editor.setCursorPosition(data.cursor_offset);
            }
            updateTitle();
            showNotification('Document restored from recovery snapshot');
            
            // Delete the snapshot after successful restore
            try {
                await window.__TAURI__.core.invoke('delete_recovery_snapshot', { id });
            } catch (e) {
                console.log('Snapshot cleanup skipped');
            }
        }
    } catch (e) {
        console.error('Failed to restore snapshot:', e);
    }
}

async function cleanupOldSnapshots() {
    try {
        await window.__TAURI__.core.invoke('cleanup_old_snapshots', { maxAgeHours: 72 });
    } catch (e) {
        console.error('Failed to cleanup snapshots:', e);
    }
}

async function loadSettings() {
    try {
        const result = await window.__TAURI__.core.invoke('read_settings');
        settings = { ...settings, ...result };
    } catch (e) {
        console.log('Using default settings');
    }
}

function applyTheme() {
    document.documentElement.setAttribute('data-theme', settings.theme);
}

function setupEventListeners() {
    document.addEventListener('keydown', handleKeyboard);
    
    if (settings.autoSave) {
        setInterval(autoSave, settings.autoSaveInterval);
    }
}

function handleKeyboard(e) {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const modifier = isMac ? e.metaKey : e.ctrlKey;
    
    if (modifier && e.shiftKey) {
        switch(e.key.toLowerCase()) {
            case 'f':
                e.preventDefault();
                toggleFocusMode();
                break;
            case 't':
                e.preventDefault();
                toggleTypewriterMode();
                break;
            case 'o':
                e.preventDefault();
                toggleOutline();
                break;
        }
        return;
    }
    
    if (modifier && e.key === 's') {
        e.preventDefault();
        saveDocument();
    } else if (modifier && e.key === 'n') {
        e.preventDefault();
        createNewDocument();
    } else if (modifier && e.key === 'o') {
        e.preventDefault();
        openDocument();
    } else if (modifier && e.key === 'b') {
        e.preventDefault();
        formatBold();
    } else if (modifier && e.key === 'i') {
        e.preventDefault();
        formatItalic();
    } else if (modifier && e.key === 'f') {
        e.preventDefault();
        searchManager.show();
    } else if (modifier && e.key === 'h') {
        e.preventDefault();
        searchManager.show();
    }
}

function toggleFocusMode() {
    if (typeof editor.toggleFocusMode === 'function') {
        const newState = editor.toggleFocusMode();
        settings.focusMode = newState;
        document.getElementById('btn-focus')?.classList.toggle('active', newState);
        saveSettings();
    }
}

function toggleTypewriterMode() {
    if (typeof editor.toggleTypewriterMode === 'function') {
        const newState = editor.toggleTypewriterMode();
        settings.typewriterMode = newState;
        document.getElementById('btn-typewriter')?.classList.toggle('active', newState);
        saveSettings();
    }
}

function toggleOutline() {
    if (typeof outlineManager.toggle === 'function') {
        outlineManager.toggle();
        settings.outlineVisible = outlineManager.isVisible;
        saveSettings();
    }
}

async function createNewDocument() {
    try {
        const doc = await window.__TAURI__.core.invoke('create_document', { title: 'Untitled' });
        currentDocument = {
            id: doc.id,
            title: doc.title,
            content: doc.content,
            filePath: doc.file_path || null,
            isDirty: doc.is_dirty
        };
        editor.setContent(doc.content);
        updateTitle();
    } catch (e) {
        console.error('Create document error:', e);
        currentDocument = {
            title: 'Untitled',
            content: '',
            filePath: null,
            isDirty: false
        };
        editor.setContent('');
        updateTitle();
    }
}

async function openDocument() {
    try {
        const result = await window.__TAURI__.dialog.open({
            filters: [{ name: 'Markdown', extensions: ['md', 'markdown'] }]
        });
        
        if (result) {
            if (currentDocument?.filePath) {
                await window.__TAURI__.core.invoke('unwatch_file', { path: currentDocument.filePath });
            }
            
            const doc = await window.__TAURI__.core.invoke('open_document', { path: result });
            currentDocument = {
                id: doc.id,
                title: doc.title,
                content: doc.content,
                filePath: doc.file_path || null,
                isDirty: doc.is_dirty
            };
            editor.setContent(doc.content);
            updateTitle();
            addToRecentFiles(result);
            
            if (currentDocument.filePath) {
                await window.__TAURI__.core.invoke('watch_file', { path: currentDocument.filePath });
            }
        }
    } catch (e) {
        console.error('Open error:', e);
    }
}

async function saveDocument() {
    if (!currentDocument) return;
    
    currentDocument.content = editor.getContent();
    
    try {
        if (currentDocument.filePath) {
            await window.__TAURI__.core.invoke('save_document', {
                path: currentDocument.filePath,
                content: currentDocument.content
            });
            
            await window.__TAURI__.core.invoke('update_watched_file_state', { 
                path: currentDocument.filePath 
            });
        } else {
            await saveDocumentAs();
        }
        
        currentDocument.isDirty = false;
        updateTitle();
    } catch (e) {
        console.error('Save error:', e);
    }
}

async function checkForExternalChanges() {
    if (!currentDocument?.filePath) return;
    
    try {
        const result = await window.__TAURI__.core.invoke('check_external_change', { 
            path: currentDocument.filePath 
        });
        
        if (result && result.has_changed) {
            const choice = confirm('This file has been modified externally. Would you like to reload it?');
            if (choice) {
                await reloadDocument();
            }
        }
    } catch (e) {
        console.error('Check external changes error:', e);
    }
}

async function reloadDocument() {
    if (!currentDocument?.filePath) return;
    
    try {
        const doc = await window.__TAURI__.core.invoke('open_document', { path: currentDocument.filePath });
        currentDocument.content = doc.content;
        editor.setContent(doc.content);
        currentDocument.isDirty = false;
        updateTitle();
        
        await window.__TAURI__.core.invoke('update_watched_file_state', { 
            path: currentDocument.filePath 
        });
        
        showNotification('Document reloaded');
    } catch (e) {
        console.error('Reload error:', e);
    }
}

async function saveDocumentAs() {
    try {
        const result = await window.__TAURI__.dialog.save({
            filters: [{ name: 'Markdown', extensions: ['md'] }],
            defaultPath: currentDocument.title + '.md'
        });
        
        if (result) {
            currentDocument.filePath = result;
            await saveDocument();
            addToRecentFiles(result);
        }
    } catch (e) {
        console.error('Save as error:', e);
    }
}

async function autoSave() {
    if (currentDocument && currentDocument.isDirty && currentDocument.filePath) {
        await saveDocument();
        showNotification('Auto-saved');
    }
}

function updateTitle() {
    const dirty = currentDocument?.isDirty ? '•' : '';
    const title = currentDocument?.title || 'RustNote';
    document.title = `${dirty}${title} - RustNote`;
}

async function addToRecentFiles(path) {
    if (!settings.recentFiles) settings.recentFiles = [];
    settings.recentFiles = [path, ...settings.recentFiles.filter(p => p !== path)].slice(0, 10);
    saveSettings();
}

async function saveSettings() {
    try {
        await window.__TAURI__.core.invoke('write_settings', settings);
    } catch (e) {
        console.error('Failed to save settings:', e);
    }
}

function showNotification(message) {
    console.log(message);
}

async function toggleTheme() {
    settings.theme = settings.theme === 'light' ? 'dark' : 'light';
    applyTheme();
    saveSettings();
}

async function loadWorkspace() {
    try {
        const result = await window.__TAURI__.dialog.open({
            directory: true,
            title: 'Open Folder as Workspace'
        });
        
        if (result) {
            workspace = await window.__TAURI__.core.invoke('list_workspace', { path: result });
            renderFileTree();
        }
    } catch (e) {
        console.error('Workspace error:', e);
    }
}

function renderFileTree() {
    const tree = document.getElementById('file-tree');
    tree.innerHTML = '';
    
    if (!workspace?.files) return;
    
    workspace.files.forEach(file => {
        const div = document.createElement('div');
        div.className = 'file-item';
        div.textContent = file.name;
        div.addEventListener('click', () => openFile(file.path));
        tree.appendChild(div);
    });
}

async function openFile(path) {
    try {
        const doc = await window.__TAURI__.core.invoke('open_document', { path });
        currentDocument = {
            id: doc.id,
            title: doc.title,
            content: doc.content,
            filePath: doc.file_path || null,
            isDirty: doc.is_dirty
        };
        editor.setContent(doc.content);
        updateTitle();
    } catch (e) {
        console.error('Open file error:', e);
    }
}

async function exportHtml() {
    if (!currentDocument) return;
    
    try {
        const result = await window.__TAURI__.dialog.save({
            filters: [{ name: 'HTML', extensions: ['html'] }],
            defaultPath: currentDocument.title + '.html'
        });
        
        if (result) {
            await window.__TAURI__.core.invoke('export_to_html', {
                markdown: editor.getContent(),
                outputPath: result
            });
            showNotification('Exported to HTML');
        }
    } catch (e) {
        console.error('Export error:', e);
    }
}

async function exportPdf() {
    if (!currentDocument) return;
    
    try {
        const result = await window.__TAURI__.dialog.save({
            filters: [{ name: 'PDF', extensions: ['pdf'] }],
            defaultPath: currentDocument.title + '.pdf'
        });
        
        if (result) {
            const printHtml = await window.__TAURI__.core.invoke('get_print_html', {
                markdown: editor.getContent()
            });
            
            const printWindow = window.open('', '_blank', 'width=800,height=600');
            if (printWindow) {
                printWindow.document.write(printHtml);
                printWindow.document.close();
                
                printWindow.onload = async () => {
                    try {
                        await printWindow.print();
                        printWindow.close();
                        showNotification('PDF export completed');
                    } catch (printErr) {
                        console.error('Print error:', printErr);
                        printWindow.print();
                        showNotification('PDF export via print dialog');
                    }
                };
            } else {
                showNotification('Please allow popups for PDF export');
            }
        }
    } catch (e) {
        console.error('Export error:', e);
        showNotification('PDF export failed');
    }
}

document.getElementById('btn-new')?.addEventListener('click', createNewDocument);
document.getElementById('btn-open')?.addEventListener('click', openDocument);
document.getElementById('btn-save')?.addEventListener('click', saveDocument);
document.getElementById('btn-theme')?.addEventListener('click', toggleTheme);
document.getElementById('btn-focus')?.addEventListener('click', toggleFocusMode);
document.getElementById('btn-typewriter')?.addEventListener('click', toggleTypewriterMode);
document.getElementById('btn-outline')?.addEventListener('click', toggleOutline);
document.getElementById('btn-image')?.addEventListener('click', () => editor.insertImage());

document.addEventListener('DOMContentLoaded', init);