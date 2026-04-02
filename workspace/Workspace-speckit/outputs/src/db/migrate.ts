import { getDb } from './schema';

interface Migration {
  version: number;
  name: string;
  up: () => void;
}

const migrations: Migration[] = [
  {
    version: 1,
    name: 'initial_schema',
    up: () => {
      const db = getDb();
      db.exec(`
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
          calculated_at TEXT NOT NULL,
          FOREIGN KEY (requirement_id) REFERENCES requirements(id)
        );

        CREATE TABLE IF NOT EXISTS risk_highlights (
          id TEXT PRIMARY KEY,
          score_id TEXT NOT NULL,
          dimension TEXT NOT NULL,
          text_span TEXT NOT NULL,
          reason TEXT NOT NULL,
          FOREIGN KEY (score_id) REFERENCES scores(id)
        );

        CREATE TABLE IF NOT EXISTS optimization_suggestions (
          id TEXT PRIMARY KEY,
          score_id TEXT NOT NULL,
          category TEXT NOT NULL,
          description TEXT NOT NULL,
          priority TEXT NOT NULL,
          FOREIGN KEY (score_id) REFERENCES scores(id)
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
    },
  },
];

export function runMigrations(): void {
  const db = getDb();
  db.exec(`
    CREATE TABLE IF NOT EXISTS migrations (
      version INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      applied_at TEXT NOT NULL
    );
  `);

  const currentVersionRow = db.prepare('SELECT MAX(version) as version FROM migrations').get() as { version: number | null };
  const currentVersion = currentVersionRow?.version || 0;

  for (const migration of migrations) {
    if (migration.version > currentVersion) {
      console.log(`Running migration ${migration.version}: ${migration.name}`);
      migration.up();
      db.prepare('INSERT INTO migrations (version, name, applied_at) VALUES (?, ?, ?)').run(
        migration.version,
        migration.name,
        new Date().toISOString()
      );
      console.log(`Migration ${migration.version} completed`);
    }
  }

  console.log(`Migrations complete. Current version: ${migrations.length}`);
}

export function getSchemaVersion(): number {
  const db = getDb();
  const result = db.prepare('SELECT MAX(version) as version FROM migrations').get() as { version: number | null };
  return result?.version || 0;
}
