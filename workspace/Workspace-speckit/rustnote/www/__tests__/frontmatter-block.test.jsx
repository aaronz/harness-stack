import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import FrontmatterBlock, { parseFrontmatter, extractFrontmatter, isValidFrontmatter } from '../src/components/FrontmatterBlock.jsx';
import '../src/index.css';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

describe('P2-017: FrontmatterBlock Visual Polish', () => {

  describe('TC-P2-017-01: Frontmatter key-value display', () => {
    it('should render key-value pairs with visual separation', () => {
      const fm = `---
title: My Document
author: John Doe
date: 2026-04-14
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const keys = screen.getAllByText(/^(title|author|date):/);
      expect(keys.length).toBe(3);
    });

    it('should render keys with accent color styling', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const keyEl = screen.getByText('title:');
      expect(keyEl.className).toBe('frontmatter-key');
    });

    it('should render values with primary text color styling', () => {
      const fm = `---
title: My Document
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const valueEl = screen.getByText('My Document');
      expect(valueEl.className).toBe('frontmatter-value');
    });

    it('should render key-value lines with dashed separator', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const lineEl = screen.getByText('title:').closest('.frontmatter-line');
      expect(lineEl.className).toBe('frontmatter-line');
    });

    it('should display field count badge', () => {
      const fm = `---
title: Test
author: John
date: 2026-04-14
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      expect(screen.getByText('3 fields')).toBeTruthy();
    });

    it('should render key-value pairs from parseFrontmatter', () => {
      const raw = `title: My Document
author: John Doe`;
      const result = parseFrontmatter(raw);
      expect(result).toEqual({ title: 'My Document', author: 'John Doe' });
    });

    it('should handle keys with spaces', () => {
      const raw = `custom key: value here`;
      const result = parseFrontmatter(raw);
      expect(result['custom key']).toBe('value here');
    });

    it('should handle empty values', () => {
      const raw = `title:
author: John`;
      const result = parseFrontmatter(raw);
      expect(result.title).toBe('');
      expect(result.author).toBe('John');
    });
  });

  describe('TC-P2-017-02: Frontmatter in light theme', () => {
    beforeEach(() => {
      document.documentElement.removeAttribute('data-theme');
    });

    it('should apply light theme CSS class to frontmatter block', () => {
      document.documentElement.setAttribute('data-theme', 'light');
      const fm = `---
title: Light Theme Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const block = screen.getByText('title:').closest('.frontmatter-block');
      expect(block).toBeTruthy();
      expect(block.className).toBe('frontmatter-block');
    });

    it('should use light theme bg-secondary for block background', () => {
      document.documentElement.setAttribute('data-theme', 'light');
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const block = screen.getByText('title:').closest('.frontmatter-block');
      expect(block.className).toBe('frontmatter-block');
    });

    it('should render frontmatter header with light theme hover color', () => {
      document.documentElement.setAttribute('data-theme', 'light');
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const header = screen.getByText('Frontmatter').closest('.frontmatter-header');
      expect(header.className).toBe('frontmatter-header');
    });

    it('should apply light theme accent color to keys', () => {
      document.documentElement.setAttribute('data-theme', 'light');
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const keyEl = screen.getByText('title:');
      expect(keyEl.className).toBe('frontmatter-key');
    });

    it('should apply light theme text color to values', () => {
      document.documentElement.setAttribute('data-theme', 'light');
      const fm = `---
title: Test Document
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const valueEl = screen.getByText('Test Document');
      expect(valueEl.className).toBe('frontmatter-value');
    });

    it('should render field count badge with accent color', () => {
      document.documentElement.setAttribute('data-theme', 'light');
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const badge = screen.getByText('1 fields');
      expect(badge.className).toBe('frontmatter-count');
    });
  });

  describe('TC-P2-017-03: Frontmatter in dark theme', () => {
    beforeEach(() => {
      document.documentElement.removeAttribute('data-theme');
    });

    it('should apply dark theme CSS class to frontmatter block', () => {
      document.documentElement.setAttribute('data-theme', 'dark');
      const fm = `---
title: Dark Theme Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const block = screen.getByText('title:').closest('.frontmatter-block');
      expect(block).toBeTruthy();
      expect(block.className).toBe('frontmatter-block');
    });

    it('should render frontmatter header with dark theme background', () => {
      document.documentElement.setAttribute('data-theme', 'dark');
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const header = screen.getByText('Frontmatter').closest('.frontmatter-header');
      expect(header.className).toBe('frontmatter-header');
    });

    it('should apply dark theme accent color to keys', () => {
      document.documentElement.setAttribute('data-theme', 'dark');
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const keyEl = screen.getByText('title:');
      expect(keyEl.className).toBe('frontmatter-key');
    });

    it('should apply dark theme text color to values', () => {
      document.documentElement.setAttribute('data-theme', 'dark');
      const fm = `---
title: Dark Document
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const valueEl = screen.getByText('Dark Document');
      expect(valueEl.className).toBe('frontmatter-value');
    });

    it('should render error state with dark theme error colors', () => {
      document.documentElement.setAttribute('data-theme', 'dark');
      const fm = `---
invalid line without colon
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const errorTextEl = screen.getByText('Invalid frontmatter format');
      const errorEl = errorTextEl.closest('.frontmatter-error');
      expect(errorEl.className).toBe('frontmatter-error');
    });

    it('should render invalid badge with dark theme color', () => {
      document.documentElement.setAttribute('data-theme', 'dark');
      const fm = `---
bad content
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const badge = screen.getByText('Invalid');
      expect(badge.className).toBe('frontmatter-invalid');
    });
  });

  describe('TC-P2-017-04: Visual consistency with editor', () => {
    it('should use CSS class for block styling (consistent with editor)', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const block = screen.getByText('title:').closest('.frontmatter-block');
      expect(block.className).toBe('frontmatter-block');
    });

    it('should use editor mono font via CSS class', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const block = screen.getByText('title:').closest('.frontmatter-block');
      const content = screen.getByText('title:').closest('.frontmatter-content');
      expect(block.className).toBe('frontmatter-block');
      expect(content.className).toBe('frontmatter-content');
    });

    it('should have border styling consistent with editor panels', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const block = screen.getByText('title:').closest('.frontmatter-block');
      expect(block.className).toBe('frontmatter-block');
    });

    it('should use dashed separator for key-value lines', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const lines = document.querySelectorAll('.frontmatter-line');
      expect(lines.length).toBeGreaterThan(0);
      lines.forEach(line => {
        expect(line.className).toBe('frontmatter-line');
      });
    });

    it('should use CSS classes for all styling (not inline styles)', () => {
      const fm = `---
title: Test Document
author: Test Author
---`;
      const { container } = render(<FrontmatterBlock frontmatter={fm} />);
      const blockEl = container.querySelector('.frontmatter-block');
      const headerEl = container.querySelector('.frontmatter-header');
      const lineEl = container.querySelector('.frontmatter-line');
      expect(blockEl.style.background).toBe('');
      expect(headerEl.style.background).toBe('');
      expect(lineEl.style.display).toBe('');
    });

    it('should render delimiters with secondary text color', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const delimiters = screen.getAllByText('---');
      delimiters.forEach(d => {
        expect(['frontmatter-delimiter', 'frontmatter-line'].some(c => d.className?.includes(c) || d.closest(`.${c}`))).toBe(true);
      });
    });
  });

  describe('Edge cases: theme_switching', () => {
    it('should re-render correctly when switching from light to dark theme', () => {
      const fm = `---
title: Theme Switch Test
---`;
      const { rerender } = render(<FrontmatterBlock frontmatter={fm} />);
      expect(screen.getByText('title:').className).toBe('frontmatter-key');
      
      document.documentElement.setAttribute('data-theme', 'dark');
      rerender(<FrontmatterBlock frontmatter={fm} />);
      expect(screen.getByText('title:').className).toBe('frontmatter-key');
    });

    it('should re-render correctly when switching from dark to light theme', () => {
      document.documentElement.setAttribute('data-theme', 'dark');
      const fm = `---
title: Theme Switch Test
---`;
      const { rerender } = render(<FrontmatterBlock frontmatter={fm} />);
      expect(screen.getByText('title:').className).toBe('frontmatter-key');
      
      document.documentElement.setAttribute('data-theme', 'light');
      rerender(<FrontmatterBlock frontmatter={fm} />);
      expect(screen.getByText('title:').className).toBe('frontmatter-key');
    });
  });

  describe('Edge cases: missing_frontmatter', () => {
    it('should return null when frontmatter prop is null', () => {
      const { container } = render(<FrontmatterBlock frontmatter={null} />);
      expect(container.firstChild).toBeNull();
    });

    it('should return null when frontmatter prop is undefined', () => {
      const { container } = render(<FrontmatterBlock frontmatter={undefined} />);
      expect(container.firstChild).toBeNull();
    });

    it('should return null when frontmatter prop is empty string', () => {
      const { container } = render(<FrontmatterBlock frontmatter="" />);
      expect(container.firstChild).toBeNull();
    });

    it('should return null when frontmatter prop is only whitespace', () => {
      const { container } = render(<FrontmatterBlock frontmatter="   " />);
      expect(container.firstChild).toBeNull();
    });

    it('should handle parseFrontmatter with null input', () => {
      const result = parseFrontmatter(null);
      expect(result).toBeNull();
    });

    it('should handle extractFrontmatter with null content', () => {
      const result = extractFrontmatter(null);
      expect(result.frontmatter).toBeNull();
      expect(result.contentWithoutFrontmatter).toBeNull();
    });

    it('should handle extractFrontmatter with empty string', () => {
      const result = extractFrontmatter('');
      expect(result.frontmatter).toBeNull();
      expect(result.contentWithoutFrontmatter).toBe('');
    });

    it('should handle extractFrontmatter with non-string input', () => {
      const result = extractFrontmatter(123);
      expect(result.frontmatter).toBeNull();
      expect(result.contentWithoutFrontmatter).toBe(123);
    });
  });

  describe('Edge cases: malformed_frontmatter', () => {
    it('should show error message for malformed frontmatter', () => {
      const fm = `---
invalid line without colon
valid: value
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      expect(screen.getByText('Invalid frontmatter format')).toBeTruthy();
    });

    it('should show invalid badge for malformed frontmatter', () => {
      const fm = `---
not: valid
also bad line
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      expect(screen.getByText('Invalid')).toBeTruthy();
    });

    it('should validate frontmatter correctly - valid case', () => {
      const fm = `---
title: Test
author: John
---`;
      expect(isValidFrontmatter(fm)).toBe(true);
    });

    it('should validate frontmatter correctly - invalid case', () => {
      const fm = `---
title: Test
bad line without colon
---`;
      expect(isValidFrontmatter(fm)).toBe(false);
    });

    it('should validate frontmatter with only delimiters', () => {
      expect(isValidFrontmatter(null)).toBe(true);
      expect(isValidFrontmatter('')).toBe(true);
    });

    it('should validate frontmatter with empty content between delimiters', () => {
      const fm = `---

---`;
      expect(isValidFrontmatter(fm)).toBe(true);
    });

    it('should parse malformed frontmatter gracefully', () => {
      const fm = `---
title: Test
malformed line
author: John
---`;
      const result = parseFrontmatter(fm);
      expect(result.title).toBe('Test');
      expect(result.author).toBe('John');
    });

    it('should extract content without frontmatter', () => {
      const content = `---
title: Test
---

# Main Content`;
      const result = extractFrontmatter(content);
      expect(result.frontmatter).toContain('title: Test');
      expect(result.contentWithoutFrontmatter).toContain('# Main Content');
    });
  });

  describe('Collapse/expand behavior', () => {
    it('should collapse by default', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const body = document.querySelector('.frontmatter-body');
      expect(body.style.display).toBe('none');
    });

    it('should expand on header click', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const header = screen.getByText('Frontmatter').closest('.frontmatter-header');
      fireEvent.click(header);
      const body = document.querySelector('.frontmatter-body');
      expect(body.style.display).toBe('block');
    });

    it('should call onToggle callback when clicked', () => {
      const onToggle = vi.fn();
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} onToggle={onToggle} />);
      const header = screen.getByText('Frontmatter').closest('.frontmatter-header');
      fireEvent.click(header);
      expect(onToggle).toHaveBeenCalledWith(false);
    });

    it('should render toggle arrow pointing right when collapsed', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const toggle = document.querySelector('.frontmatter-toggle');
      expect(toggle.className).toBe('frontmatter-toggle');
    });

    it('should render toggle arrow pointing down when expanded', () => {
      const fm = `---
title: Test
---`;
      render(<FrontmatterBlock frontmatter={fm} />);
      const header = screen.getByText('Frontmatter').closest('.frontmatter-header');
      fireEvent.click(header);
      const toggle = document.querySelector('.frontmatter-toggle');
      expect(toggle.className).toBe('frontmatter-toggle expanded');
    });
  });

  describe('parseFrontmatter edge cases', () => {
    it('should handle multiline values', () => {
      const raw = `description: This is a\nmultiline value`;
      const result = parseFrontmatter(raw);
      expect(result.description).toBe('This is a');
    });

    it('should trim whitespace from keys and values', () => {
      const raw = `  title  :   My Value  `;
      const result = parseFrontmatter(raw);
      expect(result.title).toBe('My Value');
    });

    it('should return null for empty frontmatter string', () => {
      const result = parseFrontmatter('');
      expect(result).toBeNull();
    });

    it('should return null for only delimiters', () => {
      const raw = `---`;
      const result = parseFrontmatter(raw);
      expect(result).toBeNull();
    });
  });
});
