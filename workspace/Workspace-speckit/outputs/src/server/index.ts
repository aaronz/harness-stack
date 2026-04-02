import express from 'express';
import cors from 'cors';
import { initDb, closeDb } from '../db/schema';
import { errorHandler, notFoundHandler } from '../middleware/error-handler';
import { providerSelector } from '../middleware/provider-selector';
import { auditMiddleware } from '../middleware/audit-logger';
import { rateLimitMiddleware } from '../middleware/rate-limiter';
import { sanitizeInput } from '../middleware/security';
import requirementsRouter from './routes/requirements';
import configRouter from './routes/config';

const app = express();
const PORT = process.env.PORT || 3001;

app.use(cors());
app.use(express.json({ limit: '10mb' }));
app.use(express.text({ limit: '10mb' }));

app.use(sanitizeInput);
app.use(auditMiddleware);
app.use(rateLimitMiddleware);
app.use(providerSelector);

app.use('/api/requirements', requirementsRouter);
app.use('/api/config', configRouter);

app.get('/api/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

app.use(notFoundHandler);
app.use(errorHandler);

async function startServer(): Promise<void> {
  try {
    await initDb();
    console.log('Database initialized');

    app.listen(PORT, () => {
      console.log(`Server running on http://localhost:${PORT}`);
    });
  } catch (error) {
    console.error('Failed to start server:', error);
    process.exit(1);
  }
}

process.on('SIGINT', () => {
  console.log('Shutting down...');
  closeDb();
  process.exit(0);
});

startServer();
