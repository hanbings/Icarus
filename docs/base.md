# 基本环境搭建

> 安装 Docker、Nginx 和 Certbot

基于 Debian 11 / Debian 12 与 deb 包。

**强烈建议您不要使用 `root` 账号运行指令，而是创建一个拥有 `sudo` 权限的低权限账户**

## 更新包

```bash
sudo apt update
```

## 安装 Docker

[官方文档](https://docs.docker.com/engine/install/debian/)

1. 安装官方仓库 GPG key

   ```bash
   sudo apt-get update
   sudo apt-get install ca-certificates curl
   sudo install -m 0755 -d /etc/apt/keyrings
   sudo curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc
   sudo chmod a+r /etc/apt/keyrings/docker.asc
   ```

2. 添加仓库

   ```bash
   echo \
     "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian \
     $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
     sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
   sudo apt-get update
   ```

3. 安装 Docker、Docker Compose

   ```bash
   sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
   ```

## 安装 Nginx

[官方文档](https://nginx.org/en/linux_packages.html#Debian)

1. 安装官方仓库 GPG key

   ```bash
   curl https://nginx.org/keys/nginx_signing.key | gpg --dearmor \
       | sudo tee /usr/share/keyrings/nginx-archive-keyring.gpg >/dev/null
   ```

2. 添加仓库

   ```bash
   echo "deb [signed-by=/usr/share/keyrings/nginx-archive-keyring.gpg] \
   http://nginx.org/packages/debian `lsb_release -cs` nginx" \
       | sudo tee /etc/apt/sources.list.d/nginx.list
   
   # 设置官方源优先级    
   echo -e "Package: *\nPin: origin nginx.org\nPin: release o=nginx\nPin-Priority: 900\n" \
       | sudo tee /etc/apt/preferences.d/99nginx
       
   sudo apt update
   ```

3. 安装 Nginx

   ```bash
   sudo apt install nginx
   ```

## Certbot

[官方文档](https://certbot.eff.org/instructions?ws=nginx&os=snap)

1. 安装 snap

   https://snapcraft.io/docs/installing-snap-on-debian

   ```bash
   sudo apt update
   sudo apt install snapd
   ```

2. 使用 snap 安装 certbot

   ```bash
   sudo snap install --classic certbot
   sudo ln -s /snap/bin/certbot /usr/bin/certbot
   ```

3. 签发证书方式

   ```bash
   certbot --nginx
   ```

   
