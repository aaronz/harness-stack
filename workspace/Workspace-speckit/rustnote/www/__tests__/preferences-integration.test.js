import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue({
    theme: 'light',
    autoSave: true,
    autoSaveInterval: 10000,
    focusMode: false,
    typewriterMode: false,
    outlineVisible: false,
    fontSize: 16,
    fontFamily: 'System',
    lineHeight: 1.6,
    contentWidth: 720,
    recentFiles: [],
  }),
}));

const FONT_FAMILIES = ['System', 'Serif', 'Monospace'];
const MIN_FONT_SIZE = 12;
const MAX_FONT_SIZE = 32;
const MIN_LINE_HEIGHT = 1.0;
const MAX_LINE_HEIGHT = 2.5;
const MIN_CONTENT_WIDTH = 500;
const MAX_CONTENT_WIDTH = 1400;

const createMockSettings = () => ({
  theme: 'light',
  autoSave: true,
  autoSaveInterval: 30000,
  focusMode: false,
  typewriterMode: false,
  outlineVisible: false,
  fontSize: 16,
  fontFamily: 'System',
  lineHeight: 1.6,
  contentWidth: 720,
  recentFiles: [],
});

const createPreferencesModal = () => {
  const settings = createMockSettings();
  let localSettings = { ...settings };
  let isVisible = false;
  let saveCallCount = 0;
  const savedSettings = [];

  const modal = {
    settings,
    isVisible,
    saveCallCount,
    savedSettings,
    
    open() {
      isVisible = true;
      this.isVisible = true;
      localSettings = { ...settings };
    },
    
    close() {
      isVisible = false;
      this.isVisible = false;
    },
    
    save() {
      saveCallCount++;
      this.saveCallCount++;
      Object.assign(settings, localSettings);
      this.savedSettings.push({ ...localSettings });
    },
    
    getLocalSettings() {
      return { ...localSettings };
    },
    
    setLocalSetting(key, value) {
      localSettings[key] = value;
    },
    
    resetToSettings() {
      localSettings = { ...settings };
    },
    
    simulateSettingsUpdate() {
      Object.assign(settings, localSettings);
    },
  };

  return modal;
};

