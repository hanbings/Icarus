# Harbor Docker Registry 搭建

1. [下载 harbor 离线安装包](https://github.com/goharbor/harbor/releases)

2. 到合适的目录解压：

   ```bash
   tar -zxvf 下载的文件名 目录
   ```

3. 编辑配置文件 `harbor.yml`，其中比较重要的部分：

   ```yaml
   # 主机名
   hostname: registry.icaruspw.dev
   
   # 端口
   http:
     port: 50000
   
   # 如果需要在外部使用 nginx 反向代理至域名访问，需要设置这个基本链接
   external_url: https://registry.icaruspw.dev
   
   # 管理员初始密码
   harbor_admin_password: ************
   
   # 这个是外部的数据目录
   data_volume: /app/harbor/data
   ```

4. 安装

   ```bash
   ./install.sh
   ```

5. 管理 harbor

   进行 `./install.sh` 之后，后续管理都是以 `docker compose` 的方式

   ```bash
   # 启动
   docker compose up
   
   # 更新并启动
   docker compose up -d
   
   # 停止
   docker compose stop
   ```

   
