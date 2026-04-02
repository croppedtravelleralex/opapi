好的，我为你设计了一套超级详细的执行手册，每一步都力求清晰、可验证，并且涵盖了几乎所有可能的坑点。

---

### 🎯 目标

- **硬件**：一台公网 Ubuntu 服务器 (本文以 `Ubuntu 24.04` 为例)
- **软件**：Docker, Docker Compose, Nginx (用于反向代理)
- **账号**：2个以上的 ChatGPT Business 账号 (闲鱼或淘宝购买)，以及一个已解析到服务器的域名 (如 `api.yourdomain.com`)
- **最终产物**：一个 OpenAI 格式的 API Key (形如 `sk-xxx`)，OpenClaw 可直接使用。

---

### 🧱 阶段一：部署前准备 (环境搭建)

这个阶段的目标是把服务器环境准备好，包括安装 Docker、配置防火墙、安装 Nginx 代理。

#### 1.1 更新系统并安装基础工具

连接到你的 Ubuntu 服务器，执行以下命令：

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install curl git ufw wget -y
```

#### 1.2 安装 Docker 和 Docker Compose (使用官方脚本)

这是最稳定的安装方式，无需手动添加源。

```bash
# 下载并执行 Docker 官方安装脚本
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# 将当前用户加入 docker 组，避免每次输入 sudo (需要重新登录生效)
sudo usermod -aG docker $USER

# 验证安装 (如果提示权限错误，请重新登录 SSH 后再试)
docker --version
docker compose version
```

- **验证**：如果两个命令都正常输出版本号，说明 Docker 环境就绪。

#### 1.3 配置 UFW 防火墙 (仅开放必要端口)

```bash
# 允许 SSH 连接 (非常重要！防止把自己锁在外面)
sudo ufw allow 22/tcp

# 允许 HTTP 和 HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# 启用防火墙
sudo ufw enable

# 查看状态确认
sudo ufw status
```

- **验证**：输出应显示 `22,80,443/tcp` 为 `ALLOW`。

#### 1.4 安装 1Panel (强烈推荐)

1Panel 是一个可视化的 Linux 管理面板，可以非常方便地管理 Docker 容器、配置 Nginx 反向代理和申请 SSL 证书。

```bash
curl -sSL https://resource.fit2cloud.com/1panel/package/quick_start.sh -o quick_start.sh && sudo bash quick_start.sh
```

- 安装过程中，一路回车使用默认配置即可。
- **重要**：安装完成后，**务必记录**终端输出的：
    1.  **安全入口** (外网访问地址，例如 `http://你的IP:24045/xxxxx`)
    2.  **用户名**
    3.  **密码**
- **后续步骤**：在浏览器访问安全入口，登录 1Panel 后台。

---

### 🐳 阶段二：部署 Sub2API (核心中转站)

这个阶段将使用 Docker Compose 一键部署 Sub2API 及其依赖的数据库和缓存。

#### 2.1 创建项目目录并下载配置文件

回到服务器终端 (或使用 1Panel 自带的终端)，执行：

```bash
# 创建项目文件夹
mkdir -p ~/sub2api && cd ~/sub2api

# 下载 docker-compose 配置文件 (推荐 local 版，方便备份)
wget https://raw.githubusercontent.com/Wei-Shaw/sub2api/main/deploy/docker-compose.local.yml
```

#### 2.2 生成强密码和配置文件

Sub2API 需要一些密钥才能运行。我们生成一组随机密码并写入 `.env` 文件。

```bash
# 创建一个包含随机密码的 .env 文件
cat > .env << EOF
POSTGRES_USER=sub2api
POSTGRES_PASSWORD=$(openssl rand -base64 24)
POSTGRES_DB=sub2api
REDIS_PASSWORD=$(openssl rand -base64 24)
JWT_SECRET=$(openssl rand -base64 32)
ADMIN_PASSWORD=$(openssl rand -base64 16)
APP_PORT=8080
EOF

# 查看生成的密码 (建议复制保存到本地记事本，特别是 ADMIN_PASSWORD)
cat .env
```

