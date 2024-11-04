# Aurora

> **彩神星**（94 Aurora）是最大的[主带](https://zh.wikipedia.org/wiki/主小行星帶)[小行星](https://zh.wikipedia.org/wiki/小行星)之一

配置中心，用于向客户端下发配置文件。

配置中心自身附带组成集群所需要的配置，可以以 `toml` 文件（命名为 `aurora.toml`）的形式保存在与主程序同一目录下或使用环境变量（`AURORA_CONFIG`）传递 `json`

```toml
ip = "127.0.0.1"
port = 10000
endpoint = "http://127.0.0.1:10000"
nodes = [
    "http://127.0.0.1:10000",
    "http://127.0.0.1:10001",
    "http://127.0.0.1:10002"
]
secret = "2a5335de-7db7-0163-46fd-4b6eb79ce143"

```

```json
{
    "nodes": [
        "http://127.0.0.1:10000",
        "http://127.0.0.1:10001",
        "http://127.0.0.1:10002"
    ]
}
```

> 请注意，写入环境变量时需要将 `json` 压缩为一行
>
> 可以一部分配置使用 `toml` 文件传递，一部分配置使用 `json`，在程序启动时会根据 `toml` - `json` 的顺序加载它们。