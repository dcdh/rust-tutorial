use std::env;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use crate::add::add;
use crate::hello_world::{FrenchSayHelloWorld, HelloWorldService};
use crate::remote_hello_world::HelloWorldClient;

mod add;
mod hello_world;
mod remote_hello_world;

async fn hello_world() -> String {
    let hello_world_url = retrieve_hello_world_url();
    let client = HelloWorldClient::new(hello_world_url);
    let response = client.fetch_hello().await.unwrap();
    response
}

fn retrieve_hello_world_url() -> String {
    env::var("HELLO_WORLD_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

fn create_app() -> Router {
    let asset_path = retrieve_assets_path();
    println!("asset_path {:?}", asset_path);
    Router::new().route("/hello", get(hello_world))
        .route_service("/", ServeDir::new(asset_path.join("index.html")))
}

fn retrieve_assets_path() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to retrieve current directory");
    let current_exe = env::current_exe().expect("Failed to retrieve executable directory");

    let assets_path = current_dir.join("assets");

    if assets_path.is_dir() {
        assets_path
    } else {
        current_exe.parent().unwrap_or_else(|| Path::new(".")).join("assets")
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let i = add(1, 2);
    println!("{}", i);

    let french_say_hello_world = FrenchSayHelloWorld;
    let hello_world_service = HelloWorldService::new(&french_say_hello_world);
    println!("{}", hello_world_service.say_hello_world());

    // 🏗 Définition des routes
    let app = create_app();

    // 📡 Lancement du serveur sur un port dynamique
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Serveur démarré sur http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod integration_tests {
    use axum_test::TestServer;
    use testcontainers::core::{ContainerPort, WaitFor};
    use testcontainers::GenericImage;
    use testcontainers::runners::AsyncRunner;
    use crate::create_app;

    #[tokio::test]
    async fn test_hello_world() {
        // Given
        let container = GenericImage::new("strm/helloworld-http", "latest")
            .with_exposed_port(ContainerPort::Tcp(80))
            .with_wait_for(WaitFor::millis(100))
            .start()
            .await
            .expect("Failed to start container");
        let port = container.get_host_port_ipv4(80).await.unwrap();
        unsafe {
            std::env::set_var("HELLO_WORLD_URL", format!("http://localhost:{}", port));
        }

        let server = TestServer::new(create_app()).unwrap();

        // When
        let response = server.get("/hello").await;

        // Then
        response.assert_status_ok();
        response.assert_text_contains("HTTP Hello World");
    }
}
