import { Request, Response, NextFunction } from 'express';

export function validateBody(fields: string[]) {
  return (req: Request, res: Response, next: NextFunction): void => {
    const missing = fields.filter((f) => !req.body[f]);
    if (missing.length > 0) {
      res.status(400).json({ error: { message: `Missing required fields: ${missing.join(', ')}`, code: 'MISSING_FIELDS' } });
      return;
    }
    next();
  };
}

export function requireFields(fields: string[]) {
  return validateBody(fields);
}