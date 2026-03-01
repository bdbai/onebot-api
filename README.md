# Onebot API

库如其名，这是一个Onebot V11协议的实现  
目前已完成对Onebot V11协议所有API的实现

# Usage

## Client用法

```rust
use std::time::Duration;
use onebot_api::api::APISender;
use onebot_api::communication::utils::{Client, Event};
use onebot_api::communication::ws::WsService;
use onebot_api::event::EventReceiver;
use onebot_api::text;

#[tokio::main]
async fn main() {
    let ws_service = WsService::new("wss://example.com", Some("example_token".to_string())).unwrap();
    let client = Client::new(ws_service, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
    
    let msg_id = client.send_private_msg(123456, text!("this is a {}", "message"), None).await.unwrap();
    client.send_like(123456, Some(10)).await.unwrap();
    
    let mut event_receiver = client.get_receiver();
    while let Ok(event) = event_receiver.recv().await && let Event::Event(event) = &*event {
        println!("{:#?}", event)
    }
}
```

## 正向WebSocket

```rust
use std::time::Duration;
use onebot_api::communication::utils::Client;
use onebot_api::communication::ws::WsService;

#[tokio::main]
async fn main() {
    let ws_service = WsService::new("wss://example.com", Some("example_token".to_string())).unwrap();
    let client = Client::new(ws_service, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```

## 反向WebSocket

```rust
use onebot_api::communication::utils::Client;
use onebot_api::communication::ws_reverse::WsReverseService;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let ws_reverse_service = WsReverseService::new("0.0.0.0:8080", Some("example_token".to_string()));
    let client = Client::new(ws_reverse_service, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```

## Http

```rust
use onebot_api::communication::utils::Client;
use std::time::Duration;
use onebot_api::communication::http::HttpService;

#[tokio::main]
async fn main() {
    let http_service = HttpService::new("https://example.com", Some("example_token".to_string())).unwrap();
    let client = Client::new(http_service, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```

## Http Post

```rust
use onebot_api::communication::utils::Client;
use std::time::Duration;
use onebot_api::communication::http_post::HttpPostService;

#[tokio::main]
async fn main() {
    let http_post_service = HttpPostService::new("0.0.0.0:8080", None, Some("example_secret".to_string())).unwrap();
    let client = Client::new(http_post_service, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```

## SSE

```rust
use onebot_api::communication::utils::Client;
use std::time::Duration;
use onebot_api::communication::sse::SseService;

#[tokio::main]
async fn main() {
    let sse_service = SseService::new("https://example.com/_events", Some("example_token".to_string())).unwrap();
    let client = Client::new(sse_service, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```

## 组合器
同时，我们设计了组合器来将不同的底层连接放在同一个Client上  
例如，你可以创建一个SseService和一个HttpService，同时通过组合器将它们放在同一个Client上  
其行为与直接用WsService并无差别

### `SplitCombiner`
将事件接收与API发送分为两个不同服务实现  
服务分为 `send_side` 与 `read_side`  
其中，`send_side` 负责API发送服务，`read_side` 负责事件接收服务  
`send_side` 的事件通道由一个 processor task 负责  
processor 将 `send_side` 的API响应事件并入原事件通道，其余事件丢弃
```rust
use onebot_api::communication::utils::Client;
use std::time::Duration;
use onebot_api::communication::combiner::SplitCombiner;
use onebot_api::communication::http::HttpService;
use onebot_api::communication::sse::SseService;

#[tokio::main]
async fn main() {
    let sse_service = SseService::new("https://example.com/_events", Some("example_token".to_string())).unwrap();
    let http_service = HttpService::new("https://example.com", Some("example_token".to_string())).unwrap();
    let combiner = SplitCombiner::new(http_service, sse_service);
    let client = Client::new(combiner, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```
### `BothEventCombiner`
详见 `SplitCombiner`  
与 `SplitCombiner` 的区别在于  
`BothEventCombiner` 会将 `send_side` 的所有事件均并入原事件通道  
因此，`BothEventCombiner` 不存在 processor task
```rust
use onebot_api::communication::combiner::BothEventCombiner;
use onebot_api::communication::http::HttpService;
use onebot_api::communication::utils::Client;
use onebot_api::communication::ws::WsService;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let ws_service = WsService::new("https://example.com", Some("example_token".to_string())).unwrap();
    let http_service = HttpService::new("https://example.com", Some("example_token".to_string())).unwrap();
    let combiner = BothEventCombiner::new(http_service, ws_service);
    let client = Client::new(combiner, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```
## TIPS
对于组合器，组合器与组合器之间也是可以被组合器所连接的  
因此，对于一个bot消息集群，可以通过多个 `BothEventCombiner` 来实现同一个client接收所有消息
```rust
use onebot_api::communication::sse::SseService;
use onebot_api::communication::utils::Client;
use std::time::Duration;
use onebot_api::communication::combiner::BothEventCombiner;

#[tokio::main]
async fn main() {
    let bot_1 = SseService::new("http://127.0.0.1:5000", None).unwrap();
    let bot_2 = SseService::new("http://127.0.0.1:6000", None).unwrap();
    let bot_3 = SseService::new("http://127.0.0.1:7000", None).unwrap();
    let bot_4 = SseService::new("http://127.0.0.1:8000", None).unwrap();
    
    let combiner_1 = BothEventCombiner::new(bot_1, bot_2);
    let combiner_2 = BothEventCombiner::new(bot_3, bot_4);
    let combiner = BothEventCombiner::new(combiner_1, combiner_2);
    
    let client = Client::new(combiner, Some(Duration::from_secs(5)), None, None);
    client.start_service().await.unwrap();
}
```

