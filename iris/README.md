<h1 align="center">🪻✨ Iris</h1>

## Iris 虹神星（鸢尾花）

Iris 是一个针对分布式系统的工具集合

- [x] Raft 算法实现
- [ ] Gossip 算法实现

## 使用（呜呜没空写详细文档了）

在 Rust 编写的分布式应用中使用 Iris ：

（以 aurora 配置中心为例子）

1. 在 Rust 应用中添加依赖：

   ```toml
   [package]
   name = "aurora"
   version = "0.1.0"
   edition = "2021"
   
   [dependencies]
   iris = { path = "../iris" }
   log = "0.4"
   env_logger = "0.11"
   uuid = { version = "1.11.0", features = ["v4", "serde"] }
   
   actix-web = "4"
   tokio = { version = "1.0", features = ["full"] }
   serde = { version = "1", features = ["derive"] }
   serde_json = "1"
   reqwest = "0.12.8"
   ```
   
2. 创建 State 在 Actix 中共享上下文：

   ```rust
   use iris_irides::raft::client::IrisRaftClient;
   use iris_irides::raft::config::IrisRaftConfig;
   use iris_irides::raft::state::{IrisRaftClock, IrisRaftNodeState};
   
   let clock = Data::new(Mutex::new(IrisRaftClock::new()));
   let client = Data::new(Mutex::new(IrisRaftClient::new(config.endpoint.clone())));
   let node_state = Data::new(Mutex::new(IrisRaftNodeState::new(
       IrisRaftConfig::no_log_compaction(
           config.id,
           config.secret,
           config.endpoint,
           // heartbeat timeout
           200,
           // election timeout (range random)
           (300, 800),
       ),
   )));
   ```

   > 注意：需要自己处理配置文件的读取
   > 可以选择 figment 作为配置文件中间层
   > `figment = { version = "0.10", features = ["toml"] }`

3. 将 AppState 注入到 actix 上下文，并注册路由

   ```rust
   // 忽略了 actix use 引言，需自行补全
   
   HttpServer::new(move || {
       App::new()
           .app_data(Data::clone(&clock))
           .app_data(Data::clone(&client))
           .app_data(Data::clone(&node_state))
           // metadata
           .route("/cluster/node", web::get().to(iris_irides::raft::endpoint_metadata::get_node))
           .route("/cluster/nodes", web::get().to(iris_irides::raft::endpoint_metadata::get_nodes))
           .route("/cluster/status",web::get().to(iris_irides::raft::endpoint_metadata::get_status))
           // check
           .route("/cluster/check",web::post().to(iris_irides::raft::endpoint_check::post_check))
           // action
           .route("/cluster/append",web::post().to(iris_irides::raft::endpoint_action::post_append))
           .route("/cluster/commit",web::post().to(iris_irides::raft::endpoint_action::post_commit))
           .route("/cluster/vote",web::post().to(iris_irides::raft::endpoint_action::post_vote))
   })
   .bind((config.ip, config.port))?
   .run()
   .await
   ```
   
   > endpoint 的形式与注册的 actix 的路由有关，例如本例子中注册到 `/cluster` 下，那么 endpoint 为 `http://127.0.0.1/cluster`
   >
   > 您也可以选用其他路由端点，但暂不支持为每一个接口都设置路由前缀
   >
   > endpoint 需要携带协议名称 http 或 https
