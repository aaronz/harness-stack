import { Request, Response, NextFunction } from 'express';
import { createError } from './error-handler';

export function sanitizeInput(req: Request, res: Response, next: NextFunction): void {
  if (req.body && typeof req.body === 'object') {
    for (const key in req.body) {
      const value = req.body[key];
      if (typeof value === 'string') {
        const sanitized = value
          .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '')
          .replace(/javascript:/gi, '')
          .replace(/on\w+\s*=/gi, '');
        req.body[key] = sanitized;
      }
    }
  }
  next();
}

export function apiKeyAuth(required = false) {
  return (req: Request, res: Response, next: NextFunction): void => {
    const apiKey = req.headers['x-api-key'] as string;
    const configuredKey = process.env.API_KEY;

    if (!configuredKey) {
      if (required) {
        next(createError('API authentication required', 401, 'AUTH_REQUIRED'));
        return;
      }
      next();
      return;
    }

    if (!apiKey) {
      next(createError('API key required', 401, 'API_KEY_MISSING'));
      return;
    }

    if (apiKey !== configuredKey) {
      next(createError('Invalid API key', 401, 'API_KEY_INVALID'));
      return;
    }

    next();
  };
}
