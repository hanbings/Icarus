![Icarus](https://ice.frostsky.com/2024/10/25/71cadee62ff279e70272ad13431e9cce.png)

<h1 align="center">🌟 Icarus</h1>

**简直天坑**

## 项目

项目包含满足赛题基础部分所描述的 Github Rank 应用（主程序）以及一部分用于拉取和分析 Github 数据所需要的基础设施。

[详细架构描述](./docs/architecture.md)

### 主程序
- [starplex](./starplex) - 星从 用户侧应用的后端 Java
- [starplex-web](./starplex-web) - 星从 用户侧应用的前端 React
- ceres - 谷神星 公开服务的状态监测网页 Vue

### 基础设施
- [Icarus](./icarus) - 伊卡洛斯 分布式系统控制面板 - React（Next.js / TypeScript）
- [Iris](./iris) - 虹神星 分布式工具库 - Rust
- [Makemake](./makemake) - 鸟神星 分布式消息队列 - Rust
- [Flora](./flora) - 花神星 分布式发现中心 - Rust
- [Aurora](./aurora) - 彩神星 分布式配置中心 - Rust

### 爬取器与分析器

- [salacia](./salacia) - 潫神星 Github 原始数据爬取器 Java
- pallas - 智神星 Github 数据分析器 Python

## 贡献

[贡献指南](./CONTRIBUTING.md)