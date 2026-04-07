import { Router, Request, Response } from 'express';

const router = Router();

router.post('/', (req: Request, res: Response) => {
  const { requirementText } = req.body;

  if (!requirementText) {
    return res.status(400).json({ error: 'requirementText is required' });
  }

  res.json({
    message: 'Evaluation endpoint - to be implemented',
    requirementText
  });
});

export default router;
