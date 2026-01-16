use std::net::{IpAddr, SocketAddr};

use axum::{
    Router, extract::DefaultBodyLimit, routing::{get, post}
};
use copycat::{
    AppState,
    configuration::get_configuration,
    expiration::delete_expired,
    routes::{index_handler, paste_handler, raw_handler, upload_handler},
    tracing::init_tracing,
};
use mclog::{parser::Parser};
use tokio::signal;
use tracing::{debug, error};

#[tokio::main]
async fn main() {
    init_tracing();

    let configuration = get_configuration().unwrap_or_else(|err| {
        error!("Failed to read configuration: {}", err);
        std::process::exit(1);
    });

    #[cfg(feature = "magika")]
    let magika_session = magika::Session::new()
        .ok()
        .map(|s| std::sync::Arc::new(std::sync::Mutex::new(s)));

    let app_state = AppState {
        #[cfg(feature = "magika")]
        magika: magika_session,
    };

    let router = Router::new()
        .route("/", get(index_handler))
        .route("/{id}", get(paste_handler))
        .route("/parse", get(parse_handler))
        .route("/upload", post(upload_handler))
        .route("/raw/{id}", get(raw_handler))
        .with_state(app_state)
        .layer(DefaultBodyLimit::max(5_000_000));

    let application = Application {
        host: configuration.application.host,
        port: configuration.application.port,
    };

    let listener = tokio::net::TcpListener::bind(application.address())
        .await
        .unwrap_or_else(|err| {
            error!(
                "Failed to bind TcpListener on {}: {}",
                application.address(),
                err
            );
            std::process::exit(1)
        });

    debug!(
        "Listening on {}",
        listener.local_addr().unwrap_or_else(|err| {
            error!(
                "Failed to get local_addr of TcpListener '{:?}': {}",
                listener, err
            );
            std::process::exit(1);
        })
    );

    tokio::spawn(async move {
        if let Err(err) = delete_expired(&configuration.data).await {
            error!(
                "Failed to run background task for deleting expired files: {}",
                err
            );
            std::process::exit(1);
        }
    });

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|err| {
            error!("Failed to serve the Axum app: {}", err);
            std::process::exit(1);
        })
}

struct Application {
    host: String,
    port: u16,
}

impl Application {
    fn address(&self) -> SocketAddr {
        let host = self.host.parse::<IpAddr>().unwrap_or_else(|err| {
            error!("Failed to parse '{}' into IpAddr: {}", self.host, err);
            std::process::exit(1);
        });

        SocketAddr::from((host, self.port))
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.unwrap_or_else(|err| {
            error!("Failed to install Ctrl+C handler: {}", err);
            std::process::exit(1);
        });
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap_or_else(|err| {
                error!("Failed to install signal handler: {}", err);
                std::process::exit(1);
            })
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn parse_handler() -> String {
    let lines = [
        "INFO jedna",
        "WARN dva",
        "tri",
        "ERROR pet",
        "sest",
        "sedm",
        "WARN ahoj",
    ];
    let parser = Parser::new(&lines);
    let parsed_log = parser.parse();

    format!("{:?}", parsed_log)
}
