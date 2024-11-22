use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::{TokioIo, TokioTimer};
use tokio::net::{TcpListener, TcpStream};
use tracing::Instrument;

use crate::router::Router;
use std::{net::SocketAddr, sync::Arc};

pub struct WebServer {
    addr: SocketAddr,
    router: Arc<Router>,
}

impl WebServer {
    pub fn new(addr: SocketAddr, router: Router) -> Self {
        Self {
            addr,
            router: Arc::new(router),
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn run_server(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.addr).await?;
        tracing::info!("Listening on: {}", self.addr);
        let span = tracing::Span::current();
        loop {
            let (stream, addr) = listener.accept().await?;
            let router = self.router.clone();
            tokio::task::spawn(
                Self::handle_connection(addr, TokioIo::new(stream), router)
                    .instrument(span.clone()),
            );
        }
    }

    #[tracing::instrument(skip_all, fields(client=%client_addr))]
    async fn handle_connection(
        client_addr: SocketAddr,
        io: TokioIo<TcpStream>,
        router: Arc<Router>,
    ) {
        tracing::info!("Accepted connection");
        let server = http1::Builder::new()
            .timer(TokioTimer::new())
            .serve_connection(io, service_fn(|req| async { router.route(req).await }))
            .await;
        if let Err(e) = server {
            tracing::error!("Error serving connection: {}", e);
        } else {
            tracing::info!("Finished serving connection");
        }
    }
}
