pub mod error;

async fn websocket_handler(&self, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| self.websocket(socket))
}

async fn websocket(&mut self, stream: WebSocket) {
    let (mut sender, mut receiver) = stream.split();
    let mut username = String::new();
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            check_username(&state, &mut username, &name);
            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text("Username already taken".to_string()))
                    .await;
            }
            return;
        }
    }
    let mut rx = state.tx.subscribe();
    let msg = format!("{username} joined.");
    println!("{}", msg);
    let _ = state.tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    let tx = state.tx.clone();
    let name = username.clone();
    let mut receive_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            let _ = tx.send(format!("{name}: {msg}"));
        }
    });

    tokio::select! {
        _ = (&mut send_task) => receive_task.abort(),
        _ = (&mut receive_task) => send_task.abort(),
    }

    let msg = format!("{username} left.");
    println!("{}", msg);
    let _ = state.tx.send(msg);
    state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, username: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();
    if !user_set.contains(name) {
        user_set.insert(name.to_string());
        username.push_str(name);
    }
}
