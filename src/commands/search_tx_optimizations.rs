use std::process;

use anyhow::{Context, Result};
use clap::Args;
use sqlx::{migrate::Migrator, PgPool};

static MIGRATOR: Migrator = sqlx::migrate!("sql/migrations");

#[derive(Debug, Args)]
#[command(
  about = "Command to apply or drop search transaction optimizations in the archive database.",
  long_about = "Command to apply or drop search transaction optimizations in the archive database.
  Exit Codes:
    0 - Optimizations applied (status check or apply success)
    1 - Optimizations not applied (status check or cannot drop because not applied)
    2 - Error applying or dropping optimizations
    3 - Invalid arguments or missing required arguments"
)]
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

    // Apply optimizations
    if self.apply {
      if self.check_if_optimizations_applied(&pool).await? {
        eprintln!("Search transaction optimizations are already applied. No need to apply again.");
        process::exit(0);
      }
      self.apply_optimizations(&pool).await.unwrap_or_else(|err| {
        eprintln!("Error applying optimizations: {:?}", err);
        process::exit(2);
      });
    // Drop optimizations
    } else if self.drop {
      if !self.check_if_optimizations_applied(&pool).await? {
        eprintln!("Cannot drop since search transaction optimizations are not applied.");
        process::exit(1);
      }
      self.drop_optimizations(&pool).await.unwrap_or_else(|err| {
        eprintln!("Error dropping optimizations: {:?}", err);
        process::exit(2);
      });
    // Check if optimizations are applied
    } else if self.check {
      let applied = self.check_if_optimizations_applied(&pool).await?;
      if applied {
        println!("Search transaction optimizations are already applied.");
        process::exit(0);
      } else {
        println!("Search transaction optimizations are not applied.");
        process::exit(1);
      }
    } else {
      eprintln!("You must specify either --apply or --drop or --check.");
      process::exit(3);
    }
    Ok(())
  }

  async fn apply_optimizations(&self, pool: &PgPool) -> Result<()> {
    println!("Applying search transaction optimizations on Archive Database (this may take a few minutes)...");

    MIGRATOR.run(pool).await.with_context(|| "Failed to apply optimizations")?;

    println!("Optimizations applied successfully.");
    Ok(())
  }

  async fn drop_optimizations(&self, pool: &PgPool) -> Result<()> {
    println!("Dropping search transaction optimizations from Archive Database...");

    MIGRATOR.undo(pool, 0).await.with_context(|| "Failed to drop optimizations")?;

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
