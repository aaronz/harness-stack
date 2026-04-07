import { Request, Response, NextFunction } from 'express';

const auditLogs: { timestamp: string; method: string; path: string; statusCode?: number; duration?: number }[] = [];

export const auditMiddleware = (req: Request, res: Response, next: NextFunction): void => {
  const start = Date.now();
  res.on('finish', () => {
    auditLogs.push({ timestamp: new Date().toISOString(), method: req.method, path: req.path, statusCode: res.statusCode, duration: Date.now() - start });
    if (auditLogs.length > 10000) auditLogs.shift();
    console.log(`[AUDIT] ${req.method} ${req.path} ${res.statusCode} ${Date.now() - start}ms`);
  });
  next();
};

export function getAuditLogs(limit = 100) {
  return auditLogs.slice(-limit);
}