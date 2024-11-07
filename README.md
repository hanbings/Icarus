![Icarus](https://ice.frostsky.com/2024/10/25/71cadee62ff279e70272ad13431e9cce.png)

<h1 align="center">🌟 Icarus</h1>

**简直天坑**

## 文档

- [架构](/docs/architecture.md)
- [基本运行环境（Docker Nginx 和 Certbot）](./docs/base.md)
- [MongoDB Replica Set 搭建](./docs/mongodb-replica-set.md)
- [Redis Cluster 搭建](./docs/redis-cluster.md)
- [CI / CD 构建系统与自动化部署系统](./docs/ci-cd.md)
- [Drone CI 构建工具搭建](./docs/drone-ci.md)
- [Harbor 仓库搭建](./docs/harbor.md)
- [Raft 共识算法的 Iris 实现](./docs/iris-raft.md)
- [随机构建 Bento Box 网格](./docs/build-a-random-bento-box-grip.md)

## 项目

项目包含满足赛题基础部分所描述的 Github Rank 应用（主程序）以及一部分用于拉取和分析 Github 数据所需要的基础设施。

[详细架构描述](./docs/architecture.md)

[演示视频](https://icaruspw.dev/demo.mp4)

### 主程序
- [Starplex](./starplex) - 星从 用户侧应用的后端 - Java
- [Starplex-web](./starplex-web) - 星从 用户侧应用的前端 - TypeScript（React）
- [Ceres](./ceres) - 谷神星 公开服务的状态监测网页后端 - Go
- [Ceres-web](./ceres-web) - 谷神星 公开服务的状态监测网页前端 - TypeScript（Vue）

### 基础设施
- [Icarus](./icarus) - 伊卡洛斯 分布式系统控制面板后端 - Rust
- [Icarus-web](./icarus-web) - 伊卡洛斯 分布式系统控制面板前端 - TypeScript（React）
- [Iris](./iris) - 虹神星 分布式工具库 - Rust
- [Makemake](./makemake) - 鸟神星 分布式消息队列 - Rust
- [Flora](./flora) - 花神星 分布式发现中心 - Rust
- [Aurora](./aurora) - 彩神星 分布式配置中心 - Rust
- Luminous - 流光 算子 - Rust

### 爬取器与分析器

- [Salacia](./salacia) - 潫神星 Github 原始数据爬取器 Java
- [Pallas](./pallas) - 智神星 Github 数据分析器 Python

## 贡献

[贡献指南](./CONTRIBUTING.md)

本项目的代码均使用 [MIT License](https://github.com/hanbings/icarus/blob/main/LICENSE) 进行开源