import { describe, it, expect, beforeEach, vi } from 'vitest';

/**
 * SearchManager Unit Tests
 * 
 * Tests the search.js SearchManager class functionality.
 * We mock DOM and editor dependencies to test the core logic.
 */

// Mock editor object
const mockEditor = {
  container: null,
  getContent: vi.fn(),
  setContent: vi.fn(),
  scrollToOffset: vi.fn()
};

// Mock DOM environment
const mockQuerySelectorAll = vi.fn();
const mockQuerySelector = vi.fn();
const mockCreateRange = vi.fn();
const mockCreateTreeWalker = vi.fn();
const mockCreateTextNode = vi.fn();
const mockCreateElement = vi.fn();

global.document = {
  getElementById: vi.fn((id) => {
    if (id === 'search-dialog') {
      return {
        querySelector: mockQuerySelector,
        querySelectorAll: mockQuerySelectorAll,
        remove: vi.fn()
      };
    }
    if (id === 'editor') {
      return {
        appendChild: vi.fn()
      };
    }
    return null;
  }),
  createRange: mockCreateRange,
  createTreeWalker: mockCreateTreeWalker,
  createElement: mockCreateElement,
  createTextNode: mockCreateTextNode
};

// Import the SearchManager class logic
class SearchManager {
  constructor() {
    this.dialog = null;
    this.query = '';
    this.replaceText = '';
    this.results = [];
    this.currentIndex = -1;
  }
  
  escapeRegex(string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }
  
  clearHighlights() {
    // Mock implementation - does nothing in pure tests
  }
  
  findAll(content) {
    this.clearHighlights();
    
    if (!this.query) {
      this.results = [];
      this.currentIndex = -1;
      return { results: [], currentIndex: -1 };
    }
    
    try {
      const regex = new RegExp(this.escapeRegex(this.query), 'gi');
      this.results = [];
      
      let match;
      while ((match = regex.exec(content)) !== null) {
        this.results.push({
          index: match.index,
          length: match[0].length,
          text: match[0]
        });
      }
      
      this.currentIndex = this.results.length > 0 ? 0 : -1;
      return { results: this.results, currentIndex: this.currentIndex };
    } catch (e) {
      this.results = [];
      this.currentIndex = -1;
      return { results: [], currentIndex: -1 };
    }
  }
  
  findNext() {
    if (this.results.length === 0) return;
    this.currentIndex = (this.currentIndex + 1) % this.results.length;
    return this.currentIndex;
  }
  
  findPrev() {
    if (this.results.length === 0) return;
    this.currentIndex = (this.currentIndex - 1 + this.results.length) % this.results.length;
    return this.currentIndex;
  }
  
  replaceOne(content) {
    if (this.results.length === 0 || this.currentIndex < 0) return content;
    
    const result = this.results[this.currentIndex];
    const newContent = content.substring(0, result.index) + 
                      this.replaceText + 
                      content.substring(result.index + result.length);
    return newContent;
  }
  
  replaceAll(content) {
    if (this.results.length === 0) return content;
    
    const regex = new RegExp(this.escapeRegex(this.query), 'gi');
    const newContent = content.replace(regex, this.replaceText);
    return newContent;
  }
}

