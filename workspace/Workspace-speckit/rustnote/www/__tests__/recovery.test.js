import { describe, it, expect, beforeEach, vi } from 'vitest';

global.window = {
  __TAURI__: {
    core: {
      invoke: vi.fn()
    }
  }
};

class RecoveryDialog {
  constructor() {
    this.dialog = null;
    this.snapshots = [];
  }
  
  escapeHtml(text) {
    if (!text) return '';
    return String(text)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
  }
  
  formatDate(timestamp) {
    const date = new Date(timestamp);
    return date.toLocaleString();
  }
  
  generateSnapshotHtml() {
    if (this.snapshots.length === 0) {
      return '<p class="no-snapshots">No snapshots available</p>';
    }
    
    return this.snapshots.map(snapshot => {
      const formattedDate = this.formatDate(snapshot.timestamp);
      const preview = snapshot.content_preview || '';
      
      return `
        <div class="snapshot-item" data-id="${snapshot.id}">
          <div class="snapshot-header">
            <span class="snapshot-title">${this.escapeHtml(snapshot.title || 'Untitled')}</span>
            <span class="snapshot-date">${formattedDate}</span>
          </div>
          <div class="snapshot-preview">${this.escapeHtml(preview)}</div>
        </div>
      `;
    }).join('');
  }
  
  filterSnapshots(filter) {
    if (!filter) return this.snapshots;
    return this.snapshots.filter(s => 
      s.title?.toLowerCase().includes(filter.toLowerCase()) ||
      s.file_path?.toLowerCase().includes(filter.toLowerCase())
    );
  }
  
  sortSnapshots(sortBy = 'date', descending = true) {
    const sorted = [...this.snapshots];
    
    switch (sortBy) {
      case 'date':
        sorted.sort((a, b) => a.timestamp - b.timestamp);
        break;
      case 'title':
        sorted.sort((a, b) => (a.title || '').localeCompare(b.title || ''));
        break;
      case 'size':
        sorted.sort((a, b) => (a.content_preview?.length || 0) - (b.content_preview?.length || 0));
        break;
    }
    
    return descending ? sorted.reverse() : sorted;
  }
}

