import { Router } from 'express';
import { prisma } from '../index.js';
import { AuthRequest } from '../middleware/auth.js';

const router = Router();

router.get('/', async (req: AuthRequest, res) => {
  const providers = await prisma.lLMProvider.findMany({
    orderBy: { priority: 'asc' }
  });
  res.json(providers);
});

router.post('/', async (req: AuthRequest, res) => {
  const { name, provider, model, endpoint, apiKey, maxTokens, temperature, timeout, priority, enabled } = req.body;

  const newProvider = await prisma.lLMProvider.create({
    data: {
      name,
      provider,
      model,
      endpoint,
      apiKey,
      maxTokens: maxTokens || 4096,
      temperature: temperature || 0.7,
      timeout: timeout || 30000,
      priority: priority || 1,
      enabled: enabled ?? true
    }
  });

  res.json(newProvider);
});

router.put('/:id', async (req: AuthRequest, res) => {
  const { id } = req.params;
  const { name, model, endpoint, apiKey, maxTokens, temperature, timeout, priority, enabled } = req.body;

  const updated = await prisma.lLMProvider.update({
    where: { id },
    data: {
      ...(name && { name }),
      ...(model && { model }),
      ...(endpoint && { endpoint }),
      ...(apiKey && { apiKey }),
      ...(maxTokens && { maxTokens }),
      ...(temperature && { temperature }),
      ...(timeout && { timeout }),
      ...(priority && { priority }),
      ...(enabled !== undefined && { enabled })
    }
  });

  res.json(updated);
});

router.delete('/:id', async (req: AuthRequest, res) => {
  const { id } = req.params;
  await prisma.lLMProvider.delete({ where: { id } });
  res.json({ success: true });
});

export default router;