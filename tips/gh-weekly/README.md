# gh-weekly - GitHub Weekly Report Generator

A tool for analyzing GitHub commit history and automatically generating weekly reports using OpenAI-compatible APIs.

## Features

- 🔍 Automatically fetches GitHub commits from the past week
- 🤖 Generates intelligent reports using AI (DeepSeek, OpenAI, OpenRouter, etc.)
- 🎯 Supports repository filtering and exclusion
- 📝 Customizable report character count (default: 500 chars)
- ✨ Custom writing style support via positional arguments
- 🔧 Raw commit information viewing
- 🌐 Multi-service AI API support with standard OpenAI configuration

## Requirements

### System Dependencies
- Python 3.6+
- GitHub CLI (`gh`) - [Installation Guide](https://cli.github.com/)

### Python Dependencies
```bash
pip install openai
```

### Configuration
1. **GitHub CLI Authentication**
   ```bash
   gh auth login
   ```

2. **AI API Configuration** (Standard OpenAI environment variables)
   ```bash
   # DeepSeek (default)
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

## Usage

### Basic Usage

```bash
# Generate default 500-character weekly report
./gh-weekly.py

# Generate custom length report
./gh-weekly.py --chars 800

# Override API key via command line
./gh-weekly.py --api-key "sk-xxx"

# Custom writing style
./gh-weekly.py 简洁的 技术 总结

# Combined options
./gh-weekly.py 详细的 业务 报告 --chars 1000 --filter core
```

### Writing Style Customization

```bash
# Simple style description
./gh-weekly.py 简洁 总结

# Multi-word style description
./gh-weekly.py 详细的 技术 分析 报告

# Style with other options
./gh-weekly.py 面向 管理层 的 高级 总结 --chars 800 --filter core
```

### Filtering Options

```bash
# Include only repositories containing "plugin" in name
./gh-weekly.py --filter plugin

# Exclude repositories containing "test" in name
./gh-weekly.py --exclude test

# Combine filters with custom style
./gh-weekly.py 简洁 报告 --filter "utils|tools" --exclude "temp|backup"
```

### Raw Information View

```bash
# View raw commit information instead of generating report
./gh-weekly.py --raw

# View filtered raw information
./gh-weekly.py --raw --filter plugin
```

## Supported AI Services

The tool uses standard OpenAI environment variables and works with multiple AI services:

### DeepSeek (Default)
```bash
export OPENAI_API_KEY="sk-deepseek-key"
# Uses: https://api.deepseek.com with deepseek-chat model
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
# Also supports: openai/gpt-4o, meta-llama/llama-3.1-405b, etc.
```

### Any OpenAI-Compatible API
Configure `OPENAI_BASE_URL` and `OPENAI_MODEL` for your service.

## Example Use Cases

### Team Reports
```bash
# Generate detailed report including all projects
./gh-weekly.py 团队 周报 --chars 800

# Focus only on core product related commits
./gh-weekly.py 核心 产品 进展 --filter "core|main|prod" --exclude "test|demo"
```

### Personal Summary
```bash
# Concise personal weekly summary
./gh-weekly.py 个人 总结 --chars 300

# Technical focus
./gh-weekly.py 技术 工作 汇报 --raw --filter "personal|side"
```

### Cross-Platform Usage
```bash
# Use with different AI services
export OPENAI_API_KEY="sk-openai-key"
export OPENAI_BASE_URL="https://api.openai.com/v1"
export OPENAI_MODEL="gpt-4"
./gh-weekly.py 高层 汇报 --chars 600 --filter "core|main"
```

### Automation Integration
```bash
#!/bin/bash
# Example cron job script
export OPENAI_API_KEY="your-key"
cd /path/to/project
./gh-weekly.py 自动化 周报 --chars 600 > "weekly_$(date +%Y%m%d).txt"
```