describe('RecoveryDialog Unit Tests', () => {
  let recoveryDialog;
  
  beforeEach(() => {
    recoveryDialog = new RecoveryDialog();
    window.__TAURI__.core.invoke.mockReset();
  });
  
  describe('escapeHtml', () => {
    it('escapes HTML special characters', () => {
      expect(recoveryDialog.escapeHtml('<script>')).toBe('&lt;script&gt;');
      expect(recoveryDialog.escapeHtml('a & b')).toBe('a &amp; b');
      expect(recoveryDialog.escapeHtml('"quoted"')).toBe('&quot;quoted&quot;');
      expect(recoveryDialog.escapeHtml("'single'")).toBe('&#39;single&#39;');
    });
    
    it('returns empty string for empty input', () => {
      expect(recoveryDialog.escapeHtml('')).toBe('');
      expect(recoveryDialog.escapeHtml(null)).toBe('');
      expect(recoveryDialog.escapeHtml(undefined)).toBe('');
    });
  });
  
  describe('formatDate', () => {
    it('formats valid timestamp', () => {
      const timestamp = new Date('2024-01-15T10:30:00').getTime();
      const result = recoveryDialog.formatDate(timestamp);
      
      expect(result).toContain('2024');
      expect(result).toContain('15');
    });
    
    it('handles zero timestamp', () => {
      const timestamp = 0;
      const result = recoveryDialog.formatDate(timestamp);
      
      expect(typeof result).toBe('string');
      expect(result.length).toBeGreaterThan(0);
    });
  });
  
  describe('generateSnapshotHtml', () => {
    it('returns no snapshots message when empty', () => {
      recoveryDialog.snapshots = [];
      
      const result = recoveryDialog.generateSnapshotHtml();
      
      expect(result).toContain('no-snapshots');
      expect(result).toContain('No snapshots available');
    });
    
    it('generates HTML for single snapshot', () => {
      recoveryDialog.snapshots = [{
        id: 'snap-1',
        title: 'Test Document',
        timestamp: Date.now(),
        content_preview: 'This is a preview',
        file_path: '/path/to/doc.md'
      }];
      
      const result = recoveryDialog.generateSnapshotHtml();
      
      expect(result).toContain('snapshot-item');
      expect(result).toContain('Test Document');
      expect(result).toContain('This is a preview');
    });
    
    it('handles snapshot without title', () => {
      recoveryDialog.snapshots = [{
        id: 'snap-1',
        timestamp: Date.now(),
        content_preview: 'Preview only'
      }];
      
      const result = recoveryDialog.generateSnapshotHtml();
      
      expect(result).toContain('Untitled');
    });
    
    it('generates HTML for multiple snapshots', () => {
      recoveryDialog.snapshots = [
        { id: 'snap-1', title: 'Doc 1', timestamp: Date.now(), content_preview: 'Preview 1' },
        { id: 'snap-2', title: 'Doc 2', timestamp: Date.now(), content_preview: 'Preview 2' }
      ];
      
      const result = recoveryDialog.generateSnapshotHtml();
      
      expect(result).toContain('Doc 1');
      expect(result).toContain('Doc 2');
    });
  });
  
  describe('filterSnapshots', () => {
    beforeEach(() => {
      recoveryDialog.snapshots = [
        { id: 'snap-1', title: 'Alpha Document', file_path: '/docs/alpha.md' },
        { id: 'snap-2', title: 'Beta Report', file_path: '/reports/beta.md' },
        { id: 'snap-3', title: 'Alpha Beta', file_path: '/docs/combined.md' }
      ];
    });
    
    it('returns all snapshots when filter is empty', () => {
      const result = recoveryDialog.filterSnapshots('');
      expect(result.length).toBe(3);
    });
    
    it('filters by title (case insensitive)', () => {
      const result = recoveryDialog.filterSnapshots('alpha');
      expect(result.length).toBe(2);
    });
    
    it('filters by file path', () => {
      const result = recoveryDialog.filterSnapshots('/docs/');
      expect(result.length).toBe(2);
    });
    
    it('returns empty array when no matches', () => {
      const result = recoveryDialog.filterSnapshots('xyz');
      expect(result.length).toBe(0);
    });
  });
  
  describe('sortSnapshots', () => {
    beforeEach(() => {
      const now = Date.now();
      recoveryDialog.snapshots = [
        { id: 'snap-1', title: 'Zebra', timestamp: now - 1000, content_preview: 'aaaa' },
        { id: 'snap-2', title: 'Apple', timestamp: now - 2000, content_preview: 'ccc' },
        { id: 'snap-3', title: 'Mango', timestamp: now, content_preview: 'bb' }
      ];
    });
    
    it('sorts by date ascending', () => {
      const result = recoveryDialog.sortSnapshots('date', false);
      expect(result[0].title).toBe('Apple');
      expect(result[2].title).toBe('Mango');
    });
    
    it('sorts by date descending', () => {
      const result = recoveryDialog.sortSnapshots('date', true);
      expect(result[0].title).toBe('Mango');
      expect(result[2].title).toBe('Apple');
    });
    
    it('sorts by title alphabetically ascending', () => {
      const result = recoveryDialog.sortSnapshots('title', false);
      expect(result[0].title).toBe('Apple');
      expect(result[2].title).toBe('Zebra');
    });
    
    it('sorts by title alphabetically descending', () => {
      const result = recoveryDialog.sortSnapshots('title', true);
      expect(result[0].title).toBe('Zebra');
      expect(result[2].title).toBe('Apple');
    });
    
    it('sorts by size (preview length)', () => {
      const result = recoveryDialog.sortSnapshots('size', false);
      expect(result[0].content_preview).toBe('bb');
      expect(result[1].content_preview).toBe('ccc');
      expect(result[2].content_preview).toBe('aaaa');
    });
    
    it('sorts by size descending', () => {
      const result = recoveryDialog.sortSnapshots('size', true);
      expect(result[0].content_preview).toBe('aaaa');
      expect(result[2].content_preview).toBe('bb');
    });
    
    it('does not mutate original array', () => {
      const original = [...recoveryDialog.snapshots];
      recoveryDialog.sortSnapshots('date', true);
      
      expect(recoveryDialog.snapshots[0].title).toBe(original[0].title);
    });
  });
});
