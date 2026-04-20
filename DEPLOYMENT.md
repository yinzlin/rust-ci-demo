# ============================================
# CI/CD 完整部署指南
# ============================================

## 目录
1. [GitHub仓库准备](#1-github仓库准备)
2. [GitHub Secrets配置](#2-github-secrets配置)
3. [Ubuntu服务器准备](#3-ubuntu服务器准备)
4. [首次部署](#4-首次部署)
5. [验证部署](#5-验证部署)

---

## 1. GitHub仓库准备

### 1.1 创建/重命名仓库
确保你的GitHub仓库名称是唯一的（不要用 `demo`，建议用 `rust-ci-demo`）

### 1.2 配置远程仓库
```bash
# 移除旧的origin
 git remote remove origin

# 添加新的origin（替换为你的GitHub用户名）
 git remote add origin https://github.com/你的用户名/rust-ci-demo.git

# 推送代码
 git add .
 git commit -m "feat: 完善CI/CD流程"
 git push -u origin main
```

---

## 2. GitHub Secrets配置

在GitHub仓库的 **Settings > Secrets and variables > Actions > New repository secret** 中添加以下密钥：

| Secret名称 | 说明 | 获取方式 |
|------------|------|----------|
| `SSH_PRIVATE_KEY` | 服务器SSH私钥 | 见下方生成 |
| `GHCR_TOKEN` | GitHub Container Registry Token | 见下方生成 |

### 2.1 生成SSH密钥对
在你的本地电脑上生成（Windows PowerShell或Linux/Mac终端）：
```bash
# 生成SSH密钥对（一路回车，不设密码）
ssh-keygen -t ed25519 -C "github-actions-deploy"

# 查看公钥（复制到服务器）
cat ~/.ssh/id_ed25519.pub

# 查看私钥（复制到GitHub Secrets的 SSH_PRIVATE_KEY）
cat ~/.ssh/id_ed25519
```

### 2.2 生成GHCR_TOKEN
1. 访问: https://github.com/settings/tokens
2. 点击: `Generate new token` → `Generate new token (classic)`
3. 勾选权限: `repo`, `write:packages`, `read:packages`
4. 生成并复制Token（只显示一次！）

---

## 3. Ubuntu服务器准备（39.96.223.65）

### 3.1 登录服务器
```bash
ssh root@39.96.223.65
```

### 3.2 安装Docker和Docker Compose
```bash
# 更新包列表
apt update && apt upgrade -y

# 安装Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# 启动Docker服务
 systemctl start docker
 systemctl enable docker

# 安装Docker Compose
apt install docker-compose -y

# 验证安装
 docker --version
 docker-compose --version
```

### 3.3 配置SSH认证
将你本地生成的公钥添加到服务器：
```bash
# 在服务器上编辑 ~/.ssh/authorized_keys
 nano ~/.ssh/authorized_keys
# 粘贴你的公钥（id_ed25519.pub的内容），保存退出

# 设置正确权限
chmod 600 ~/.ssh/authorized_keys
chmod 700 ~/.ssh
```

### 3.4 登录GitHub Container Registry
```bash
# 登录ghcr.io（使用GHCR_TOKEN）
echo "你的GHCR_TOKEN" | docker login ghcr.io -u 你的GitHub用户名 --password-stdin
```

---

## 4. 首次部署

### 4.1 推送代码触发CI/CD
```bash
# 确保所有更改已提交
 git add .
 git commit -m "chore: 完成CI/CD配置"
 git push origin main
```

### 4.2 查看GitHub Actions
访问: https://github.com/你的用户名/rust-ci-demo/actions

---

## 5. 验证部署

### 5.1 检查应用是否运行
在服务器上：
```bash
# 查看容器状态
 docker ps

# 查看应用日志
 docker logs rust-ci-demo -f
```

### 5.2 访问应用
打开浏览器访问: http://39.96.223.65:8080

---

## 日常部署流程

每次推送到main分支后：
1. CI自动运行（格式化、Clippy、测试、构建）
2. CD自动触发（构建Docker镜像、推送到ghcr.io）
3. 自动部署到Ubuntu服务器

---

## 故障排查

### 问题：无法连接到服务器
检查：
- SSH密钥是否正确配置
- 服务器防火墙是否开放22端口
- GitHub Secrets是否正确设置

### 问题：Docker镜像拉取失败
检查：
- GHCR_TOKEN是否正确配置
- 服务器是否已登录ghcr.io
- 镜像名称是否正确

### 问题：应用无法访问
检查：
- 容器是否正常运行: `docker ps`
- 应用日志: `docker logs rust-ci-demo`
- 服务器防火墙是否开放8080端口: `ufw status`

---

## 技术栈符合规则检查

✅ 规则1: rustfmt + clippy检查  
✅ 规则7: Docker部署 + release构建  
✅ 规则9: edition 2024  
✅ 规则11: 所有类型都有注释