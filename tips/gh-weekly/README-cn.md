# gh-weekly - GitHub 周报生成器

一个用于分析 GitHub commit 历史并自动生成中文周报的工具，支持多种 OpenAI 兼容的 AI 服务。

## 功能特性

- 🔍 自动获取最近一周的 GitHub commits
- 🤖 使用多种 AI 服务生成智能周报（DeepSeek、OpenAI、OpenRouter 等）
- 🎯 支持仓库过滤和排除
- 📝 可自定义报告字符数（默认：500 字符）
- ✨ 通过位置参数支持自定义写作风格
- 🔧 支持原始 commit 信息查看
- 🌐 多服务 AI API 支持，使用标准 OpenAI 配置

## 依赖要求

### 系统依赖
- Python 3.6+
- GitHub CLI (`gh`) - [安装指南](https://cli.github.com/)

### Python 包依赖
```bash
pip install openai
```

### 环境配置
1. **GitHub CLI 认证**
   ```bash
   gh auth login
   ```

2. **AI API 配置**（标准 OpenAI 环境变量）
   ```bash
   # DeepSeek（默认）
   export OPENAI_API_KEY="sk-deepseek-key"

   # OpenAI
   export OPENAI_API_KEY="sk-openai-key"
   export OPENAI_BASE_URL="https://api.openai.com/v1"
   export OPENAI_MODEL="gpt-4"

   # OpenRouter
   export OPENAI_API_KEY="sk-or-key"
   export OPENAI_BASE_URL="https://openrouter.ai/api/v1"
   export OPENAI_MODEL="anthropic/claude-3.5-sonnet"
   ```

## 使用方法

### 基本用法

```bash
# 生成默认 500 字周报
./gh-weekly.py

# 生成自定义长度周报
./gh-weekly.py --chars 800

# 使用命令行指定 API Key
./gh-weekly.py --api-key "sk-xxx"

# 自定义写作风格
./gh-weekly.py 简洁的 技术 总结

# 组合选项
./gh-weekly.py 详细的 业务 报告 --chars 1000 --filter core
```

### 写作风格定制

```bash
# 简单风格描述
./gh-weekly.py 简洁 总结

# 多词风格描述
./gh-weekly.py 详细的 技术 分析 报告

# 风格配合其他选项
./gh-weekly.py 面向 管理层 的 高级 总结 --chars 800 --filter core
```

### 过滤选项

```bash
# 只包含名称包含 "plugin" 的仓库
./gh-weekly.py --filter plugin

# 排除名称包含 "test" 的仓库
./gh-weekly.py --exclude test

# 组合过滤器与自定义风格
./gh-weekly.py 简洁 报告 --filter "utils|tools" --exclude "temp|backup"
```

### 原始信息查看

```bash
# 查看原始 commit 信息而非生成周报
./gh-weekly.py --raw

# 过滤后查看原始信息
./gh-weekly.py --raw --filter plugin
```

## 支持的 AI 服务

工具使用标准 OpenAI 环境变量，支持多种 AI 服务：

### DeepSeek（默认）
```bash
export OPENAI_API_KEY="sk-deepseek-key"
# 使用：https://api.deepseek.com 和 deepseek-chat 模型
```

### OpenAI
```bash
export OPENAI_API_KEY="sk-openai-key"
export OPENAI_BASE_URL="https://api.openai.com/v1"
export OPENAI_MODEL="gpt-4"
```

### OpenRouter
```bash
export OPENAI_API_KEY="sk-or-key"
export OPENAI_BASE_URL="https://openrouter.ai/api/v1"
export OPENAI_MODEL="anthropic/claude-3.5-sonnet"
# 也支持：openai/gpt-4o, meta-llama/llama-3.1-405b 等
```

### 任何 OpenAI 兼容 API
配置 `OPENAI_BASE_URL` 和 `OPENAI_MODEL` 以使用您的服务。

## 示例用法场景

### 团队周报
```bash
# 生成包含所有项目的详细周报
./gh-weekly.py 团队 周报 --chars 800

# 只关注核心产品相关的提交
./gh-weekly.py 核心 产品 进展 --filter "core|main|prod" --exclude "test|demo"
```

### 个人总结
```bash
# 简洁的个人周总结
./gh-weekly.py 个人 总结 --chars 300

# 技术工作汇报
./gh-weekly.py 技术 工作 汇报 --raw --filter "personal|side"
```

### 跨平台使用
```bash
# 使用不同的 AI 服务
export OPENAI_API_KEY="sk-openai-key"
export OPENAI_BASE_URL="https://api.openai.com/v1"
export OPENAI_MODEL="gpt-4"
./gh-weekly.py 高层 汇报 --chars 600 --filter "core|main"
```

### 自动化集成
```bash
#!/bin/bash
# 定时任务脚本示例
export OPENAI_API_KEY="your-key"
cd /path/to/project
./gh-weekly.py 自动化 周报 --chars 600 > "weekly_$(date +%Y%m%d).txt"
```
