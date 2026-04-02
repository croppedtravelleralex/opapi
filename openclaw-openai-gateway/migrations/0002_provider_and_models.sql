PRAGMA foreign_keys = ON;

CREATE TABLE providers (
  id TEXT PRIMARY KEY,
  class TEXT NOT NULL,
  vendor TEXT NOT NULL,
  name TEXT NOT NULL,
  protocol TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1,
  supports_stream INTEGER NOT NULL DEFAULT 0,
  supports_responses_shape INTEGER NOT NULL DEFAULT 0,
  health_score REAL NOT NULL DEFAULT 1.0,
  risk_level TEXT NOT NULL DEFAULT 'normal',
  cost_score REAL NOT NULL DEFAULT 0.0,
  latency_score REAL NOT NULL DEFAULT 0.0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE provider_capabilities (
  id TEXT PRIMARY KEY,
  provider_id TEXT NOT NULL,
  model_name TEXT NOT NULL,
  input_modes_json TEXT NOT NULL DEFAULT '[]',
  output_modes_json TEXT NOT NULL DEFAULT '[]',
  supports_stream INTEGER NOT NULL DEFAULT 0,
  supports_tools INTEGER NOT NULL DEFAULT 0,
  supports_responses_api INTEGER NOT NULL DEFAULT 0,
  supports_embeddings INTEGER NOT NULL DEFAULT 0,
  max_context_tokens INTEGER,
  reliability_tier TEXT,
  FOREIGN KEY (provider_id) REFERENCES providers(id)
);

CREATE INDEX idx_provider_capabilities_provider_id
  ON provider_capabilities(provider_id);

CREATE TABLE model_catalog (
  id TEXT PRIMARY KEY,
  canonical_name TEXT NOT NULL UNIQUE,
  aliases_json TEXT NOT NULL DEFAULT '[]',
  family TEXT,
  vendor TEXT,
  lifecycle TEXT NOT NULL DEFAULT 'active',
  capability_tags_json TEXT NOT NULL DEFAULT '[]',
  default_route_policy_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE model_availability (
  id TEXT PRIMARY KEY,
  model_id TEXT NOT NULL,
  provider_id TEXT NOT NULL,
  pool_id TEXT,
  region TEXT,
  status TEXT NOT NULL DEFAULT 'available',
  health_score REAL NOT NULL DEFAULT 1.0,
  success_rate REAL,
  avg_latency_ms REAL,
  last_checked_at TEXT,
  FOREIGN KEY (model_id) REFERENCES model_catalog(id),
  FOREIGN KEY (provider_id) REFERENCES providers(id)
);
