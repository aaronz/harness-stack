class SettingsPanel {
    constructor() {
        this.dialog = null;
    }
    
    show() {
        if (this.dialog) return;
        
        this.dialog = document.createElement('div');
        this.dialog.id = 'settings-dialog';
        this.dialog.innerHTML = `
            <h2>Settings</h2>
            <div class="setting">
                <label>Theme</label>
                <select id="setting-theme">
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                </select>
            </div>
            <div class="setting">
                <label>Auto-save</label>
                <input type="checkbox" id="setting-autosave" checked>
            </div>
            <div class="setting">
                <label>Auto-save interval (seconds)</label>
                <input type="number" id="setting-interval" value="30" min="10" max="300">
            </div>
            <div class="setting">
                <label>Font Family</label>
                <select id="setting-font">
                    <option value="System">System</option>
                    <option value="Serif">Serif</option>
                    <option value="Monospace">Monospace</option>
                </select>
            </div>
            <div class="setting">
                <label>Font Size</label>
                <input type="number" id="setting-size" value="16" min="12" max="32">
            </div>
            <div class="buttons">
                <button id="settings-save">Save</button>
                <button id="settings-cancel">Cancel</button>
            </div>
        `;
        
        document.body.appendChild(this.dialog);
        
        this.dialog.querySelector('#setting-theme').value = settings.theme;
        this.dialog.querySelector('#setting-autosave').checked = settings.autoSave;
        this.dialog.querySelector('#setting-interval').value = settings.autoSaveInterval / 1000;
        
        this.dialog.querySelector('#settings-save').addEventListener('click', () => this.save());
        this.dialog.querySelector('#settings-cancel').addEventListener('click', () => this.hide());
    }
    
    async save() {
        settings.theme = this.dialog.querySelector('#setting-theme').value;
        settings.autoSave = this.dialog.querySelector('#setting-autosave').checked;
        settings.autoSaveInterval = parseInt(this.dialog.querySelector('#setting-interval').value) * 1000;
        
        try {
            await window.__TAURI__.core.invoke('write_settings', settings);
        } catch (e) {
            console.error('Failed to save settings:', e);
        }
        
        applyTheme();
        
        if (settings.autoSave) {
            clearInterval(autoSaveInterval);
            autoSaveInterval = setInterval(autoSave, settings.autoSaveInterval);
        }
        
        this.hide();
    }
    
    hide() {
        if (this.dialog) {
            this.dialog.remove();
            this.dialog = null;
        }
    }
}

const settingsPanel = new SettingsPanel();

document.getElementById('btn-theme').addEventListener('contextmenu', (e) => {
    e.preventDefault();
    settingsPanel.show();
});

let autoSaveInterval = null;

function startAutoSave() {
    if (autoSaveInterval) clearInterval(autoSaveInterval);
    if (settings.autoSave) {
        autoSaveInterval = setInterval(autoSave, settings.autoSaveInterval);
    }
}

async function saveSnapshot() {
    if (!currentDocument?.filePath) return;
    
    try {
        const snapshotPath = currentDocument.filePath + '.backup';
        await window.__TAURI__.core.invoke('write_document_content', {
            path: snapshotPath,
            content: editor.getContent()
        });
    } catch (e) {
        console.error('Snapshot error:', e);
    }
}

function startSnapshot() {
    setInterval(() => {
        if (currentDocument?.isDirty && currentDocument?.filePath) {
            saveSnapshot();
        }
    }, 30000);
}

async function checkRecovery() {
    try {
        const recentFiles = settings.recentFiles || [];
        for (const path of recentFiles) {
            const backupPath = path + '.backup';
            console.log('Checking recovery for:', backupPath);
        }
    } catch (e) {
        console.error('Recovery check error:', e);
    }
}

function setupExportMenu() {
    const toolbar = document.getElementById('toolbar');
    
    const exportBtn = document.createElement('button');
    exportBtn.id = 'btn-export';
    exportBtn.textContent = 'Export';
    exportBtn.style.marginLeft = '8px';
    
    exportBtn.addEventListener('click', (e) => {
        const menu = document.createElement('div');
        menu.style.cssText = `
            position: absolute;
            background: var(--bg-primary);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            padding: 8px 0;
            min-width: 120px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            z-index: 1000;
        `;
        
        const htmlItem = document.createElement('div');
        htmlItem.textContent = 'Export HTML';
        htmlItem.style.cssText = 'padding: 8px 16px; cursor: pointer;';
        htmlItem.addEventListener('click', () => { exportHtml(); menu.remove(); });
        
        const pdfItem = document.createElement('div');
        pdfItem.textContent = 'Export PDF';
        pdfItem.style.cssText = 'padding: 8px 16px; cursor: pointer;';
        pdfItem.addEventListener('click', () => { exportPdf(); menu.remove(); });
        
        menu.appendChild(htmlItem);
        menu.appendChild(pdfItem);
        
        const rect = exportBtn.getBoundingClientRect();
        menu.style.left = rect.left + 'px';
        menu.style.top = (rect.bottom + 4) + 'px';
        
        document.body.appendChild(menu);
        
        const closeMenu = (ev) => {
            if (!menu.contains(ev.target)) {
                menu.remove();
                document.removeEventListener('click', closeMenu);
            }
        };
        setTimeout(() => document.addEventListener('click', closeMenu), 0);
    });
    
    toolbar.appendChild(exportBtn);
}

function initAdditionalFeatures() {
    startAutoSave();
    startSnapshot();
    checkRecovery();
    setupExportMenu();
}

const originalInit = init;
init = async function() {
    await originalInit();
    initAdditionalFeatures();
};