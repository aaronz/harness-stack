import 'dotenv/config';
import express from 'express';
import cors from 'cors';
import { PrismaClient } from '@prisma/client';
import authRoutes from './routes/auth.js';
import evaluationRoutes from './routes/evaluation.js';
import providerRoutes from './routes/provider.js';
import configRoutes from './routes/config.js';
import { authMiddleware } from './middleware/auth.js';

export const prisma = new PrismaClient();

const app = express();
const PORT = process.env.PORT || 3001;

app.use(cors());
app.use(express.json());

// Public routes
app.use('/api/auth', authRoutes);

// Protected routes
app.use('/api/evaluations', authMiddleware, evaluationRoutes);
app.use('/api/providers', authMiddleware, providerRoutes);
app.use('/api/config', authMiddleware, configRoutes);

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});

export default app;