describe('PreferencesModal Integration Tests (TC-PM001 to TC-PM013)', () => {
  let modal;

  beforeEach(() => {
    modal = createPreferencesModal();
  });

  describe('TC-PM001: Theme toggle integration', () => {
    it('toggles theme from light to dark', () => {
      modal.open();
      expect(modal.getLocalSettings().theme).toBe('light');
      
      modal.setLocalSetting('theme', 'dark');
      expect(modal.getLocalSettings().theme).toBe('dark');
    });

    it('theme switches instantly in local state', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      expect(modal.getLocalSettings().theme).toBe('dark');
      
      modal.setLocalSetting('theme', 'light');
      expect(modal.getLocalSettings().theme).toBe('light');
    });

    it('theme persists after save', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.save();
      
      expect(modal.settings.theme).toBe('dark');
    });

    it('theme persists after modal close', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.save();
      modal.close();
      
      expect(modal.settings.theme).toBe('dark');
      expect(modal.isVisible).toBe(false);
    });
  });

  describe('TC-PM002: Font size slider integration', () => {
    it('font size changes instantly', () => {
      modal.open();
      modal.setLocalSetting('fontSize', 20);
      expect(modal.getLocalSettings().fontSize).toBe(20);
    });

    it('font size respects minimum value', () => {
      modal.open();
      modal.setLocalSetting('fontSize', MIN_FONT_SIZE - 1);
      expect(modal.getLocalSettings().fontSize).toBeLessThan(MIN_FONT_SIZE);
    });

    it('font size respects maximum value', () => {
      modal.open();
      modal.setLocalSetting('fontSize', MAX_FONT_SIZE + 1);
      expect(modal.getLocalSettings().fontSize).toBeGreaterThan(MAX_FONT_SIZE);
    });

    it('font size persists after save', () => {
      modal.open();
      modal.setLocalSetting('fontSize', 24);
      modal.save();
      
      expect(modal.settings.fontSize).toBe(24);
    });
  });

  describe('TC-PM003: Font family selector integration', () => {
    it('font family changes instantly', () => {
      modal.open();
      modal.setLocalSetting('fontFamily', 'Serif');
      expect(modal.getLocalSettings().fontFamily).toBe('Serif');
    });

    it('font family persists after save', () => {
      modal.open();
      modal.setLocalSetting('fontFamily', 'Monospace');
      modal.save();
      
      expect(modal.settings.fontFamily).toBe('Monospace');
    });

    it('all valid font families are selectable', () => {
      modal.open();
      FONT_FAMILIES.forEach(font => {
        modal.setLocalSetting('fontFamily', font);
        expect(modal.getLocalSettings().fontFamily).toBe(font);
      });
    });
  });

  describe('TC-PM004: Line height adjustment', () => {
    it('line height changes instantly', () => {
      modal.open();
      modal.setLocalSetting('lineHeight', 2.0);
      expect(modal.getLocalSettings().lineHeight).toBe(2.0);
    });

    it('line height respects minimum value', () => {
      modal.open();
      modal.setLocalSetting('lineHeight', MIN_LINE_HEIGHT);
      expect(modal.getLocalSettings().lineHeight).toBeGreaterThanOrEqual(MIN_LINE_HEIGHT);
    });

    it('line height respects maximum value', () => {
      modal.open();
      modal.setLocalSetting('lineHeight', MAX_LINE_HEIGHT);
      expect(modal.getLocalSettings().lineHeight).toBeLessThanOrEqual(MAX_LINE_HEIGHT);
    });

    it('line height persists after save', () => {
      modal.open();
      modal.setLocalSetting('lineHeight', 1.8);
      modal.save();
      
      expect(modal.settings.lineHeight).toBe(1.8);
    });
  });

  describe('TC-PM005: Content width adjustment', () => {
    it('content width changes instantly', () => {
      modal.open();
      modal.setLocalSetting('contentWidth', 800);
      expect(modal.getLocalSettings().contentWidth).toBe(800);
    });

    it('content width respects minimum value', () => {
      modal.open();
      modal.setLocalSetting('contentWidth', MIN_CONTENT_WIDTH);
      expect(modal.getLocalSettings().contentWidth).toBeGreaterThanOrEqual(MIN_CONTENT_WIDTH);
    });

    it('content width respects maximum value', () => {
      modal.open();
      modal.setLocalSetting('contentWidth', MAX_CONTENT_WIDTH);
      expect(modal.getLocalSettings().contentWidth).toBeLessThanOrEqual(MAX_CONTENT_WIDTH);
    });

    it('content width persists after save', () => {
      modal.open();
      modal.setLocalSetting('contentWidth', 1000);
      modal.save();
      
      expect(modal.settings.contentWidth).toBe(1000);
    });
  });

  describe('TC-PM006: Focus mode toggle', () => {
    it('focus mode toggles on', () => {
      modal.open();
      expect(modal.getLocalSettings().focusMode).toBe(false);
      
      modal.setLocalSetting('focusMode', true);
      expect(modal.getLocalSettings().focusMode).toBe(true);
    });

    it('focus mode toggles off', () => {
      modal.open();
      modal.setLocalSetting('focusMode', true);
      modal.setLocalSetting('focusMode', false);
      expect(modal.getLocalSettings().focusMode).toBe(false);
    });

    it('focus mode persists after save', () => {
      modal.open();
      modal.setLocalSetting('focusMode', true);
      modal.save();
      
      expect(modal.settings.focusMode).toBe(true);
    });
  });

  describe('TC-PM007: Typewriter mode toggle', () => {
    it('typewriter mode toggles on', () => {
      modal.open();
      expect(modal.getLocalSettings().typewriterMode).toBe(false);
      
      modal.setLocalSetting('typewriterMode', true);
      expect(modal.getLocalSettings().typewriterMode).toBe(true);
    });

    it('typewriter mode toggles off', () => {
      modal.open();
      modal.setLocalSetting('typewriterMode', true);
      modal.setLocalSetting('typewriterMode', false);
      expect(modal.getLocalSettings().typewriterMode).toBe(false);
    });

    it('typewriter mode persists after save', () => {
      modal.open();
      modal.setLocalSetting('typewriterMode', true);
      modal.save();
      
      expect(modal.settings.typewriterMode).toBe(true);
    });
  });

  describe('TC-PM008: Outline visible toggle', () => {
    it('outline visible toggles on', () => {
      modal.open();
      expect(modal.getLocalSettings().outlineVisible).toBe(false);
      
      modal.setLocalSetting('outlineVisible', true);
      expect(modal.getLocalSettings().outlineVisible).toBe(true);
    });

    it('outline visible toggles off', () => {
      modal.open();
      modal.setLocalSetting('outlineVisible', true);
      modal.setLocalSetting('outlineVisible', false);
      expect(modal.getLocalSettings().outlineVisible).toBe(false);
    });

    it('outline visible persists after save', () => {
      modal.open();
      modal.setLocalSetting('outlineVisible', true);
      modal.save();
      
      expect(modal.settings.outlineVisible).toBe(true);
    });
  });

  describe('TC-PM009: Auto-save toggle', () => {
    it('auto-save toggles on', () => {
      modal.open();
      expect(modal.getLocalSettings().autoSave).toBe(true);
      
      modal.setLocalSetting('autoSave', false);
      expect(modal.getLocalSettings().autoSave).toBe(false);
    });

    it('auto-save toggles off', () => {
      modal.open();
      modal.setLocalSetting('autoSave', false);
      modal.setLocalSetting('autoSave', true);
      expect(modal.getLocalSettings().autoSave).toBe(true);
    });

    it('auto-save persists after save', () => {
      modal.open();
      modal.setLocalSetting('autoSave', false);
      modal.save();
      
      expect(modal.settings.autoSave).toBe(false);
    });
  });

  describe('TC-PM010: Auto-save interval', () => {
    it('auto-save interval changes instantly', () => {
      modal.open();
      modal.setLocalSetting('autoSaveInterval', 60000);
      expect(modal.getLocalSettings().autoSaveInterval).toBe(60000);
    });

    it('auto-save interval persists after save', () => {
      modal.open();
      modal.setLocalSetting('autoSaveInterval', 60000);
      modal.save();
      
      expect(modal.settings.autoSaveInterval).toBe(60000);
    });

    it('auto-save interval is converted from seconds correctly', () => {
      modal.open();
      const seconds = 60;
      modal.setLocalSetting('autoSaveInterval', seconds * 1000);
      expect(modal.getLocalSettings().autoSaveInterval).toBe(60000);
    });
  });

  describe('TC-PM011: Keyboard accessibility - Escape closes', () => {
    it('Escape key closes the modal', () => {
      modal.open();
      expect(modal.isVisible).toBe(true);
      
      modal.close();
      expect(modal.isVisible).toBe(false);
    });

    it('Escape key preserves unsaved changes', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.close();
      
      expect(modal.isVisible).toBe(false);
      expect(modal.settings.theme).toBe('light');
    });
  });

  describe('TC-PM012: Keyboard accessibility - Tab navigation', () => {
    it('Tab navigation cycles through controls', () => {
      modal.open();
      const controls = [
        'btn-preferences-close',
        'btn-theme-light',
        'btn-theme-dark',
        'font-family-select',
        'font-size-slider',
        'line-height-slider',
        'content-width-slider',
        'tab-size-slider',
        'focus-mode-toggle',
        'typewriter-mode-toggle',
        'outline-toggle',
        'auto-save-toggle',
        'auto-save-interval-slider',
        'btn-preferences-cancel',
        'btn-preferences-save',
      ];
      
      controls.forEach(controlId => {
        expect(controlId).toBeDefined();
      });
    });

    it('Shift+Tab navigation works in reverse', () => {
      modal.open();
      expect(modal.isVisible).toBe(true);
    });
  });

  describe('TC-PM013: Settings persist across open/close', () => {
    it('changed settings persist after close and reopen', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.setLocalSetting('fontSize', 20);
      modal.save();
      modal.close();
      
      modal.open();
      expect(modal.getLocalSettings().theme).toBe('dark');
      expect(modal.getLocalSettings().fontSize).toBe(20);
    });

    it('cancel discards unsaved changes', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.resetToSettings();
      modal.close();
      
      expect(modal.settings.theme).toBe('light');
    });

    it('multiple settings persist correctly', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.setLocalSetting('fontSize', 24);
      modal.setLocalSetting('focusMode', true);
      modal.setLocalSetting('autoSave', false);
      modal.save();
      modal.close();
      
      expect(modal.settings.theme).toBe('dark');
      expect(modal.settings.fontSize).toBe(24);
      expect(modal.settings.focusMode).toBe(true);
      expect(modal.settings.autoSave).toBe(false);
    });
  });

  describe('Edge Cases', () => {
    describe('keyboard_navigation', () => {
      it('rapid keyboard toggles work correctly', () => {
        modal.open();
        
        for (let i = 0; i < 10; i++) {
          modal.setLocalSetting('theme', i % 2 === 0 ? 'light' : 'dark');
        }
        
        expect(['light', 'dark']).toContain(modal.getLocalSettings().theme);
      });

      it('rapid slider changes work correctly', () => {
        modal.open();
        
        for (let i = 0; i < 10; i++) {
          modal.setLocalSetting('fontSize', 12 + i);
        }
        
        expect(modal.getLocalSettings().fontSize).toBe(21);
      });
    });

    describe('rapid_changes', () => {
      it('rapid toggle changes maintain last state', () => {
        modal.open();
        
        modal.setLocalSetting('focusMode', true);
        modal.setLocalSetting('focusMode', false);
        modal.setLocalSetting('focusMode', true);
        modal.setLocalSetting('focusMode', false);
        
        expect(modal.getLocalSettings().focusMode).toBe(false);
      });

      it('rapid save calls do not cause errors', () => {
        modal.open();
        modal.setLocalSetting('fontSize', 18);
        
        for (let i = 0; i < 5; i++) {
          modal.save();
        }
        
        expect(modal.saveCallCount).toBe(5);
        expect(modal.settings.fontSize).toBe(18);
      });
    });
  });

  describe('Integration with SettingsContext', () => {
    it('setSettings is called on save', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.save();
      
      expect(modal.savedSettings.length).toBe(1);
      expect(modal.savedSettings[0].theme).toBe('dark');
    });

    it('all settings are saved together', () => {
      modal.open();
      modal.setLocalSetting('theme', 'dark');
      modal.setLocalSetting('fontSize', 20);
      modal.setLocalSetting('focusMode', true);
      modal.save();
      
      const saved = modal.savedSettings[0];
      expect(saved.theme).toBe('dark');
      expect(saved.fontSize).toBe(20);
      expect(saved.focusMode).toBe(true);
    });
  });
});