- **注意**：`ADMIN_PASSWORD` 是后续登录管理后台的密码，需要记下来。

#### 2.3 启动 Sub2API 服务

```bash
# 使用 docker-compose 启动 (-d 表示后台运行)
docker compose -f docker-compose.local.yml up -d

# 查看容器运行状态
docker compose -f docker-compose.local.yml ps
```

- **验证**：你应该看到三个容器 (`sub2api`, `postgres`, `redis`) 的状态都是 `Up`。

---

### 🌐 阶段三：配置域名与 HTTPS (公网访问)

直接通过 IP 访问 Sub2API 可能会导致前端 JS 文件加载失败，因此我们需要绑定域名并开启 HTTPS。

#### 3.1 域名解析

登录你的域名服务商 (阿里云、腾讯云、Cloudflare 等)，添加一条 **A 记录**：
- **主机记录**：`api` (如果你想用 `api.yourdomain.com` 访问)
- **记录值**：你的服务器公网 IP
- **等待生效**：通常几分钟内生效。

#### 3.2 使用 1Panel 配置反向代理

1.  登录 1Panel 后台。
2.  点击左侧 **应用商店**，安装 **OpenResty** (这是 Nginx 的增强版)。
3.  安装完成后，点击左侧 **网站** -> **创建网站** -> **反向代理**。
4.  **主域名**：填入 `api.yourdomain.com` (你的完整域名)。
5.  **代理地址**：填入 `http://127.0.0.1:8080` (因为 Sub2API 在宿主机上监听了 8080 端口)。
6.  点击 **确认**。
7.  **自动 HTTPS**：在网站列表中找到刚创建的站点，点击 **HTTPS** -> **启用** -> **Let‘s Encrypt** -> 勾选所有选项 -> **保存**。

- **验证**：在浏览器访问 `https://api.yourdomain.com`。如果能看到 Sub2API 的登录页面，说明代理和 HTTPS 配置成功。

---

### 🔧 阶段四：配置 Sub2API (添加 Business 账号)

这是最关键的一步，将你的多个 Business 账号通过 OAuth 授权接入中转站。

#### 4.1 首次登录与充值

1.  打开浏览器，访问 `https://api.yourdomain.com`。
2.  登录账号：`admin@sub2api.local`，密码为你在 `2.2` 步骤中生成的 `ADMIN_PASSWORD`。
3.  **【大坑预警】**：登录后，请立即给自己充值，否则后续测试会一直报错。
    -   点击左侧 **用户管理** -> 点击你的用户名 -> **更多** -> **充值**。
    -   充值金额填一个很大的数字，比如 `1000` 美元。

#### 4.2 创建分组 (Group)

分组用来区分不同的账号池。因为你的 Business 账号走的是 OpenAI 协议，我们创建一个 OpenAI 分组。

1.  点击左侧 **分组管理** -> **添加分组**。
2.  **分组名称**：`Business_Account_Pool`
3.  **平台类型**：选择 **OpenAI**。
4.  点击 **提交**。

#### 4.3 添加 Business 账号 (OAuth 方式)

针对每一个 Business 账号，重复以下步骤：

1.  点击左侧 **账号管理** -> **添加账号**。
2.  **账号名称**：自定义，如 `Business_01`。
3.  **平台**：选择 **OpenAI**。
4.  **类型**：选择 **OAuth**。
5.  点击 **提交** 后，页面会弹出一个 **授权链接** (很长的一串 URL)。
6.  **【核心操作】**：复制这个授权链接，在**浏览器的新无痕窗口**中打开。
7.  在新窗口中登录你的 **ChatGPT Business 账号**。
8.  **【极其重要】**：登录成功后，页面可能会让你选择 **工作空间 (Workspace)**。**必须选择你的 Business 团队空间**，而不是个人空间。否则无法使用 Team 配额。
9.  授权后，浏览器地址栏会变成 `http://localhost/?code=xxx...`。
10. **完整复制**这个 `http://localhost` 开头的整段链接。
11. 回到 Sub2API 后台，将复制的链接粘贴到 **回调地址** 输入框中，点击 **提交**。
12. 添加成功后，点击该账号右侧的 **测试连接**。如果显示“连接成功”，说明账号生效了。
13. **重复**：对下一个 Business 账号执行 1-12 步，直到所有账号都添加完毕。

