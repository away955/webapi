use std::sync::Arc;

use axum::{
    extract::{ws, State},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

use crate::{auth, AppState};

pub(super) async fn handler(
    user: auth::Claims,
    State(state): State<Arc<AppState>>,
    upgrade: ws::WebSocketUpgrade,
) -> impl IntoResponse {
    upgrade
        .on_failed_upgrade(|err| tracing::warn!("websocket conn error:{}", err))
        .on_upgrade(move |sockect| websocket(user.userid, state, sockect))
}

async fn websocket(uid: i32, state: Arc<AppState>, socket: ws::WebSocket) {
    tracing::info!("当前连接用户：{}", uid);

    let (mut sender, mut receiver) = socket.split();

    let tx = state.chat.clone();
    let mut rx = tx.subscribe();

    // 发送消息
    tokio::spawn(async move {
        while let Ok(payload) = rx.recv().await {
            match payload {
                Payload::One(myself, who, msg) => {
                    if who.ne(&uid) {
                        continue;
                    }

                    let json = match serde_json::to_string(&Payload::One(who, myself, msg.clone()))
                    {
                        Ok(data) => data,
                        Err(err) => {
                            tracing::warn!("{myself} to {who}: 发送消息失败：{err}");
                            continue;
                        }
                    };
                    let res = sender.send(ws::Message::Text(json)).await;
                    match res {
                        Ok(_) => tracing::debug!("{} to {}: {:?}", myself, who, msg),
                        Err(err) => tracing::warn!("{myself} to {who}: 发送消息失败：{err}"),
                    }
                }
                Payload::Group(_, _, _) => todo!(),
                Payload::All(myself, msg) => {
                    if myself.eq(&uid) {
                        continue;
                    }

                    let json = match serde_json::to_string(&Payload::All(myself, msg.clone())) {
                        Ok(data) => data,
                        Err(err) => {
                            tracing::warn!("{myself} to all: 发送消息失败：{err}");
                            continue;
                        }
                    };
                    let res = sender.send(ws::Message::Text(json)).await;
                    match res {
                        Ok(_) => tracing::debug!("{} to all: {:?}", myself, msg),
                        Err(err) => tracing::warn!("{myself} to all: 发送消息失败：{err}"),
                    }
                }
                Payload::None => todo!(),
            }
        }
    });

    // 接收消息
    while let Some(res) = receiver.next().await {
        let msg = match res {
            Ok(data) => data,
            Err(err) => {
                tracing::warn!("websocket读取消息失败:{err}");
                break;
            }
        };

        let payload = match msg {
            ws::Message::Text(text) => serde_json::from_slice::<Payload>(text.as_bytes())
                .unwrap_or_else(|err| {
                    tracing::warn!("解析消息错误：{err}");
                    Payload::None
                }),
            ws::Message::Close(_) => break,
            ws::Message::Ping(_) => todo!(),
            ws::Message::Pong(_) => todo!(),
            ws::Message::Binary(_) => todo!(),
        };
        tracing::info!("解析数据：{:?}", payload);
        match payload {
            Payload::One(_, who, msg) => {
                let _ = tx.send(Payload::One(uid, who, msg));
            }
            Payload::Group(_, groupid, msg) => {
                let _ = tx.send(Payload::Group(uid, groupid, msg));
            }
            Payload::All(_, msg) => {
                let _ = tx.send(Payload::All(uid, msg));
            }
            Payload::None => todo!(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) enum Payload {
    One(i32, i32, Message),
    Group(i32, i32, Message),
    All(i32, Message),
    None,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub(crate) struct Message {
    pub(crate) method: String,
    pub(crate) data: Option<String>,
}
