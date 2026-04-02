PRAGMA foreign_keys = ON;

CREATE TABLE tenants (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  status TEXT NOT NULL DEFAULT 'active',
  tier TEXT NOT NULL DEFAULT 'default',
  allowed_provider_classes_json TEXT NOT NULL DEFAULT '[]',
  allowed_models_json TEXT NOT NULL DEFAULT '[]',
  quota_policy_id TEXT,
  billing_policy_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  name TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  env TEXT NOT NULL DEFAULT 'prod',
  allowed_models_json TEXT NOT NULL DEFAULT '[]',
  route_policy_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

CREATE UNIQUE INDEX idx_projects_tenant_name
  ON projects(tenant_id, name);

CREATE TABLE api_keys (
  id TEXT PRIMARY KEY,
  tenant_id TEXT,
  project_id TEXT,
  key_hash TEXT NOT NULL UNIQUE,
  label TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  scopes_json TEXT NOT NULL DEFAULT '[]',
  allowed_models_json TEXT NOT NULL DEFAULT '[]',
  rate_limit_policy_id TEXT,
  expires_at TEXT,
  last_used_at TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (tenant_id) REFERENCES tenants(id),
  FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE role_bindings (
  id TEXT PRIMARY KEY,
  subject_type TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  tenant_id TEXT,
  project_id TEXT,
  role TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (tenant_id) REFERENCES tenants(id),
  FOREIGN KEY (project_id) REFERENCES projects(id)
);
