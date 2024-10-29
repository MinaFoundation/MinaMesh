use tokio::{select, signal};

pub async fn shutdown_signal() {
  let windows = async {
    signal::ctrl_c().await.unwrap_or_else(|_| panic!("Error: failed to install windows shutdown handler"));
  };

  #[cfg(unix)]
  let unix = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .unwrap_or_else(|_| panic!("Error: failed to install unix shutdown handler"))
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  select! {
    () = windows => {},
    () = unix => {},
  }

  println!("Signal received - starting graceful shutdown...");
}