#### 4.4 将账号加入分组

1.  回到 **账号管理** 列表。
2.  点击每个账号右侧的 **编辑**。
3.  在 **分组** 下拉框中，勾选你之前创建的 `Business_Account_Pool`。
4.  保存。

#### 4.5 创建 API Key

1.  点击左侧 **API 管理** -> **添加 API**。
2.  **名称**：`For_OpenClaw`
3.  **关联分组**：勾选 `Business_Account_Pool`。
4.  **提交**。
5.  生成后，你会看到一个以 `sk-` 开头的密钥。**请立即复制保存**，关闭页面后就看不到了。

---

### 🤖 阶段五：对接 OpenClaw

现在你的中转站 API 已经准备好了，我们把它接入 OpenClaw。

#### 5.1 环境变量配置法 (推荐，简单快速)

在你的云服务器上，找到 OpenClaw 的启动脚本或服务文件，在启动前注入环境变量：

```bash
# 设置 API 地址和密钥
export OPENAI_BASE_URL="https://api.yourdomain.com/v1"
export OPENAI_API_KEY="sk-你刚刚生成的密钥"

# 启动 OpenClaw (根据你的实际启动命令调整)
openclaw serve --host 0.0.0.0 --port 3000
```

#### 5.2 永久生效 (Systemd 服务)

如果你使用 systemd 管理 OpenClaw，编辑服务文件：

```bash
sudo nano /etc/systemd/system/openclaw.service
```

在 `[Service]` 部分下面添加 Environment 行：

```ini
[Service]
Environment="OPENAI_BASE_URL=https://api.yourdomain.com/v1"
Environment="OPENAI_API_KEY=sk-你刚刚生成的密钥"
ExecStart=/path/to/your/openclaw serve
```

保存后，重载并重启服务：

```bash
sudo systemctl daemon-reload
sudo systemctl restart openclaw
```

---

### ✅ 阶段六：验证与确认

完成所有配置后，我们来验证整个链路是否通畅。

1.  **验证 Sub2API 节点**：
    在服务器终端执行：
    ```bash
    curl https://api.yourdomain.com/v1/models -H "Authorization: Bearer sk-你刚刚生成的密钥"
    ```
    **预期结果**：返回一个包含模型列表的 JSON 数据。如果没有报错，说明中转站 API 工作正常。

2.  **验证 OpenClaw**：
    向 OpenClaw 发送一条简单的消息 (如 "Hello")。
    然后去 **Sub2API 后台** -> **运维日志**，查看是否有成功的调用记录，以及 Token 消耗是否正确。

3.  **确认故障转移 (可选)**：
    进入 Sub2API 后台的账号管理，**禁用**其中一个 Business 账号 (点击调度按钮变成灰色)。
    再次通过 OpenClaw 发起请求。查看日志，确认请求自动路由到了另一个可用的账号上。

### 💡 避坑指南与总结

- **坑点 1**：**OAuth 必须选 Workspace**。不选 Workspace，系统会默认走个人免费版配额，而 Business 账号的个人配额通常是 0，会导致调用失败。
- **坑点 2**：**必须充值**。Sub2API 默认余额为 0，即使上游账号有额度，下游也无法调用。
- **坑点 3**：**BaseURL 必须带 `/v1`**。在 OpenClaw 配置中，`OPENAI_BASE_URL` 的结尾必须是 `/v1`，否则会报 404 错误。
- **坑点 4**：**代理和域名**。不要直接暴露 8080 端口，用 Nginx 反向代理并开启 HTTPS，否则前端控制台可能报错，且 API Key 有泄露风险。

按照这套流程操作下来，你相当于拥有了一套企业级的 API 网关。以后额度快用完了，只需要去 Sub2API 后台添加新的 Business 账号，OpenClaw 那边无需任何改动，即可自动享有新账号的额度。