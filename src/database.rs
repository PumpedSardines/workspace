use homedir::get_my_home;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::str::FromStr;

pub struct Database {
    pool: SqlitePool,
}

#[derive(sqlx::FromRow, Clone)]
pub struct Workspace {
    pub name: String,
    pub path: String,
}

impl Database {
    pub fn default_db_path() -> PathBuf {
        let current_dir = get_my_home().unwrap().unwrap();
        let path = current_dir.join(".workspace.db");
        path
    }

    pub fn from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn from_path(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = path.to_str().ok_or("Couldn't convert path to string")?;
        let options = SqliteConnectOptions::from_str(db_url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn migrate_legacy_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let has_migrated = sqlx::query!(
            r#"
            SELECT has_moved FROM legacy_move_data
            "#
        )
        .fetch_optional(&self.pool)
        .await?;

        if has_migrated.is_some() {
            return Ok(());
        }

        let workspaces = crate::legacy::load();

        for workspace in workspaces {
            if self
                .add(workspace.name.clone(), workspace.path)
                .await
                .is_err()
            {
                eprintln!("Failed to migrate workspace: {}", workspace.name);
            }
        }

        sqlx::query!(
            r#"
            INSERT INTO legacy_move_data (has_moved)
            VALUES (1)
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::from_path(Self::default_db_path()).await?)
    }

    pub async fn has_with_name(&self, name: String) -> Result<bool, Box<dyn std::error::Error>> {
        let workspace = sqlx::query!(
            r#"
            SELECT * FROM workspaces
            WHERE name = ?
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(workspace.is_some())
    }

    pub async fn has_with_path(&self, path: String) -> Result<bool, Box<dyn std::error::Error>> {
        let workspace = sqlx::query!(
            r#"
            SELECT * FROM workspaces
            WHERE path = ?
            "#,
            path
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(workspace.is_some())
    }

    pub async fn add(&self, name: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO workspaces (name, path)
            VALUES (?, ?)
            "#,
            name,
            path
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get(&self, name: String) -> Result<Option<Workspace>, Box<dyn std::error::Error>> {
        let workspace = sqlx::query_as!(
            Workspace,
            r#"
            SELECT * FROM workspaces
            WHERE name = ?
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(workspace)
    }

    pub async fn list(&self) -> Result<Vec<Workspace>, Box<dyn std::error::Error>> {
        let workspaces = sqlx::query_as!(Workspace, "SELECT * FROM workspaces")
            .fetch_all(&self.pool)
            .await?;

        Ok(workspaces)
    }

    pub async fn remove(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM workspaces
            WHERE name = ?
            "#,
            name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
