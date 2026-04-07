import { Request, Response, NextFunction } from 'express';

const store = new Map<string, { count: number; resetAt: number }>();
const WINDOW = 60000;
const MAX = 100;

function check(identifier: string): boolean {
  const now = Date.now();
  const entry = store.get(identifier);
  if (!entry || entry.resetAt < now) { store.set(identifier, { count: 1, resetAt: now + WINDOW }); return true; }
  if (entry.count >= MAX) return false;
  entry.count++;
  return true;
}

setInterval(() => { const now = Date.now(); for (const [k, v] of store) if (v.resetAt < now) store.delete(k); }, 60000);

export function rateLimitMiddleware(req: Request, res: Response, next: NextFunction): void {
  const id = req.ip || 'unknown';
  if (!check(id)) { res.status(429).json({ error: { message: 'Too many requests', code: 'RATE_LIMITED' } }); return; }
  next();
}