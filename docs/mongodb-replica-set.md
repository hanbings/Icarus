# MongoDB Replica Set 搭建

本文档采用三台服务器在 Docker 环境下组成副本集。

## 准备 Docker Compose 文件

## 准备配置文件

## 组成集群

```bash
# 进入 docker 容器内
$ docker exec -it {容器名字} mongosh
# 组成集群
mongosh $ rs.initiate({_id: "{集群名称}",members: [{ _id: 0, host: "{IP-0}:{端口}" },{ _id: 1, host: "{IP-1}:{端口}" },{ _id: 2, host: "{IP-2}:{端口}" }]})
```

## 开启鉴权

```bash
# 创建 OpenSSL 私钥文件
$ openssl rand -base64 753 > mongo.key
# 必须设置权限和所有者
$ chown 999 mongo.key
$ chmod 400 mongo.key
```

**拼接连接字符串**

```bash
mongodb://用户名:密码@{IP-0}:{端口},{IP-1}:{端口},{IP-2}:{端口}/数据库名称?replicaSet={集群名称}
```
