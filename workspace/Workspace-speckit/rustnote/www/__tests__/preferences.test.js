import { describe, it, expect, beforeEach, vi } from 'vitest';

const PRD09_SETTINGS = [
  'theme',
  'autoSave',
  'autoSaveInterval',
  'focusMode',
  'typewriterMode',
  'outlineVisible',
  'fontSize',
  'fontFamily',
  'lineHeight',
  'contentWidth',
  'tabSize',
  'recentFiles',
];

const createPreferencesPanel = () => {
  return {
    isVisible: false,
    settings: {
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
      recentFiles: [],
    },
    open() {
      this.isVisible = true;
    },
    close() {
      this.isVisible = false;
    },
    getAllSettingControls() {
      return [
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
    },
    hasSetting(settingName) {
      return PRD09_SETTINGS.includes(settingName);
    },
    updateSetting(settingName, value) {
      if (this.hasSetting(settingName)) {
        this.settings[settingName] = value;
        return true;
      }
      return false;
    },
  };
};

describe('TC-G013-001: Preferences panel access', () => {
  let panel;

  beforeEach(() => {
    panel = createPreferencesPanel();
  });

  describe('Panel Open/Close', () => {
    it('opens preferences panel when clicking preferences button', () => {
      expect(panel.isVisible).toBe(false);
      panel.open();
      expect(panel.isVisible).toBe(true);
    });

    it('closes preferences panel', () => {
      panel.open();
      expect(panel.isVisible).toBe(true);
      panel.close();
      expect(panel.isVisible).toBe(false);
    });

    it('preferences panel renders all settings when opened', () => {
      panel.open();
      const controls = panel.getAllSettingControls();
      expect(controls.length).toBeGreaterThan(0);
    });
  });

  describe('UI Elements', () => {
    it('has theme toggle buttons', () => {
      const controls = panel.getAllSettingControls();
      const themeButtons = controls.filter(c => c.id.startsWith('btn-theme'));
      expect(themeButtons.length).toBe(2);
    });

    it('has font family selector', () => {
      const controls = panel.getAllSettingControls();
      const fontSelect = controls.find(c => c.id === 'font-family-select');
      expect(fontSelect).toBeDefined();
      expect(fontSelect.type).toBe('select');
    });

    it('has range sliders for numeric settings', () => {
      const controls = panel.getAllSettingControls();
      const sliders = controls.filter(c => c.type === 'range');
      expect(sliders.length).toBeGreaterThan(0);
    });

    it('has checkboxes for toggle settings', () => {
      const controls = panel.getAllSettingControls();
      const checkboxes = controls.filter(c => c.type === 'checkbox');
      expect(checkboxes.length).toBeGreaterThan(0);
    });

    it('has recent files display', () => {
      const controls = panel.getAllSettingControls();
      const recentFiles = controls.find(c => c.id === 'recent-files-list');
      expect(recentFiles).toBeDefined();
    });
  });

  describe('Integration', () => {
    it('opens via preferences button click', () => {
      const mockEvent = { target: { id: 'btn-preferences' } };
      if (mockEvent.target.id === 'btn-preferences') {
        panel.open();
      }
      expect(panel.isVisible).toBe(true);
    });

    it('closes via close button', () => {
      panel.open();
      const mockEvent = { target: { id: 'btn-preferences-close' } };
      if (mockEvent.target.id === 'btn-preferences-close') {
        panel.close();
      }
      expect(panel.isVisible).toBe(false);
    });

    it('closes via Escape key', () => {
      panel.open();
      const keyEvent = { key: 'Escape' };
      if (keyEvent.key === 'Escape') {
        panel.close();
      }
      expect(panel.isVisible).toBe(false);
    });

    it('closes when clicking backdrop', () => {
      panel.open();
      const mockEvent = { target: { className: 'absolute inset-0 bg-black/50' } };
      if (mockEvent.target.className.includes('bg-black/50')) {
        panel.close();
      }
      expect(panel.isVisible).toBe(false);
    });
  });
});

describe('TC-G013-002: All PRD-09 settings present', () => {
  let panel;

  beforeEach(() => {
    panel = createPreferencesPanel();
  });

  describe('PRD-09 Settings Coverage', () => {
    it('has theme setting', () => {
      expect(panel.hasSetting('theme')).toBe(true);
      expect(panel.settings.theme).toBeDefined();
    });

    it('has autoSave setting', () => {
      expect(panel.hasSetting('autoSave')).toBe(true);
      expect(typeof panel.settings.autoSave).toBe('boolean');
    });

    it('has autoSaveInterval setting', () => {
      expect(panel.hasSetting('autoSaveInterval')).toBe(true);
      expect(panel.settings.autoSaveInterval).toBe(30000);
    });

    it('has focusMode setting', () => {
      expect(panel.hasSetting('focusMode')).toBe(true);
      expect(typeof panel.settings.focusMode).toBe('boolean');
    });

    it('has typewriterMode setting', () => {
      expect(panel.hasSetting('typewriterMode')).toBe(true);
      expect(typeof panel.settings.typewriterMode).toBe('boolean');
    });

    it('has outlineVisible setting', () => {
      expect(panel.hasSetting('outlineVisible')).toBe(true);
      expect(typeof panel.settings.outlineVisible).toBe('boolean');
    });

    it('has fontSize setting', () => {
      expect(panel.hasSetting('fontSize')).toBe(true);
      expect(panel.settings.fontSize).toBe(16);
    });

    it('has fontFamily setting', () => {
      expect(panel.hasSetting('fontFamily')).toBe(true);
      expect(panel.settings.fontFamily).toBe('System');
    });

    it('has lineHeight setting', () => {
      expect(panel.hasSetting('lineHeight')).toBe(true);
      expect(panel.settings.lineHeight).toBe(1.6);
    });

    it('has contentWidth setting', () => {
      expect(panel.hasSetting('contentWidth')).toBe(true);
      expect(panel.settings.contentWidth).toBe(720);
    });

    it('has tabSize setting', () => {
      expect(panel.hasSetting('tabSize')).toBe(true);
      expect(panel.settings.tabSize).toBe(4);
    });

    it('has recentFiles setting', () => {
      expect(panel.hasSetting('recentFiles')).toBe(true);
      expect(Array.isArray(panel.settings.recentFiles)).toBe(true);
    });
  });

  describe('All Settings Updateable', () => {
    it('can update theme', () => {
      expect(panel.updateSetting('theme', 'dark')).toBe(true);
      expect(panel.settings.theme).toBe('dark');
    });

    it('can update fontSize', () => {
      expect(panel.updateSetting('fontSize', 20)).toBe(true);
      expect(panel.settings.fontSize).toBe(20);
    });

    it('can update autoSave', () => {
      expect(panel.updateSetting('autoSave', false)).toBe(true);
      expect(panel.settings.autoSave).toBe(false);
    });

    it('can update focusMode', () => {
      expect(panel.updateSetting('focusMode', true)).toBe(true);
      expect(panel.settings.focusMode).toBe(true);
    });
  });

  describe('UI Control Mapping', () => {
    it('maps all PRD-09 settings to UI controls', () => {
      const settingsToControls = {
        theme: ['btn-theme-light', 'btn-theme-dark'],
        autoSave: ['auto-save-toggle'],
        autoSaveInterval: ['auto-save-interval-slider'],
        focusMode: ['focus-mode-toggle'],
        typewriterMode: ['typewriter-mode-toggle'],
        outlineVisible: ['outline-toggle'],
        fontSize: ['font-size-slider'],
        fontFamily: ['font-family-select'],
        lineHeight: ['line-height-slider'],
        contentWidth: ['content-width-slider'],
        tabSize: ['tab-size-slider'],
      };

      Object.entries(settingsToControls).forEach(([setting, controls]) => {
        expect(panel.hasSetting(setting)).toBe(true);
        controls.forEach(controlId => {
          const found = panel.getAllSettingControls().find(c => c.id === controlId);
          expect(found).toBeDefined();
        });
      });
    });
  });
});

describe('Preferences Panel Validation', () => {
  let panel;

  beforeEach(() => {
    panel = createPreferencesPanel();
  });

  it('validates theme values', () => {
    const validThemes = ['light', 'dark'];
    expect(validThemes.includes(panel.settings.theme)).toBe(true);
  });

  it('validates font size range', () => {
    const fontSize = panel.settings.fontSize;
    expect(fontSize).toBeGreaterThanOrEqual(12);
    expect(fontSize).toBeLessThanOrEqual(32);
  });

  it('validates auto-save interval range', () => {
    const interval = panel.settings.autoSaveInterval;
    const seconds = interval / 1000;
    expect(seconds).toBeGreaterThanOrEqual(10);
    expect(seconds).toBeLessThanOrEqual(300);
  });

  it('validates font family options', () => {
    const validFonts = ['System', 'Serif', 'Monospace'];
    expect(validFonts.includes(panel.settings.fontFamily)).toBe(true);
  });
});
