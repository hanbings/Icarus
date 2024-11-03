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
    let node = Node {
        endpoint: config.endpoint.clone(),
    };
    let nodes = config
        .nodes
        .iter()
        .map(|node| Node {
            endpoint: node.clone(),
        })
        .collect();
    info!("Setting up server...");
    let node_state = Data::new(Mutex::new(NodeState {
        node,
        nodes,
        node_type: NodeType::Follower,
        leader: None,
        term: 0,
        index: 0,
        log: vec![],
        data: HashMap::new(),
        secret: config.secret.clone(),
    }));
    let node_clock = Data::new(Mutex::new(NodeClockState {
        clock: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        heartbeat: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        election: 0,
    }));
    let client = Data::new(Mutex::new(client::Client::new()));
   ```

   > 注意：需要自己处理配置文件的读取
   > 可以选择 figment 作为配置文件中间层
   > `figment = { version = "0.10", features = ["toml"] }`

3. 将 AppState 注入到 actix 上下文，并注册路由

   ```rust
   // 忽略了 actix use 引用，需自行补全
   
    info!("Application Running...");
    HttpServer::new(move || {
        App::new()
            .app_data(node_state.clone())
            .app_data(node_clock.clone())
            .app_data(client.clone())
            .service(iris_irides::raft::endpoint_append::append)
            .service(iris_irides::raft::endpoint_vote::vote)
            .service(iris_irides::raft::endpoint_check::check)
            .service(iris_irides::raft::endpoint_metadata::get_state)
            .service(iris_irides::raft::endpoint_metadata::get_data)
            .service(iris_irides::raft::endpoint_metadata::post_data)
    })
    .bind((config.ip, config.port))?
    .run()
    .await
   }
   ```

   > ~~endpoint 的形式与注册的 actix 的路由有关，例如本例子中注册到 `/cluster` 下，那么 endpoint 为 `http://127.0.0.1/cluster`~~
   >
   > ~~您也可以选用其他路由端点，但暂不支持为每一个接口都设置路由前缀~~
   >
   > ~~endpoint 需要携带协议名称 http 或 https~~
   >
   > 现在不允许修改端口了 uwu

4. 创建用于定时请求 `/raft/check` 接口的 `watchdog` 程序，为 `iris` 提供一个时钟

   ```rust
   info!("Initializing client...");
   tokio::spawn(client::async_clock(
       config.endpoint.clone(),
       config.secret.clone(),
   ));
   ```

5. 简单的安全机制

   ```rust
   use iris_irides::security::secret::secret_middleware;
   
   ...
   
   HttpServer::new(move || {
       // 使用 iris 内置的简单 Bearer 解析工具（actix middleware）
       // 需要添加 actix-web-httpauth = "0.8.2" 作为依赖
       let auth = HttpAuthentication::with_fn(secret_middleware);
       App::new()
           .warp(auth)
            
   ...
   ```

   使用 `secret_middleware`，只需要在对应的 `handler` 中像使用 `state` 一样添加 `auth: BearerAuth` 即可：

   ```rust
   #[post("/config/{key}")]
   async fn post_config_key(
       node_state: web::Data<Mutex<NodeState>>,
       auth: BearerAuth,
   ) -> Result<HttpResponse, Error> {
       let node_state = node_state.lock().await;
   
       if node_state.secret.is_some() && 
          auth.token().to_string() != node_state.secret.clone().unwrap() {
           return Ok(HttpResponse::Unauthorized().json(Message::unauthorized()));
       }
       
   ...
   ```

   
