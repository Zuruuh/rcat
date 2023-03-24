use tokio;

pub async fn client() -> Result<(), String> {
    let tcp_client = tokio::net::TcpStream::connect("localhost:2323").await.map_err(|_| "failed to connect")?;
    let (mut reader, mut writer) = tcp_client.into_split();

    let client_reader = tokio::spawn(async move {
        tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await
    });

    let client_writer = tokio::spawn(async move {
        tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await
    });

    tokio::select! {
        _ = client_reader => {}
        _ = client_writer => {}
    };

    Ok(())
}

pub async fn server() -> Result<(), String> {
    let tcp_listener = tokio::net::TcpListener::bind("localhost:2323").await.map_err(|_| "failed to bind")?;
    let (tcp_handle, _) = tcp_listener.accept().await.map_err(|_| "failed to connect")?;

    let (mut reader, mut writer) = tcp_handle.into_split();

    let client_reader = tokio::spawn(async move {
        tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await
    });

    let client_writer = tokio::spawn(async move {
        tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await
    });

    tokio::select! {
        _ = client_reader => {}
        _ = client_writer => {}
    };

    Ok(())
}

