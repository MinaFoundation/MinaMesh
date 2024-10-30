use anyhow::{bail, Result};
use clap::Args;
use sqlx::{PgPool, Row};

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
    println!("Applying search transaction optimizations on Archive Database (this may take few minutes)...");

    // Load and execute the SQL from the file
    let sql = include_str!("../../sql/migrations/apply_search_tx_optimizations.sql");
    self.execute_sql_file(pool, sql, "-- NEXT --").await?;

    println!("Optimizations applied successfully.");
    Ok(())
  }

  async fn drop_optimizations(&self, pool: &PgPool) -> Result<()> {
    println!("Dropping search transaction optimizations from Archive Database...");

    // Load and execute the SQL from the file
    let sql = include_str!("../../sql/migrations/drop_search_tx_optimizations.sql");
    self.execute_sql_file(pool, sql, ";").await?;

    println!("Optimizations dropped successfully.");
    Ok(())
  }

  async fn check_if_optimizations_applied(&self, pool: &PgPool) -> Result<bool> {
    // select if table exists
    let result = sqlx::query(
      "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'user_commands_aggregated')",
    )
    .fetch_one(pool)
    .await?;
    Ok(result.get(0))
  }

  async fn execute_sql_file(&self, pool: &PgPool, file_content: &str, split_by: &str) -> Result<()> {
    let statements: Vec<&str> = file_content.split(split_by).filter(|stmt| !stmt.trim().is_empty()).collect();
    for stmt in statements {
      sqlx::query(stmt).execute(pool).await?;
    }
    Ok(())
  }
}
