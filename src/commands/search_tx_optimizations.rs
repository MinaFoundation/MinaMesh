use anyhow::{bail, Result};
use clap::Args;
use sqlx::{migrate::Migrator, PgPool};

static MIGRATOR: Migrator = sqlx::migrate!("sql/migrations");

#[derive(Debug, Args)]
#[command(about = "Command to apply or drop search transaction optimizations in the archive database.")]
pub struct SearchTxOptimizationsCommand {
  /// URL to the archive database
  #[arg(long, env = "MINAMESH_ARCHIVE_DATABASE_URL")]
  archive_database_url: String,

  /// Apply optimizations
  #[arg(long, conflicts_with = "drop")]
  apply: bool,

  /// Drop optimizations
  #[arg(long, conflicts_with = "apply")]
  drop: bool,

  /// Check if optimizations are applied
  #[arg(long)]
  check: bool,
}

impl SearchTxOptimizationsCommand {
  pub async fn run(&self) -> Result<()> {
    // Connect to the database
    let pool = PgPool::connect(&self.archive_database_url).await?;

    // Check if optimizations are already applied
    if self.apply && self.check_if_optimizations_applied(&pool).await? {
      bail!("Search transaction optimizations are already applied.");
    } else if self.drop && !self.check_if_optimizations_applied(&pool).await? {
      bail!("Search transaction optimizations are not applied.");
    }

    if self.apply {
      self.apply_optimizations(&pool).await?;
    } else if self.drop {
      self.drop_optimizations(&pool).await?;
    } else if self.check {
      let applied = self.check_if_optimizations_applied(&pool).await?;
      if applied {
        println!("Search transaction optimizations are already applied.");
      } else {
        println!("Search transaction optimizations are not applied.");
      }
    } else {
      bail!("You must specify either --apply or --drop or --check.");
    }
    Ok(())
  }

  async fn apply_optimizations(&self, pool: &PgPool) -> Result<()> {
    println!("Applying search transaction optimizations on Archive Database (this may take a few minutes)...");

    MIGRATOR.run(pool).await?;

    println!("Optimizations applied successfully.");
    Ok(())
  }

  async fn drop_optimizations(&self, pool: &PgPool) -> Result<()> {
    println!("Dropping search transaction optimizations from Archive Database...");

    MIGRATOR.undo(pool, 0).await?;

    println!("Optimizations dropped successfully.");
    Ok(())
  }

  async fn check_if_optimizations_applied(&self, pool: &PgPool) -> Result<bool> {
    // Check if the migrations table exists
    let table_exists: Option<String> =
      sqlx::query_scalar("SELECT to_regclass('_sqlx_migrations')::text").fetch_one(pool).await?;

    if table_exists.is_none() {
      // The table doesn't exist so the optimizations have not been applied
      return Ok(false);
    }

    // select the latest migration version in the DB
    let result: Option<i64> = sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations").fetch_one(pool).await?;
    let db_latest_version = result.unwrap_or(0);

    // get the latest migration from MIGRATOR
    let latest_version = MIGRATOR.iter().fold(0, |acc, m| acc.max(m.version));

    // check if the latest migration version is the same as the latest version in
    // the MIGRATOR
    Ok(latest_version == db_latest_version)
  }
}
