import { describe, it, expect, beforeEach, vi } from 'vitest';

/**
 * Settings Panel Unit Tests
 * 
 * Tests the settings.js settings panel functionality.
 */

const createSettings = () => ({
  theme: 'light',
  autoSave: true,
  autoSaveInterval: 30000,
  fontFamily: 'System',
  fontSize: 16,
  lineHeight: 1.6,
  tabSize: 4,
  recentFiles: []
});

const createSettingsPanel = () => {
  return {
    dialog: null,
    validateTheme(theme) {
      return ['light', 'dark'].includes(theme);
    },
    
    validateFontSize(size) {
      return size >= 12 && size <= 32;
    },
    
    validateAutoSaveInterval(interval) {
      const seconds = interval / 1000;
      return seconds >= 10 && seconds <= 300;
    },
    
    parseIntervalFromSeconds(seconds) {
      return seconds * 1000;
    },
    
    serializeIntervalToSeconds(ms) {
      return ms / 1000;
    },
    
    isValidFontFamily(font) {
      return ['System', 'Serif', 'Monospace'].includes(font);
    },
    
    mergeSettings(current, updates) {
      return { ...current, ...updates };
    },
    
    validateSettings(settings) {
      const errors = [];
      
      if (!this.validateTheme(settings.theme)) {
        errors.push('Invalid theme');
      }
      if (!this.validateFontSize(settings.fontSize)) {
        errors.push('Font size must be between 12 and 32');
      }
      if (!this.validateAutoSaveInterval(settings.autoSaveInterval)) {
        errors.push('Auto-save interval must be between 10 and 300 seconds');
      }
      if (!this.isValidFontFamily(settings.fontFamily)) {
        errors.push('Invalid font family');
      }
      
      return { valid: errors.length === 0, errors };
    }
  };
};

