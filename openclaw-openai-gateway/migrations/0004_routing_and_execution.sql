PRAGMA foreign_keys = ON;

CREATE TABLE route_policies (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  target_models_json TEXT NOT NULL DEFAULT '[]',
  allowed_provider_classes_json TEXT NOT NULL DEFAULT '[]',
  allowed_pools_json TEXT NOT NULL DEFAULT '[]',
  allowed_regions_json TEXT NOT NULL DEFAULT '[]',
  selection_strategy TEXT NOT NULL,
  fallback_chain_json TEXT NOT NULL DEFAULT '[]',
  circuit_breaker_policy_id TEXT,
  cost_weight REAL NOT NULL DEFAULT 0.2,
  latency_weight REAL NOT NULL DEFAULT 0.2,
  reliability_weight REAL NOT NULL DEFAULT 0.5,
  risk_weight REAL NOT NULL DEFAULT 0.1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE routing_decisions (
  id TEXT PRIMARY KEY,
  request_id TEXT NOT NULL UNIQUE,
  tenant_id TEXT,
  project_id TEXT,
  api_key_id TEXT,
  model_name TEXT NOT NULL,
  provider_class_selected TEXT,
  provider_id TEXT,
  pool_id TEXT,
  account_or_session_id TEXT,
  egress_profile_id TEXT,
  fallback_chain_snapshot_json TEXT NOT NULL DEFAULT '[]',
  selection_reason_json TEXT NOT NULL DEFAULT '[]',
  downgrade_reason TEXT,
  rejection_reason TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (tenant_id) REFERENCES tenants(id),
  FOREIGN KEY (project_id) REFERENCES projects(id),
  FOREIGN KEY (api_key_id) REFERENCES api_keys(id)
);

CREATE INDEX idx_routing_decisions_model_name ON routing_decisions(model_name);
CREATE INDEX idx_routing_decisions_api_key_id ON routing_decisions(api_key_id);

CREATE TABLE execution_attempts (
  id TEXT PRIMARY KEY,
  routing_decision_id TEXT NOT NULL,
  attempt_index INTEGER NOT NULL,
  executor_type TEXT NOT NULL,
  upstream_request_ref TEXT,
  started_at TEXT NOT NULL,
  ended_at TEXT,
  status TEXT NOT NULL,
  latency_ms INTEGER,
  error_type TEXT,
  error_detail_redacted TEXT,
  FOREIGN KEY (routing_decision_id) REFERENCES routing_decisions(id)
);

CREATE INDEX idx_execution_attempts_routing_decision_id
  ON execution_attempts(routing_decision_id);

CREATE TABLE stream_sessions (
  id TEXT PRIMARY KEY,
  request_id TEXT NOT NULL,
  routing_decision_id TEXT NOT NULL,
  protocol TEXT NOT NULL,
  first_token_at TEXT,
  ended_at TEXT,
  status TEXT NOT NULL,
  close_reason TEXT,
  FOREIGN KEY (routing_decision_id) REFERENCES routing_decisions(id)
);
