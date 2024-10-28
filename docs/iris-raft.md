# Raft 共识算法的 Iris 实现

## Raft 共识算法

这一部分是本人对 Raft 的理解与简单总结，如果您此前并不了解 Raft，可以选择下方的原文和译文链接详细阅读。

感谢这些前辈们设计出如此巧妙的算法和译者详细、精心编排的译文。

[Raft 论文原文](https://pdos.csail.mit.edu/6.824/papers/raft-extended.pdf)

[Raft 论文中文翻译](https://arthurchiao.art/blog/raft-paper-zh/)

### 简要描述

Raft 共识算法将分布式系统的复杂问题拆分为了独立的可解决、可解释和可理解的模块：

- Leader Electrion（选举）
- Log Replication（日志复制）
- Safety（安全性）

Raft 共识算法主要机制：

1. Election Safety（选举安全）：在任意 Term（任期）内，**最多只会有一个** Leader 被选出来
2. Leader Append-Only（只追加）：Leader 从不覆盖或删除它的日志中的 Entry；只会追加（Append）
3. Log Matching（日志匹配）：如果两个日志包含了 Index 和 Term **完全相同**的 Entry， 那从这个 Index 往前的那些 Entry 也都是完全相同的
4. Leader Completeness（领导者完整性）：如果一个 Entry 在某个 Term 被提交，那它将出现在**所有 Term 更大的 Leader** 的 Log 中
5. State Machine Safety（状态机安全）：如果一个节点在特定 Index 应用了一个 Entry 到它的状态机，那其他节点不会在相同 Index 应用另一个不同的 Entry

### 状态机

### 任期

### RPC 请求

#### 状态

- Leader

- Follower
- Candidate

#### AppendEntries

用于：

1. 带有操作的 AppendEntries
2. 用于维护 Leader 身份的 heartbeat（心跳机制）

#### RequestVote

在集群的初始状态或当前 Term 的 Leader 离线后，集群中的一个节点将从 Follower 转为 Candidate 状态，并由一个 Candidate 向其他的未及时检测出 Leader 离线的 Follower 发送 RequestVote。如果集群中同时有多个 Follower 节点转为了 Candidate，将会导致当前 term 作废，并使用随机化超时机制避免脑裂问题出现。

## Iris 实现

Iris 实现（下称实现）使用 Rust 语言编写，并使用 Rust 高性能异步 Web 框架 Actix 作为数据传输通道。

### 整体流程

#### 组成集群

当节点启动时，计时器为 0

#### 处理客户端的数据请求

#### Leader 离线

#### Follower 离线

### 机制实现

#### Web API RPC

#### 心跳

#### 随机化选举超时

