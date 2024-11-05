# 谷神星

> 是火星和木星轨道之间的主小行星带中最亮的天体。

用于定时发送请求检测服务是否存活的 Status 程序后端。

从环境变量 `CERES_STATUS_CONFIG` 读入配置文件。

配置文件数据结构如下：

```go
type Config struct {
	// 本服务运行的端口 
	Port  string        `json:"port"` 
	// 本服务运行的主机范围，一般为 0.0.0.0
	Host  string        `json:"host"` 
	// 监测服务配置组
	Entry []ConfigEntry `json:"entry"`
}

type ConfigEntry struct {
	// 要监测的服务名称，不允许重复
	ServiceName      string `json:"service_name"` 
	// 需要请求的端点
	Endpoint         string `json:"endpoint"` 
	// 请求的间隔（分钟）
	Interval         int    `json:"interval"` 
	// 请求超时时间（秒钟）
	Timeout          int    `json:"timeout"` 
	// 期待的在线时的状态码
	OnlineStatusCode int    `json:"online_status_code"`
}
```

样例：

```json
{
    "port": "63000",
    "host": "0.0.0.0",
    "entry": [
        {
            "service_name": "Ceres Status",
            "endpoint": "http://127.0.0.1:63000",
            "interval": 1,
            "timeout": 5,
            "online_status_code": 200
        }
    ]
}
```

> 请注意，写入环境变量时需要将 json 压缩为一行