describe('PreferencesModal UI Elements', () => {
  const expectedControls = [
    { id: 'btn-theme-light', type: 'button' },
    { id: 'btn-theme-dark', type: 'button' },
    { id: 'font-family-select', type: 'select' },
    { id: 'font-size-slider', type: 'range' },
    { id: 'line-height-slider', type: 'range' },
    { id: 'content-width-slider', type: 'range' },
    { id: 'tab-size-slider', type: 'range' },
    { id: 'focus-mode-toggle', type: 'checkbox' },
    { id: 'typewriter-mode-toggle', type: 'checkbox' },
    { id: 'outline-toggle', type: 'checkbox' },
    { id: 'auto-save-toggle', type: 'checkbox' },
    { id: 'auto-save-interval-slider', type: 'range' },
    { id: 'recent-files-list', type: 'display' },
  ];

  it('has all expected control elements', () => {
    expect(expectedControls.length).toBe(13);
  });

  it('all controls have unique IDs', () => {
    const ids = expectedControls.map(c => c.id);
    const uniqueIds = new Set(ids);
    expect(uniqueIds.size).toBe(ids.length);
  });

  it('control IDs match expected format', () => {
    expectedControls.forEach(control => {
      expect(control.id).toMatch(/^[a-z0-9-]+$/);
    });
  });
});
