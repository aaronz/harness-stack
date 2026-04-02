import { Request, Response, NextFunction } from 'express';

interface AuditLogEntry {
  timestamp: string;
  method: string;
  path: string;
  statusCode?: number;
  userId?: string;
  duration?: number;
  ip?: string;
}

class AuditLogger {
  private logs: AuditLogEntry[] = [];
  private maxLogs = 10000;

  log(entry: Omit<AuditLogEntry, 'timestamp'>): void {
    const logEntry: AuditLogEntry = {
      ...entry,
      timestamp: new Date().toISOString(),
    };
    this.logs.push(logEntry);
    if (this.logs.length > this.maxLogs) {
      this.logs.shift();
    }
    console.log(`[AUDIT] ${logEntry.timestamp} ${logEntry.method} ${logEntry.path} ${logEntry.statusCode || ''}`);
  }

  getLogs(limit = 100): AuditLogEntry[] {
    return this.logs.slice(-limit);
  }

  getLogsByPath(path: string, limit = 50): AuditLogEntry[] {
    return this.logs.filter((l) => l.path === path).slice(-limit);
  }

  getLogsByUser(userId: string, limit = 50): AuditLogEntry[] {
    return this.logs.filter((l) => l.userId === userId).slice(-limit);
  }
}

export const auditLogger = new AuditLogger();

export function auditMiddleware(req: Request, res: Response, next: NextFunction): void {
  const startTime = Date.now();

  res.on('finish', () => {
    const duration = Date.now() - startTime;
    auditLogger.log({
      method: req.method,
      path: req.path,
      statusCode: res.statusCode,
      duration,
      ip: req.ip || req.socket.remoteAddress,
    });
  });

  next();
}
