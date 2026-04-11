/**
 * RustNote Recovery Dialog
 * 
 * Provides a UI for selecting and managing recovery snapshots.
 */

class RecoveryDialog {
    constructor() {
        this.dialog = null;
        this.snapshots = [];
    }

    async show() {
        if (this.dialog) return;

        try {
            this.snapshots = await window.__TAURI__.core.invoke('list_recovery_snapshots');
        } catch (e) {
            console.error('Failed to list snapshots:', e);
            this.snapshots = [];
        }

        if (this.snapshots.length === 0) {
            showNotification('No recovery snapshots found');
            return;
        }

        this.dialog = document.createElement('div');
        this.dialog.id = 'recovery-dialog';
        this.dialog.className = 'modal-overlay';
        this.dialog.innerHTML = `
            <div class="modal-content recovery-dialog-content">
                <div class="modal-header">
                    <h2>Recovery Snapshots</h2>
                    <button class="modal-close" id="recovery-close">&times;</button>
                </div>
                <div class="modal-body">
                    <p class="recovery-intro">Select a snapshot to restore. The current document will be saved first.</p>
                    <div class="snapshot-list" id="snapshot-list">
                        ${this.renderSnapshotList()}
                    </div>
                </div>
            </div>
        `;

        document.body.appendChild(this.dialog);

        this.dialog.querySelector('#recovery-close').addEventListener('click', () => this.hide());
        this.dialog.querySelectorAll('.snapshot-item').forEach(item => {
            item.addEventListener('click', () => this.selectSnapshot(item.dataset.id));
        });

        this.dialog.addEventListener('click', (e) => {
            if (e.target === this.dialog) this.hide();
        });
    }

    renderSnapshotList() {
        if (this.snapshots.length === 0) {
            return '<p class="no-snapshots">No snapshots available</p>';
        }

        return this.snapshots.map(snapshot => {
            const date = new Date(snapshot.timestamp);
            const formattedDate = date.toLocaleString();
            const preview = snapshot.content_preview || '';

            return `
                <div class="snapshot-item" data-id="${snapshot.id}">
                    <div class="snapshot-header">
                        <span class="snapshot-title">${this.escapeHtml(snapshot.title || 'Untitled')}</span>
                        <span class="snapshot-date">${formattedDate}</span>
                    </div>
                    <div class="snapshot-preview">${this.escapeHtml(preview)}</div>
                    ${snapshot.file_path ? `<div class="snapshot-path">${this.escapeHtml(snapshot.file_path)}</div>` : ''}
                    <div class="snapshot-actions">
                        <button class="snapshot-restore" data-id="${snapshot.id}">Restore</button>
                        <button class="snapshot-delete" data-id="${snapshot.id}">Delete</button>
                    </div>
                </div>
            `;
        }).join('');
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    async selectSnapshot(id) {
        if (currentDocument?.isDirty && currentDocument?.filePath) {
            await saveDocument();
        }

        await restoreSnapshot(id);
        this.hide();

        this.snapshots = await window.__TAURI__.core.invoke('list_recovery_snapshots');
    }

    async deleteSnapshot(id) {
        if (!confirm('Are you sure you want to delete this snapshot?')) {
            return;
        }

        try {
            await window.__TAURI__.core.invoke('delete_recovery_snapshot', { id });
            this.snapshots = this.snapshots.filter(s => s.id !== id);

            const list = this.dialog.querySelector('#snapshot-list');
            if (list) {
                list.innerHTML = this.renderSnapshotList();
                list.querySelectorAll('.snapshot-item').forEach(item => {
                    item.addEventListener('click', () => this.selectSnapshot(item.dataset.id));
                });
            }

            showNotification('Snapshot deleted');
        } catch (e) {
            console.error('Failed to delete snapshot:', e);
            showNotification('Failed to delete snapshot');
        }
    }

    hide() {
        if (this.dialog) {
            this.dialog.remove();
            this.dialog = null;
        }
    }
}

const recoveryDialog = new RecoveryDialog();

document.addEventListener('DOMContentLoaded', () => {
    const menuBtn = document.getElementById('btn-menu');
    if (menuBtn) {
        menuBtn.addEventListener('click', () => {});
    }
});

window.showRecoveryDialog = () => recoveryDialog.show();