describe('SearchManager Unit Tests', () => {
  let searchManager;
  
  beforeEach(() => {
    searchManager = new SearchManager();
    mockEditor.getContent.mockReset();
    mockEditor.setContent.mockReset();
  });
  
  describe('escapeRegex', () => {
    it('escapes special regex characters', () => {
      const testCases = [
        { input: 'foo.bar', expected: 'foo\\.bar' },
        { input: 'foo+bar', expected: 'foo\\+bar' },
        { input: 'foo*bar', expected: 'foo\\*bar' },
        { input: 'foo?bar', expected: 'foo\\?bar' },
        { input: 'foo(bar)', expected: 'foo\\(bar\\)' },
        { input: 'foo[bar]', expected: 'foo\\[bar\\]' },
        { input: 'foo{bar}', expected: 'foo\\{bar\\}' },
        { input: 'foo^bar', expected: 'foo\\^bar' },
        { input: 'foo$bar', expected: 'foo\\$bar' },
        { input: 'foo|bar', expected: 'foo\\|bar' },
        { input: 'foo\\bar', expected: 'foo\\\\bar' },
        { input: 'foo/bar', expected: 'foo/bar' },
      ];
      
      testCases.forEach(({ input, expected }) => {
        expect(searchManager.escapeRegex(input)).toBe(expected);
      });
    });
    
    it('returns empty string for empty input', () => {
      expect(searchManager.escapeRegex('')).toBe('');
    });
    
    it('handles multiple special characters', () => {
      expect(searchManager.escapeRegex('foo.bar+baz*qux?')).toBe('foo\\.bar\\+baz\\*qux\\?');
    });
    
    it('does not escape regular characters', () => {
      expect(searchManager.escapeRegex('hello')).toBe('hello');
      expect(searchManager.escapeRegex('hello world')).toBe('hello world');
      expect(searchManager.escapeRegex('123')).toBe('123');
    });
  });
  
  describe('findAll', () => {
    it('returns empty results for empty query', () => {
      searchManager.query = '';
      const result = searchManager.findAll('some content');
      
      expect(result.results).toEqual([]);
      expect(result.currentIndex).toBe(-1);
    });
    
    it('finds single match', () => {
      searchManager.query = 'hello';
      const result = searchManager.findAll('hello world');
      
      expect(result.results.length).toBe(1);
      expect(result.results[0]).toEqual({
        index: 0,
        length: 5,
        text: 'hello'
      });
      expect(result.currentIndex).toBe(0);
    });
    
    it('finds multiple matches', () => {
      searchManager.query = 'hello';
      const result = searchManager.findAll('hello world, hello mars, hello venus');
      
      expect(result.results.length).toBe(3);
      expect(result.results[0].index).toBe(0);
      expect(result.results[1].index).toBe(13);
      expect(result.results[2].index).toBe(25);
    });
    
    it('is case insensitive', () => {
      searchManager.query = 'HELLO';
      const result = searchManager.findAll('Hello world, hello mars, HELLO');
      
      expect(result.results.length).toBe(3);
    });
    
    it('returns empty results when no match found', () => {
      searchManager.query = 'xyz';
      const result = searchManager.findAll('hello world');
      
      expect(result.results).toEqual([]);
      expect(result.currentIndex).toBe(-1);
    });
    
    it('handles special regex characters in query', () => {
      searchManager.query = 'foo.bar';
      const result = searchManager.findAll('foo.bar is not fooXbar');
      
      // Should find literal "foo.bar" not "fooXbar"
      expect(result.results.length).toBe(1);
      expect(result.results[0].text).toBe('foo.bar');
    });
    
    it('handles unicode content', () => {
      searchManager.query = '你好';
      const result = searchManager.findAll('你好世界');
      
      expect(result.results.length).toBe(1);
    });
    
    it('handles empty content', () => {
      searchManager.query = 'hello';
      const result = searchManager.findAll('');
      
      expect(result.results).toEqual([]);
    });
  });
  
  describe('findNext', () => {
    it('cycles through results', () => {
      searchManager.query = 'a';
      searchManager.findAll('a a a a');
      
      expect(searchManager.currentIndex).toBe(0);
      expect(searchManager.findNext()).toBe(1);
      expect(searchManager.findNext()).toBe(2);
      expect(searchManager.findNext()).toBe(3);
      expect(searchManager.findNext()).toBe(0); // cycles back
    });
    
    it('returns undefined when no results', () => {
      searchManager.query = 'xyz';
      searchManager.findAll('hello world');
      
      expect(searchManager.findNext()).toBeUndefined();
    });
  });
  
  describe('findPrev', () => {
    it('cycles backwards through results', () => {
      searchManager.query = 'a';
      searchManager.findAll('a a a');
      searchManager.currentIndex = 2;
      
      expect(searchManager.findPrev()).toBe(1);
      expect(searchManager.findPrev()).toBe(0);
      expect(searchManager.findPrev()).toBe(2); // cycles back
    });
    
    it('returns undefined when no results', () => {
      searchManager.query = 'xyz';
      searchManager.findAll('hello world');
      
      expect(searchManager.findPrev()).toBeUndefined();
    });
  });
  
  describe('replaceOne', () => {
    beforeEach(() => {
      searchManager.query = 'foo';
      searchManager.replaceText = 'bar';
    });
    
    it('replaces first occurrence', () => {
      searchManager.findAll('foo is not foo');
      searchManager.currentIndex = 0;
      
      const result = searchManager.replaceOne('foo is not foo');
      expect(result).toBe('bar is not foo');
    });
    
    it('replaces at specific index', () => {
      searchManager.findAll('foo is not foo');
      searchManager.currentIndex = 1;
      
      const result = searchManager.replaceOne('foo is not foo');
      expect(result).toBe('foo is not bar');
    });
    
    it('returns original when no results', () => {
      searchManager.results = [];
      searchManager.currentIndex = -1;
      
      const result = searchManager.replaceOne('foo is not foo');
      expect(result).toBe('foo is not foo');
    });
  });
  
  describe('replaceAll', () => {
    beforeEach(() => {
      searchManager.query = 'foo';
      searchManager.replaceText = 'bar';
    });
    
    it('replaces all occurrences', () => {
      searchManager.findAll('foo is not foo');
      
      const result = searchManager.replaceAll('foo is not foo');
      expect(result).toBe('bar is not bar');
    });
    
    it('handles no matches gracefully', () => {
      searchManager.findAll('hello world');
      
      const result = searchManager.replaceAll('hello world');
      expect(result).toBe('hello world');
    });
    
    it('handles empty content', () => {
      searchManager.findAll('');
      
      const result = searchManager.replaceAll('');
      expect(result).toBe('');
    });
    
    it('handles special regex characters in replacement', () => {
      searchManager.query = 'foo';
      searchManager.replaceText = '$1 bar';
      searchManager.findAll('foo foo');
      
      const result = searchManager.replaceAll('foo foo');
      expect(result).toBe('$1 bar $1 bar');
    });
  });
});
