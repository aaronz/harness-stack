import { Request, Response, NextFunction } from 'express';

interface RateLimitEntry {
  count: number;
  resetAt: number;
}

class RateLimiter {
  private store = new Map<string, RateLimitEntry>();
  private windowMs: number;
  private maxRequests: number;

  constructor(windowMs = 60000, maxRequests = 100) {
    this.windowMs = windowMs;
    this.maxRequests = maxRequests;
  }

  check(identifier: string): boolean {
    const now = Date.now();
    const entry = this.store.get(identifier);

    if (!entry || entry.resetAt < now) {
      this.store.set(identifier, {
        count: 1,
        resetAt: now + this.windowMs,
      });
      return true;
    }

    if (entry.count >= this.maxRequests) {
      return false;
    }

    entry.count++;
    return true;
  }

  reset(identifier: string): void {
    this.store.delete(identifier);
  }

  cleanup(): void {
    const now = Date.now();
    for (const [key, entry] of this.store.entries()) {
      if (entry.resetAt < now) {
        this.store.delete(key);
      }
    }
  }
}

const apiLimiter = new RateLimiter(60000, 100);
const evaluationLimiter = new RateLimiter(60000, 20);

setInterval(() => {
  apiLimiter.cleanup();
  evaluationLimiter.cleanup();
}, 60000);

export function rateLimitMiddleware(req: Request, res: Response, next: NextFunction): void {
  const identifier = req.ip || req.socket.remoteAddress || 'unknown';
  
  const limiter = req.path.includes('/requirements') && req.method === 'POST' 
    ? evaluationLimiter 
    : apiLimiter;

  if (!limiter.check(identifier)) {
    res.status(429).json({
      error: {
        message: 'Too many requests, please try again later',
        code: 'RATE_LIMITED',
        retryAfter: 60,
      },
    });
    return;
  }

  next();
}

export { RateLimiter };
