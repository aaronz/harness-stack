import { Router } from 'express';
import { prisma } from '../index.js';
import { AuthRequest } from '../middleware/auth.js';

const router = Router();

const DEFAULT_CONFIG = {
  dimensionWeights: {
    contextSufficiency: 0.25,
    atomicity: 0.25,
    boundaryDefiniteness: 0.20,
    verifiability: 0.15,
    technicalConstraints: 0.15
  },
  complexityPenalties: {
    SIMPLE: 1.0,
    MEDIUM: 0.9,
    COMPLEX: 0.7
  },
  thresholds: {
    S: 90,
    A: 75,
    B: 60
  }
};

router.get('/', async (req: AuthRequest, res) => {
  let config = await prisma.systemConfig.findUnique({ where: { key: 'evaluation_config' } });
  
  if (!config) {
    config = await prisma.systemConfig.create({
      data: { key: 'evaluation_config', value: JSON.stringify(DEFAULT_CONFIG) }
    });
  }

  res.json(JSON.parse(config.value));
});

router.put('/', async (req: AuthRequest, res) => {
  const { dimensionWeights, complexityPenalties, thresholds } = req.body;

  const currentConfigStr = (await prisma.systemConfig.findUnique({ where: { key: 'evaluation_config' } }))?.value || JSON.stringify(DEFAULT_CONFIG);
  const currentConfig = JSON.parse(currentConfigStr);

  const updatedConfig = {
    ...currentConfig,
    ...(dimensionWeights && { dimensionWeights }),
    ...(complexityPenalties && { complexityPenalties }),
    ...(thresholds && { thresholds })
  };

  const config = await prisma.systemConfig.upsert({
    where: { key: 'evaluation_config' },
    update: { value: JSON.stringify(updatedConfig) },
    create: { key: 'evaluation_config', value: JSON.stringify(updatedConfig) }
  });

  res.json(JSON.parse(config.value));
});

export default router;