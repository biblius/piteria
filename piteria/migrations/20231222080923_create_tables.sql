CREATE TABLE IF NOT EXISTS deployments (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS nginx_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    deployment_id INT NOT NULL REFERENCES deployments(id),
    file_path TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sysd_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    deployment_id INT NOT NULL REFERENCES deployments(id),
    file_path TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);