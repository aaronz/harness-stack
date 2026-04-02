import { Router, Request, Response } from 'express';
import { runQuery, getOne, getAll } from '../../db/schema';
import { v4 as uuidv4 } from 'uuid';

const router = Router();

interface ProviderRow {
  id: string;
  name: string;
  model: string;
  config: string;
  is_default: number;
  is_local: number;
}

router.get('/providers', (req: Request, res: Response) => {
  try {
    const rows = getAll('SELECT * FROM llm_providers') as ProviderRow[];

    res.json({
      providers: rows.map((p) => ({
        id: p.id,
        name: p.name,
        model: p.model,
        config: JSON.parse(p.config),
        isDefault: p.is_default === 1,
        isLocal: p.is_local === 1,
      })),
    });
  } catch (error) {
    console.error('Error fetching providers:', error);
    res.status(500).json({
      error: {
        message: 'Failed to fetch providers',
        code: 'FETCH_ERROR',
      },
    });
  }
});

router.put('/providers/:id', (req: Request, res: Response) => {
  try {
    const { id } = req.params;
    const { model, config, isDefault, isLocal } = req.body;

    const existing = getOne('SELECT * FROM llm_providers WHERE id = ?', [id]) as ProviderRow | null;

    if (!existing) {
      res.status(404).json({
        error: {
          message: 'Provider not found',
          code: 'NOT_FOUND',
        },
      });
      return;
    }

    if (isDefault) {
      runQuery('UPDATE llm_providers SET is_default = 0', []);
    }

    const updatedConfig = config ? { ...JSON.parse(existing.config), ...config } : JSON.parse(existing.config);

    runQuery(
      `UPDATE llm_providers SET model = ?, config = ?, is_default = ?, is_local = ? WHERE id = ?`,
      [
        model || existing.model,
        JSON.stringify(updatedConfig),
        isDefault ? 1 : 0,
        isLocal !== undefined ? (isLocal ? 1 : 0) : existing.is_local,
        id,
      ]
    );

    const updated = getOne('SELECT * FROM llm_providers WHERE id = ?', [id]) as ProviderRow;

    res.json({
      provider: {
        id: updated.id,
        name: updated.name,
        model: updated.model,
        config: JSON.parse(updated.config),
        isDefault: updated.is_default === 1,
        isLocal: updated.is_local === 1,
      },
    });
  } catch (error) {
    console.error('Error updating provider:', error);
    res.status(500).json({
      error: {
        message: 'Failed to update provider',
        code: 'UPDATE_ERROR',
      },
    });
  }
});

router.post('/providers', (req: Request, res: Response) => {
  try {
    const { name, model, config, isDefault, isLocal } = req.body;

    if (!name || !model) {
      res.status(400).json({
        error: {
          message: 'Name and model are required',
          code: 'MISSING_FIELDS',
        },
      });
      return;
    }

    const id = uuidv4();

    if (isDefault) {
      runQuery('UPDATE llm_providers SET is_default = 0', []);
    }

    runQuery(
      'INSERT INTO llm_providers (id, name, model, config, is_default, is_local) VALUES (?, ?, ?, ?, ?, ?)',
      [id, name, model, JSON.stringify(config || {}), isDefault ? 1 : 0, isLocal ? 1 : 0]
    );

    res.status(201).json({
      provider: {
        id,
        name,
        model,
        config: config || {},
        isDefault: isDefault || false,
        isLocal: isLocal || false,
      },
    });
  } catch (error) {
    console.error('Error creating provider:', error);
    res.status(500).json({
      error: {
        message: 'Failed to create provider',
        code: 'CREATE_ERROR',
      },
    });
  }
});

export default router;
