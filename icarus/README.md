# Icarus

配置文件样例。可以保存为 `icarus.toml` 文件或 `ICARUS_CONFIG` 环境变量。

```toml
ip = "0.0.0.0"
port = 10086
tokens = [
    { name = "Test",token = "a60e9151-62a9-12d5-f37f-83e2ce88b334", allowed = [] }
]
config_centers = [
    { name = "Config Test", endpoints = [ "http://127.0.0.1:10000" ], size = 3 }
]
service_explores = []
message_queues = []
```

```json
{
    "ip": "0.0.0.0",
    "port": 12345,
    "tokens": [
        {
            "name": "test",
            "token": "a60e9151-62a9-12d5-f37f-83e2ce88b334",
            "allowed": []
        }
    ],
    "config_centers": [
        {
            "name": "test",
            "secret": "2a5335de-7db7-0163-46fd-4b6eb79ce143",
            "endpoints": [
                "http://127.0.0.1:10000"
            ],
            "size": 3
        }
    ],
    "service_explores": [],
    "message_queues": []
}
```