describe('Settings Panel Unit Tests', () => {
  let settingsPanel;
  let settings;
  
  beforeEach(() => {
    settingsPanel = createSettingsPanel();
    settings = createSettings();
  });
  
  describe('validateTheme', () => {
    it('accepts valid themes', () => {
      expect(settingsPanel.validateTheme('light')).toBe(true);
      expect(settingsPanel.validateTheme('dark')).toBe(true);
    });
    
    it('rejects invalid themes', () => {
      expect(settingsPanel.validateTheme('blue')).toBe(false);
      expect(settingsPanel.validateTheme('red')).toBe(false);
      expect(settingsPanel.validateTheme('')).toBe(false);
      expect(settingsPanel.validateTheme(null)).toBe(false);
      expect(settingsPanel.validateTheme(undefined)).toBe(false);
    });
  });
  
  describe('validateFontSize', () => {
    it('accepts valid font sizes', () => {
      expect(settingsPanel.validateFontSize(12)).toBe(true);
      expect(settingsPanel.validateFontSize(16)).toBe(true);
      expect(settingsPanel.validateFontSize(32)).toBe(true);
      expect(settingsPanel.validateFontSize(20)).toBe(true);
    });
    
    it('rejects font sizes below minimum', () => {
      expect(settingsPanel.validateFontSize(11)).toBe(false);
      expect(settingsPanel.validateFontSize(0)).toBe(false);
      expect(settingsPanel.validateFontSize(-1)).toBe(false);
    });
    
    it('rejects font sizes above maximum', () => {
      expect(settingsPanel.validateFontSize(33)).toBe(false);
      expect(settingsPanel.validateFontSize(100)).toBe(false);
    });
  });
  
  describe('validateAutoSaveInterval', () => {
    it('accepts valid intervals', () => {
      expect(settingsPanel.validateAutoSaveInterval(10000)).toBe(true);
      expect(settingsPanel.validateAutoSaveInterval(30000)).toBe(true);
      expect(settingsPanel.validateAutoSaveInterval(300000)).toBe(true);
      expect(settingsPanel.validateAutoSaveInterval(60000)).toBe(true);
    });
    
    it('rejects intervals below minimum (10 seconds)', () => {
      expect(settingsPanel.validateAutoSaveInterval(9000)).toBe(false);
      expect(settingsPanel.validateAutoSaveInterval(0)).toBe(false);
    });
    
    it('rejects intervals above maximum (300 seconds)', () => {
      expect(settingsPanel.validateAutoSaveInterval(301000)).toBe(false);
      expect(settingsPanel.validateAutoSaveInterval(600000)).toBe(false);
    });
  });
  
  describe('parseIntervalFromSeconds', () => {
    it('converts seconds to milliseconds', () => {
      expect(settingsPanel.parseIntervalFromSeconds(10)).toBe(10000);
      expect(settingsPanel.parseIntervalFromSeconds(30)).toBe(30000);
      expect(settingsPanel.parseIntervalFromSeconds(300)).toBe(300000);
    });
  });
  
  describe('serializeIntervalToSeconds', () => {
    it('converts milliseconds to seconds', () => {
      expect(settingsPanel.serializeIntervalToSeconds(10000)).toBe(10);
      expect(settingsPanel.serializeIntervalToSeconds(30000)).toBe(30);
      expect(settingsPanel.serializeIntervalToSeconds(300000)).toBe(300);
    });
  });
  
  describe('isValidFontFamily', () => {
    it('accepts valid font families', () => {
      expect(settingsPanel.isValidFontFamily('System')).toBe(true);
      expect(settingsPanel.isValidFontFamily('Serif')).toBe(true);
      expect(settingsPanel.isValidFontFamily('Monospace')).toBe(true);
    });
    
    it('rejects invalid font families', () => {
      expect(settingsPanel.isValidFontFamily('Arial')).toBe(false);
      expect(settingsPanel.isValidFontFamily('Helvetica')).toBe(false);
      expect(settingsPanel.isValidFontFamily('')).toBe(false);
      expect(settingsPanel.isValidFontFamily(null)).toBe(false);
    });
  });
  
  describe('mergeSettings', () => {
    it('merges updates into current settings', () => {
      const updates = { theme: 'dark', fontSize: 20 };
      const result = settingsPanel.mergeSettings(settings, updates);
      
      expect(result.theme).toBe('dark');
      expect(result.fontSize).toBe(20);
      expect(result.autoSave).toBe(true);
    });
    
    it('does not mutate original settings', () => {
      const original = { ...settings };
      settingsPanel.mergeSettings(settings, { theme: 'dark' });
      
      expect(settings.theme).toBe(original.theme);
    });
    
    it('handles empty updates', () => {
      const result = settingsPanel.mergeSettings(settings, {});
      expect(result).toEqual(settings);
    });
  });
  
  describe('validateSettings', () => {
    it('returns valid for correct settings', () => {
      const result = settingsPanel.validateSettings(settings);
      
      expect(result.valid).toBe(true);
      expect(result.errors).toEqual([]);
    });
    
    it('returns invalid for incorrect theme', () => {
      const badSettings = { ...settings, theme: 'blue' };
      const result = settingsPanel.validateSettings(badSettings);
      
      expect(result.valid).toBe(false);
      expect(result.errors).toContain('Invalid theme');
    });
    
    it('returns invalid for out-of-range font size', () => {
      const badSettings = { ...settings, fontSize: 50 };
      const result = settingsPanel.validateSettings(badSettings);
      
      expect(result.valid).toBe(false);
      expect(result.errors).toContain('Font size must be between 12 and 32');
    });
    
    it('returns invalid for out-of-range auto-save interval', () => {
      const badSettings = { ...settings, autoSaveInterval: 5000 };
      const result = settingsPanel.validateSettings(badSettings);
      
      expect(result.valid).toBe(false);
      expect(result.errors).toContain('Auto-save interval must be between 10 and 300 seconds');
    });
    
    it('returns multiple errors for multiple issues', () => {
      const badSettings = { 
        theme: 'blue', 
        fontSize: 50, 
        autoSaveInterval: 5000,
        fontFamily: 'Invalid'
      };
      const result = settingsPanel.validateSettings(badSettings);
      
      expect(result.valid).toBe(false);
      expect(result.errors.length).toBe(4);
    });
  });
});
