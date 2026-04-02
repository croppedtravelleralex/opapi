PRAGMA foreign_keys = ON;

CREATE TABLE health_states (
  id TEXT PRIMARY KEY,
  subject_type TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  score REAL NOT NULL,
  state TEXT NOT NULL,
  success_rate_1m REAL,
  success_rate_15m REAL,
  avg_latency_ms_1m REAL,
  recent_error_types_json TEXT NOT NULL DEFAULT '[]',
  updated_at TEXT NOT NULL
);

CREATE INDEX idx_health_states_subject
  ON health_states(subject_type, subject_id);

CREATE TABLE circuit_states (
  id TEXT PRIMARY KEY,
  subject_type TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  state TEXT NOT NULL,
  opened_at TEXT,
  retry_after TEXT,
  trigger_reason TEXT
);

CREATE TABLE capacity_states (
  id TEXT PRIMARY KEY,
  subject_type TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  current_qps REAL,
  current_concurrency REAL,
  saturation_level REAL,
  recommended_action TEXT,
  updated_at TEXT NOT NULL
);
