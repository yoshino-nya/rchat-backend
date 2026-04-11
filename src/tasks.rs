use std::sync::Arc;

use crate::state::AppState;

pub fn spawn_message_dispatcher(state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut rx = state.tx.subscribe();

        while let Ok(msg) = rx.recv().await {
            let users: Result<Vec<i32>, _> = sqlx::query_scalar(
                r#"
                SELECT user_id
                FROM chat_message_members
                WHERE session_id = $1
            "#,
            )
            .bind(&msg.session_id)
            .fetch_all(&state.pool)
            .await;
            if let Ok(users) = users {
                let clients = state.clients.read().await;
                for u in users {
                    if let Some(tx) = clients.get(&u) {
                        let _ = tx.send(msg.clone()).await;
                    }
                }
            }
        }
    });
}
