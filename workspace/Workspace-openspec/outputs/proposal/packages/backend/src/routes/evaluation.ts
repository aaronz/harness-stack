import { Router } from 'express';
import { prisma } from '../index.js';
import { AuthRequest } from '../middleware/auth.js';
import { evaluateRequirement } from '../services/evaluation.js';

const router = Router();

router.post('/', async (req: AuthRequest, res) => {
  try {
    const { requirementText, requirementFormat = 'text', complexity = 'MEDIUM' } = req.body;
    const userId = req.user!.id;

    const evaluation = await prisma.evaluation.create({
      data: {
        userId,
        requirementText,
        requirementFormat,
        complexity,
        status: 'PENDING'
      }
    });

    evaluateRequirement(evaluation.id).catch(console.error);

    res.json({ id: evaluation.id, status: evaluation.status });
  } catch (error) {
    res.status(500).json({ error: 'Failed to create evaluation' });
  }
});

router.get('/:id', async (req: AuthRequest, res) => {
  try {
    const { id } = req.params;
    const userId = req.user!.id;

    const evaluation = await prisma.evaluation.findFirst({
      where: { id, userId },
      include: { result: true }
    });

    if (!evaluation) {
      return res.status(404).json({ error: 'Evaluation not found' });
    }

    res.json(evaluation);
  } catch (error) {
    res.status(500).json({ error: 'Failed to get evaluation' });
  }
});

router.get('/', async (req: AuthRequest, res) => {
  try {
    const userId = req.user!.id;
    const { page = 1, limit = 20 } = req.query;

    const skip = (Number(page) - 1) * Number(limit);

    const [evaluations, total] = await Promise.all([
      prisma.evaluation.findMany({
        where: { userId },
        include: { result: true },
        orderBy: { createdAt: 'desc' },
        skip,
        take: Number(limit)
      }),
      prisma.evaluation.count({ where: { userId } })
    ]);

    res.json({ evaluations, total, page: Number(page), limit: Number(limit) });
  } catch (error) {
    res.status(500).json({ error: 'Failed to get evaluations' });
  }
});

router.post('/batch', async (req: AuthRequest, res) => {
  try {
    const { requirements } = req.body;
    const userId = req.user!.id;

    const evaluations = await Promise.all(
      requirements.map((req: { text: string; format?: string; complexity?: string }) =>
        prisma.evaluation.create({
          data: {
            userId,
            requirementText: req.text,
            requirementFormat: req.format || 'text',
            complexity: (req.complexity as any) || 'MEDIUM',
            status: 'PENDING'
          }
        })
      )
    );

    evaluations.forEach(e => evaluateRequirement(e.id).catch(console.error));

    res.json({
      batchId: `batch_${Date.now()}`,
      total: evaluations.length,
      results: evaluations.map((e, i) => ({
        index: i,
        evaluationId: e.id,
        status: 'processing'
      }))
    });
  } catch (error) {
    res.status(500).json({ error: 'Failed to create batch evaluation' });
  }
});

export default router;