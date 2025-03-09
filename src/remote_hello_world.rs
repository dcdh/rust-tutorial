use reqwest::Client;

pub struct HelloWorldClient {
    base_url: String,
    http_client: Client,
}

impl HelloWorldClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_hello(&self) -> reqwest::Result<String> {
        let url = format!("{}/", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        response.text().await
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::core::{ContainerPort, WaitFor};
    use testcontainers::runners::AsyncRunner;
    use testcontainers::GenericImage;
    use tokio;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::method;

    #[tokio::test]
    async fn test_fetch_hello_with_testcontainers() {
        // Given
        let container = GenericImage::new("strm/helloworld-http", "latest")
            .with_exposed_port(ContainerPort::Tcp(80))
            .with_wait_for(WaitFor::millis(100))
            .start()
            .await
            .expect("Failed to start container");
        let port = container.get_host_port_ipv4(80).await.unwrap();

        let client = HelloWorldClient::new(format!("http://localhost:{}", port));

        // When
        let response = client.fetch_hello().await.unwrap();

        // Then
        assert!(response.contains("HTTP Hello World"));
    }

    #[tokio::test]
    async fn test_fetch_hello_with_wiremock() {
        // Given
        let mock_server = MockServer::start().await;
        let mock = Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
            .set_body_string("Hello World"))
            .expect(1);
        mock_server.register(mock).await;

        let client = HelloWorldClient::new(mock_server.uri());

        // When
        let response = client.fetch_hello().await.unwrap();

        // Then
        assert_eq!(response, "Hello World");
        mock_server.verify().await;
    }
}
