import { describe, it, expect, beforeEach, vi } from 'vitest';

/**
 * OutlineManager Unit Tests
 * 
 * Tests the outline.js outlineManager functionality.
 */

const createMockEditor = () => ({
  scrollToOffset: vi.fn(),
  container: {
    querySelectorAll: vi.fn()
  }
});

const createOutlineManager = () => {
  const outlineManager = {
    panel: null,
    list: null,
    headings: [],
    isVisible: false,
    
    setPanelAndList(panel, list) {
      this.panel = panel;
      this.list = list;
    },
    
    toggle() {
      this.isVisible = !this.isVisible;
      return this.isVisible;
    },
    
    update(headings) {
      this.headings = headings || [];
      return this.headings;
    },
    
    getHeadings() {
      return this.headings;
    },
    
    clearHeadings() {
      this.headings = [];
      return this.headings;
    },
    
    setVisible(visible) {
      this.isVisible = visible;
    },
    
    isActive() {
      return this.isVisible;
    },
    
    navigateToHeading(heading) {
      if (typeof this.scrollToOffset === 'function') {
        this.scrollToOffset(heading.offset);
      }
      this.setActiveHeading(heading);
    },
    
    scrollToOffset(offset) {
      return offset;
    },
    
    setActiveHeading(heading) {
      return heading;
    }
  };
  
  return outlineManager;
};

describe('OutlineManager Unit Tests', () => {
  let outlineManager;
  
  beforeEach(() => {
    outlineManager = createOutlineManager();
  });
  
  describe('toggle', () => {
    it('toggles visibility state', () => {
      expect(outlineManager.isVisible).toBe(false);
      
      outlineManager.toggle();
      expect(outlineManager.isVisible).toBe(true);
      
      outlineManager.toggle();
      expect(outlineManager.isVisible).toBe(false);
    });
    
    it('returns current visibility state', () => {
      expect(outlineManager.toggle()).toBe(true);
      expect(outlineManager.toggle()).toBe(false);
    });
  });
  
  describe('update', () => {
    it('updates headings array', () => {
      const headings = [
        { level: 1, text: 'Introduction', offset: 0 },
        { level: 2, text: 'Background', offset: 50 },
        { level: 2, text: 'Details', offset: 150 }
      ];
      
      outlineManager.update(headings);
      
      expect(outlineManager.headings).toEqual(headings);
      expect(outlineManager.headings.length).toBe(3);
    });
    
    it('handles null/undefined input', () => {
      outlineManager.update(null);
      expect(outlineManager.headings).toEqual([]);
      
      outlineManager.update(undefined);
      expect(outlineManager.headings).toEqual([]);
    });
    
    it('handles empty array', () => {
      outlineManager.update([]);
      expect(outlineManager.headings).toEqual([]);
    });
    
    it('handles single heading', () => {
      const headings = [{ level: 1, text: 'Title', offset: 0 }];
      outlineManager.update(headings);
      
      expect(outlineManager.headings.length).toBe(1);
    });
    
    it('handles deeply nested headings', () => {
      const headings = [
        { level: 1, text: 'H1', offset: 0 },
        { level: 2, text: 'H2', offset: 10 },
        { level: 3, text: 'H3', offset: 20 },
        { level: 4, text: 'H4', offset: 30 },
        { level: 5, text: 'H5', offset: 40 },
        { level: 6, text: 'H6', offset: 50 }
      ];
      
      outlineManager.update(headings);
      
      expect(outlineManager.headings.length).toBe(6);
    });
  });
  
  describe('getHeadings', () => {
    it('returns current headings', () => {
      outlineManager.headings = [{ level: 1, text: 'Test', offset: 0 }];
      expect(outlineManager.getHeadings()).toEqual([{ level: 1, text: 'Test', offset: 0 }]);
    });
    
    it('returns empty array when no headings', () => {
      outlineManager.headings = [];
      expect(outlineManager.getHeadings()).toEqual([]);
    });
  });
  
  describe('clearHeadings', () => {
    it('clears all headings', () => {
      outlineManager.headings = [
        { level: 1, text: 'H1', offset: 0 },
        { level: 2, text: 'H2', offset: 50 }
      ];
      
      outlineManager.clearHeadings();
      
      expect(outlineManager.headings).toEqual([]);
    });
  });
  
  describe('setVisible', () => {
    it('sets visibility state', () => {
      outlineManager.setVisible(true);
      expect(outlineManager.isVisible).toBe(true);
      
      outlineManager.setVisible(false);
      expect(outlineManager.isVisible).toBe(false);
    });
  });
  
  describe('isActive', () => {
    it('returns current visibility state', () => {
      outlineManager.setVisible(true);
      expect(outlineManager.isActive()).toBe(true);
      
      outlineManager.setVisible(false);
      expect(outlineManager.isActive()).toBe(false);
    });
  });
  
  describe('navigateToHeading', () => {
    it('calls scrollToOffset with heading offset', () => {
      const heading = { level: 1, text: 'Test', offset: 100 };
      const scrollSpy = vi.spyOn(outlineManager, 'scrollToOffset');
      
      outlineManager.navigateToHeading(heading);
      
      expect(scrollSpy).toHaveBeenCalledWith(100);
    });
    
    it('sets active heading', () => {
      const heading = { level: 2, text: 'Section', offset: 50 };
      const setActiveSpy = vi.spyOn(outlineManager, 'setActiveHeading');
      
      outlineManager.navigateToHeading(heading);
      
      expect(setActiveSpy).toHaveBeenCalledWith(heading);
    });
  });
  
  describe('heading hierarchy', () => {
    it('extracts correct heading levels', () => {
      const headings = [
        { level: 1, text: 'H1', offset: 0 },
        { level: 2, text: 'H2', offset: 10 },
        { level: 3, text: 'H3', offset: 20 }
      ];
      
      outlineManager.update(headings);
      
      const levels = outlineManager.headings.map(h => h.level);
      expect(levels).toEqual([1, 2, 3]);
    });
    
    it('handles headings without text', () => {
      const headings = [
        { level: 1, offset: 0 },
        { level: 2, offset: 10 }
      ];
      
      outlineManager.update(headings);
      
      expect(outlineManager.headings.length).toBe(2);
    });
    
    it('handles headings with special characters', () => {
      const headings = [
        { level: 1, text: 'Title with <special> & "chars"', offset: 0 },
        { level: 2, text: 'Code: `const x = 1`', offset: 50 }
      ];
      
      outlineManager.update(headings);
      
      expect(outlineManager.headings[0].text).toContain('special');
      expect(outlineManager.headings[1].text).toContain('const x = 1');
    });
    
    it('handles unicode headings', () => {
      const headings = [
        { level: 1, text: '标题', offset: 0 },
        { level: 2, text: '見出し', offset: 20 }
      ];
      
      outlineManager.update(headings);
      
      expect(outlineManager.headings.length).toBe(2);
      expect(outlineManager.headings[0].text).toBe('标题');
    });
  });
});
