PRAGMA foreign_keys = ON;

CREATE TABLE accounts (
  id TEXT PRIMARY KEY,
  provider_id TEXT NOT NULL,
  external_account_ref TEXT,
  credential_ref TEXT NOT NULL,
  source TEXT NOT NULL,
  status TEXT NOT NULL,
  region_hint TEXT,
  tags_json TEXT NOT NULL DEFAULT '[]',
  allowed_models_json TEXT NOT NULL DEFAULT '[]',
  health_score REAL NOT NULL DEFAULT 1.0,
  risk_score REAL NOT NULL DEFAULT 0.0,
  success_rate_1h REAL,
  avg_latency_ms_1h REAL,
  last_success_at TEXT,
  last_failure_at TEXT,
  cooldown_until TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (provider_id) REFERENCES providers(id)
);

CREATE INDEX idx_accounts_provider_id ON accounts(provider_id);
CREATE INDEX idx_accounts_status ON accounts(status);

CREATE TABLE account_pools (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  purpose TEXT NOT NULL,
  provider_class TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  priority INTEGER NOT NULL DEFAULT 100,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE pool_memberships (
  id TEXT PRIMARY KEY,
  pool_id TEXT NOT NULL,
  member_type TEXT NOT NULL,
  member_id TEXT NOT NULL,
  weight REAL NOT NULL DEFAULT 1.0,
  priority INTEGER NOT NULL DEFAULT 100,
  status TEXT NOT NULL DEFAULT 'active',
  joined_at TEXT NOT NULL,
  FOREIGN KEY (pool_id) REFERENCES account_pools(id)
);

CREATE INDEX idx_pool_memberships_pool_id ON pool_memberships(pool_id);
CREATE INDEX idx_pool_memberships_member ON pool_memberships(member_type, member_id);

CREATE TABLE browser_profiles (
  id TEXT PRIMARY KEY,
  isolation_level TEXT NOT NULL,
  storage_ref TEXT NOT NULL,
  proxy_binding TEXT,
  fingerprint_policy_id TEXT,
  region TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  created_at TEXT NOT NULL,
  last_used_at TEXT
);

CREATE TABLE egress_profiles (
  id TEXT PRIMARY KEY,
  provider TEXT NOT NULL,
  region TEXT,
  static_ip TEXT,
  use_case TEXT NOT NULL,
  risk_level TEXT NOT NULL DEFAULT 'normal',
  health_score REAL NOT NULL DEFAULT 1.0,
  cost_score REAL NOT NULL DEFAULT 0.0,
  current_load REAL NOT NULL DEFAULT 0.0,
  cooldown_until TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  last_checked_at TEXT
);

CREATE TABLE web_sessions (
  id TEXT PRIMARY KEY,
  provider_id TEXT NOT NULL,
  account_id TEXT,
  browser_profile_id TEXT NOT NULL,
  egress_profile_id TEXT,
  login_state TEXT NOT NULL DEFAULT 'unknown',
  challenge_state TEXT,
  warmup_state TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  health_score REAL NOT NULL DEFAULT 1.0,
  risk_score REAL NOT NULL DEFAULT 0.0,
  last_success_at TEXT,
  last_failure_at TEXT,
  cooldown_until TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (provider_id) REFERENCES providers(id),
  FOREIGN KEY (account_id) REFERENCES accounts(id),
  FOREIGN KEY (browser_profile_id) REFERENCES browser_profiles(id),
  FOREIGN KEY (egress_profile_id) REFERENCES egress_profiles(id)
);
