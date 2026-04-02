import { v4 as uuidv4 } from 'uuid';
import { runQuery, getOne, getAll } from '../db/schema';

export interface Requirement {
  id: string;
  text: string;
  submittedAt: string;
  submittedBy: string | null;
  version: number;
}

export interface CreateRequirementInput {
  text: string;
  submittedBy?: string;
}

export class RequirementRepository {
  create(input: CreateRequirementInput): Requirement {
    const id = uuidv4();
    const submittedAt = new Date().toISOString();

    runQuery(
      'INSERT INTO requirements (id, text, submitted_at, submitted_by, version) VALUES (?, ?, ?, ?, 1)',
      [id, input.text, submittedAt, input.submittedBy || null]
    );

    return {
      id,
      text: input.text,
      submittedAt,
      submittedBy: input.submittedBy || null,
      version: 1,
    };
  }

  findById(id: string): Requirement | null {
    const row = getOne('SELECT * FROM requirements WHERE id = ?', [id]);
    if (!row) return null;

    return {
      id: row.id,
      text: row.text,
      submittedAt: row.submitted_at,
      submittedBy: row.submitted_by,
      version: row.version,
    };
  }

  findAll(limit = 100, offset = 0): Requirement[] {
    const rows = getAll('SELECT * FROM requirements ORDER BY submitted_at DESC LIMIT ? OFFSET ?', [limit, offset]);

    return rows.map((row) => ({
      id: row.id,
      text: row.text,
      submittedAt: row.submitted_at,
      submittedBy: row.submitted_by,
      version: row.version,
    }));
  }

  update(id: string, text: string): Requirement | null {
    const existing = this.findById(id);
    if (!existing) return null;

    const newVersion = existing.version + 1;
    runQuery('UPDATE requirements SET text = ?, version = ? WHERE id = ?', [text, newVersion, id]);

    return {
      ...existing,
      text,
      version: newVersion,
    };
  }

  delete(id: string): boolean {
    const existing = this.findById(id);
    if (!existing) return false;
    runQuery('DELETE FROM requirements WHERE id = ?', [id]);
    return true;
  }
}

export const requirementRepository = new RequirementRepository();
