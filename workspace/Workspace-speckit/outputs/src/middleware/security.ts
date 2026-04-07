import { Request, Response, NextFunction } from 'express';

export function sanitizeInput(req: Request, res: Response, next: NextFunction): void {
  if (req.body && typeof req.body === 'object') {
    for (const key in req.body) {
      const val = req.body[key];
      if (typeof val === 'string') {
        req.body[key] = val.replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '').replace(/javascript:/gi, '').replace(/on\w+\s*=/gi, '');
      }
    }
  }
  next();
}

export function apiKeyAuth(required = false) {
  return (req: Request, res: Response, next: NextFunction): void => {
    const apiKey = req.headers['x-api-key'] as string;
    const configuredKey = process.env.API_KEY;
    if (!configuredKey) { if (required) { res.status(401).json({ error: { message: 'API auth required', code: 'AUTH_REQUIRED' } }); return; } next(); return; }
    if (!apiKey) { res.status(401).json({ error: { message: 'API key required', code: 'API_KEY_MISSING' } }); return; }
    if (apiKey !== configuredKey) { res.status(401).json({ error: { message: 'Invalid API key', code: 'API_KEY_INVALID' } }); return; }
    next();
  };
}