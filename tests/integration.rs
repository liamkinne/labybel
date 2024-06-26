use labybel::Client;

#[tokio::test]
async fn connect_with_default_port() {
    let client = Client::new("http://127.0.0.1", None);
    assert!(client.connected().await.unwrap());
}

#[tokio::test]
async fn get_printers() {
    let client = Client::new("http://127.0.0.1", None);
    let printers = client.printers().await.unwrap();

    assert!(printers.len() > 0);
}
