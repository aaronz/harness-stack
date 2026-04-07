import initSqlJs, { Database as SqlJsDatabase } from 'sql.js';
import { v4 as uuidv4 } from 'uuid';
import * as fs from 'fs';
import * as path from 'path';

const DB_PATH = process.env.DB_PATH || './data/ai-evaluator.db';

let db: SqlJsDatabase | null = null;

async function ensureDbDir(): Promise<void> {
  const dir = path.dirname(DB_PATH);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function saveDb(): void {
  if (db) {
    const data = db.export();
    const buffer = Buffer.from(data);
    fs.writeFileSync(DB_PATH, buffer);
  }
}

export async function initDb(): Promise<SqlJsDatabase> {
  if (db) return db;

  await ensureDbDir();

  const SQL = await initSqlJs();

  if (fs.existsSync(DB_PATH)) {
    const buffer = fs.readFileSync(DB_PATH);
    db = new SQL.Database(buffer);
  } else {
    db = new SQL.Database();
  }

  db.run(`
    CREATE TABLE IF NOT EXISTS requirements (
      id TEXT PRIMARY KEY,
      text TEXT NOT NULL,
      submitted_at TEXT NOT NULL,
      submitted_by TEXT,
      version INTEGER DEFAULT 1
    );

    CREATE TABLE IF NOT EXISTS scores (
      id TEXT PRIMARY KEY,
      requirement_id TEXT NOT NULL,
      total_score REAL NOT NULL,
      level TEXT NOT NULL,
      context_score REAL NOT NULL,
      atomicity_score REAL NOT NULL,
      boundary_score REAL NOT NULL,
      verifiability_score REAL NOT NULL,
      technical_score REAL NOT NULL,
      complexity_penalty REAL NOT NULL,
      calculated_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS risk_highlights (
      id TEXT PRIMARY KEY,
      score_id TEXT NOT NULL,
      dimension TEXT NOT NULL,
      text_span TEXT NOT NULL,
      reason TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS optimization_suggestions (
      id TEXT PRIMARY KEY,
      score_id TEXT NOT NULL,
      category TEXT NOT NULL,
      description TEXT NOT NULL,
      priority TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS llm_providers (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      model TEXT NOT NULL,
      config TEXT NOT NULL,
      is_default INTEGER DEFAULT 0,
      is_local INTEGER DEFAULT 0
    );
  `);

  const result = db.exec('SELECT COUNT(*) as count FROM llm_providers');
  const count = result[0]?.values[0]?.[0] as number || 0;
  if (count === 0) {
    db.run(
      'INSERT INTO llm_providers (id, name, model, config, is_default, is_local) VALUES (?, ?, ?, ?, ?, ?)',
      [uuidv4(), 'OpenAI', 'gpt-4o', JSON.stringify({ apiKey: '' }), 1, 0]
    );
  }

  saveDb();
  return db;
}

export function getDb(): SqlJsDatabase {
  if (!db) throw new Error('Database not initialized. Call initDb() first.');
  return db;
}

export function runQuery(sql: string, params: any[] = []): void {
  getDb().run(sql, params);
  saveDb();
}

export function getOne(sql: string, params: any[] = []): any {
  const stmt = getDb().prepare(sql);
  stmt.bind(params);
  if (stmt.step()) {
    const row = stmt.getAsObject();
    stmt.free();
    return row;
  }
  stmt.free();
  return null;
}

export function getAll(sql: string, params: any[] = []): any[] {
  const stmt = getDb().prepare(sql);
  stmt.bind(params);
  const results: any[] = [];
  while (stmt.step()) results.push(stmt.getAsObject());
  stmt.free();
  return results;
}

export function closeDb(): void {
  if (db) {
    saveDb();
    db.close();
    db = null;
  }
}