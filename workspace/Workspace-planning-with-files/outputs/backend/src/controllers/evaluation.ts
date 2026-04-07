import { Request, Response } from 'express';

export const evaluateRequirement = (req: Request, res: Response) => {
  const { requirementText } = req.body;

  if (!requirementText) {
    return res.status(400).json({ error: 'requirementText is required' });
  }

  res.json({
    message: 'Evaluation controller - to be implemented',
    requirementText
  });
};
