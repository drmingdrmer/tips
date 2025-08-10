#!/usr/bin/env python3
"""
GitHub Weekly Report Generator

A tool for analyzing GitHub commit history and automatically generating weekly reports.

Usage:
    ./gh-weekly.py                             # Generate default 500-char report
    ./gh-weekly.py --chars 500                 # Custom word count
    ./gh-weekly.py --filter plugin             # Filter repositories
    ./gh-weekly.py --exclude test              # Exclude repositories
    ./gh-weekly.py --raw                       # Show raw commits only
    ./gh-weekly.py --api-key "sk-xxx"          # Override API key
    ./gh-weekly.py 简洁的 技术 总结              # Custom writing style
    ./gh-weekly.py --since "2024-01-15"        # From specific date
    ./gh-weekly.py --days 14                   # Last 14 days

Requirements:
    - GitHub CLI (gh) installed and authenticated
    - OpenAI-compatible API configuration

Environment Variables:
    export DEEPSEEK_API_KEY="your-api-key"              # Preferred for DeepSeek (default)

Environment Variables when using other openAI compatible provider:
    export OPENAI_API_KEY="your-api-key"                # Fallback or for custom services
    export OPENAI_BASE_URL="https://api.openai.com/v1"  # Optional, defaults to DeepSeek
    export OPENAI_MODEL="gpt-4"                         # Optional, defaults to deepseek-chat

Supported Services:
    - DeepSeek (default): deepseek-chat
    - OpenAI: gpt-4, gpt-3.5-turbo
    - OpenRouter: anthropic/claude-3.5-sonnet, openai/gpt-4o
    - Any OpenAI-compatible API

Examples:
    # Use DeepSeek (default)
    export DEEPSEEK_API_KEY="sk-deepseek-key"
    ./gh-weekly.py --chars 150 --filter "core|main"

    # Use OpenAI
    export OPENAI_API_KEY="sk-openai-key"
    export OPENAI_BASE_URL="https://api.openai.com/v1"
    export OPENAI_MODEL="gpt-4"
    ./gh-weekly.py --chars 800

    # Use OpenRouter
    export OPENAI_API_KEY="sk-or-key"
    export OPENAI_BASE_URL="https://openrouter.ai/api/v1"
    export OPENAI_MODEL="anthropic/claude-3.5-sonnet"
    ./gh-weekly.py
"""
# /// script
# requires-python = ">=3.6"
# dependencies = [
#     "openai",
# ]
# ///
import subprocess
import json
import re
import os
from datetime import datetime, timedelta
import sys
import argparse
from openai import OpenAI

def log_info(message):
    print(message, file=sys.stderr)

def check_dependencies():
    """Check if required dependencies are installed and configured."""
    # Check if gh CLI is installed
    try:
        subprocess.run(['gh', '--version'], capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        log_info("❌ Error: GitHub CLI (gh) is not installed.")
        log_info("📋 Please install it from: https://cli.github.com/")
        log_info("💡 On macOS: brew install gh")
        log_info("💡 On Ubuntu: sudo apt install gh")
        sys.exit(1)

    # Check if gh CLI is authenticated
    try:
        result = subprocess.run(['gh', 'auth', 'status'], capture_output=True, check=True)
    except subprocess.CalledProcessError:
        log_info("❌ Error: GitHub CLI is not authenticated.")
        log_info("📋 Please run: gh auth login")
        log_info("💡 Follow the prompts to authenticate with your GitHub account")
        sys.exit(1)

    log_info("✅ GitHub CLI is installed and authenticated")

def parse_date(date_str):
    """Parse date string in various common formats."""
    if not date_str:
        return None
    
    # Common date formats to try
    formats = [
        '%Y-%m-%d',          # 2024-01-15
        '%Y/%m/%d',          # 2024/01/15
        '%m/%d/%Y',          # 01/15/2024
        '%d/%m/%Y',          # 15/01/2024
        '%Y-%m-%d %H:%M:%S', # 2024-01-15 10:30:00
        '%Y-%m-%d %H:%M',    # 2024-01-15 10:30
        '%m-%d-%Y',          # 01-15-2024
        '%d-%m-%Y',          # 15-01-2024
    ]
    
    for fmt in formats:
        try:
            return datetime.strptime(date_str, fmt)
        except ValueError:
            continue
    
    # Try ISO format with timezone
    try:
        return datetime.fromisoformat(date_str.replace('Z', '+00:00'))
    except ValueError:
        pass
    
    raise ValueError(f"Unable to parse date: {date_str}")

def calculate_since_date(args):
    """Calculate the since date based on args.since or args.days."""
    if args.since:
        try:
            since_date = parse_date(args.since)
            return since_date.strftime('%Y-%m-%d')
        except ValueError as e:
            log_info(f"❌ Error parsing date: {e}")
            log_info("💡 Supported formats: 2024-01-15, 2024/01/15, 01/15/2024, etc.")
            sys.exit(1)
    
    days_ago = args.days if args.days else 7
    since_date = datetime.now() - timedelta(days=days_ago)
    return since_date.strftime('%Y-%m-%d')

def generate_system_prompt(max_chars, style_requirements=None):
    style_instruction = f"\n\n**Additional Style Requirements**: {style_requirements}" if style_requirements else ""
    
    return f"""You are a technical writer tasked with creating a weekly progress report based on git commit history.

**Task**: Analyze the provided git commits from the past week and generate a comprehensive weekly report, in chinese, with no more than {max_chars} chars.{style_instruction}

**Report Requirements**:
1. **Structure**: Organize into clear sections (Major Accomplishments, Bug Fixes, Technical Metrics, etc.)
2. **Technical Focus**: Emphasize architectural changes, performance improvements, and significant refactoring
3. **Scope**: Cover the most impactful commits, not every minor change
4. **Context**: Group related commits into coherent feature/improvement themes
5. **Impact**: Highlight strategic importance and future implications

**Analysis Guidelines**:
- Read both commit titles AND body messages for complete context
- Identify patterns and themes across multiple commits
- Distinguish between major features, refactoring, bug fixes, and optimizations
- Note backward compatibility considerations and version impacts
- Recognize architectural decisions and their implications

**Output Format**: Professional technical report suitable for engineering teams, emphasizing progress on significant features and system improvements rather than routine maintenance tasks.

**Length**: Comprehensive but concise - focus on substance over length. For Chinese reports, limit to specified character count and single paragraph format when requested."""

def get_commit_details(repo_owner, repo_name, sha):
    try:
        result = subprocess.run([
            'gh', 'api', f'repos/{repo_owner}/{repo_name}/commits/{sha}'
        ], capture_output=True, text=True, check=True)
        commit_detail = json.loads(result.stdout)
        return commit_detail['commit']['message']
    except:
        return None

def display_commits(filtered_commits):
    log_info(f"📋 Displaying {len(filtered_commits)} commits:\n")

    for i, commit in enumerate(filtered_commits, 1):
        repo_name = commit['repository']['name']
        repo_owner = commit['repository']['owner']['login']
        sha = commit['sha']
        author_date = commit['commit']['author']['date']
        commit_subject = commit['commit']['message'].split('\n')[0]

        log_info(f"📝 [{i}/{len(filtered_commits)}] Fetching details for {repo_name}: {commit_subject}")

        # Get full commit message
        full_message = get_commit_details(repo_owner, repo_name, sha)
        if not full_message:
            full_message = commit['commit']['message']

        # Format date
        date_obj = datetime.fromisoformat(author_date.replace('Z', '+00:00'))
        formatted_date = date_obj.strftime('%Y-%m-%d %H:%M')

        print(f"[{repo_name}] {sha[:8]} - {formatted_date}")
        print(f"Message: {full_message}")
        print("---")

def fetch_recent_commits(args):
    since_date = calculate_since_date(args)

    if args.since:
        log_info(f"🔍 Searching for commits since {args.since} (parsed as {since_date})...")
    elif args.days:
        log_info(f"🔍 Searching for commits from last {args.days} days (since {since_date})...")
    else:
        log_info(f"🔍 Searching for commits from last 7 days (since {since_date})...")

    try:
        result = subprocess.run([
            'gh', 'search', 'commits',
            '--author=@me',
            f'--author-date=>{since_date}',
            '--json=repository,sha,commit,author,committer,url'
        ], capture_output=True, text=True, check=True)

        commits = json.loads(result.stdout)

        if not commits:
            log_info("❌ No commits found in the last week.")
            return

        log_info(f"✅ Found {len(commits)} commits from GitHub")

        # Apply filters
        filtered_commits = []
        for commit in commits:
            repo_name = commit['repository']['name']

            # Apply filter pattern
            if args.filter and not re.search(args.filter, repo_name, re.IGNORECASE):
                continue

            # Apply exclude pattern
            if args.exclude and re.search(args.exclude, repo_name, re.IGNORECASE):
                continue

            filtered_commits.append(commit)

        if args.filter or args.exclude:
            log_info(f"🔧 Applied filters: {len(filtered_commits)} commits remaining")

        if not filtered_commits:
            log_info("❌ No commits found matching the filter criteria.")
            return

        if args.raw:
            display_commits(filtered_commits)
        else:
            log_info(f"📊 Fetching detailed commit messages for {len(filtered_commits)} commits...")
            style_requirements = " ".join(args.style) if args.style else None
            return generate_weekly_report(filtered_commits, args.chars, args.api_key, style_requirements)

    except subprocess.CalledProcessError as e:
        log_info(f"Error running gh command: {e}")
        log_info(f"stderr: {e.stderr}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        log_info(f"Error parsing JSON output: {e}")
        sys.exit(1)
    except Exception as e:
        log_info(f"Unexpected error: {e}")
        sys.exit(1)

def generate_weekly_report(commits, max_chars=500, api_key=None, style_requirements=None):
    log_info("📝 Preparing commit data for weekly report...")

    # Prepare commit data for the report
    commit_info = []
    successful_fetches = 0

    for i, commit in enumerate(commits, 1):
        repo_name = commit['repository']['name']
        repo_owner = commit['repository']['owner']['login']
        sha = commit['sha']
        author_date = commit['commit']['author']['date']
        commit_subject = commit['commit']['message'].split('\n')[0]

        log_info(f"📋 [{i}/{len(commits)}] Processing {repo_name}: {commit_subject}")

        # Get full commit message
        full_message = get_commit_details(repo_owner, repo_name, sha)
        if not full_message:
            full_message = commit['commit']['message']
        else:
            successful_fetches += 1

        date_obj = datetime.fromisoformat(author_date.replace('Z', '+00:00'))
        formatted_date = date_obj.strftime('%Y-%m-%d %H:%M')

        commit_info.append(f"COMMIT: {sha}\nRepository: {repo_name}\nDate: {formatted_date}\nMessage: {full_message}\n")

    log_info(f"✅ Collected commit data ({successful_fetches}/{len(commits)} detailed messages fetched)")

    # Prepare prompt for DeepSeek
    commit_text = "\n".join(commit_info)
    system_prompt = generate_system_prompt(max_chars, style_requirements)

    # Get configuration from environment variables
    api_base = os.environ.get("OPENAI_BASE_URL", "https://api.deepseek.com")
    model = os.environ.get("OPENAI_MODEL", "deepseek-chat")
    
    if not api_key:
        # If using DeepSeek (default or explicit), prefer DEEPSEEK_API_KEY then fallback to OPENAI_API_KEY
        if api_base == "https://api.deepseek.com":
            api_key = os.environ.get("DEEPSEEK_API_KEY") or os.environ.get("OPENAI_API_KEY")
        else:
            # If other service is specified, only use OPENAI_API_KEY
            api_key = os.environ.get("OPENAI_API_KEY")

    # Call OpenAI-compatible API
    masked_key = f"{api_key[:5]}***{api_key[-4:]}" if api_key else "None"
    log_info(f"🤖 Generating weekly report using {api_base} - {model} - {masked_key} (max {max_chars} chars)...")

    if not api_key:
        log_info("❌ Error: API key is required")
        log_info("📋 Please provide API key:")
        log_info("   1. Command line: --api-key 'your-api-key'")
        if api_base == "https://api.deepseek.com":
            log_info("   2. Environment: export DEEPSEEK_API_KEY='your-api-key'  # Preferred for DeepSeek")
            log_info("      Or fallback: export OPENAI_API_KEY='your-api-key'")
        else:
            log_info("   2. Environment: export OPENAI_API_KEY='your-api-key'")
        log_info("💡 Optionally configure service:")
        log_info("   export OPENAI_API_KEY='your-api-key'")
        log_info("   export OPENAI_BASE_URL='https://api.openai.com/v1'")
        log_info("   export OPENAI_MODEL='gpt-4'")
        return

    try:
        client = OpenAI(api_key=api_key, base_url=api_base)

        response = client.chat.completions.create(
            model=model,
            messages=[
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": f"Generate a weekly report based on these commits:\n\n{commit_text}"},
            ],
            stream=False
        )

        log_info("✅ Weekly report generated successfully!")
        report_content = response.choices[0].message.content
        log_info(f"📏 Report length: {len(report_content)} characters")

        # Output the actual report to stdout
        print()
        print(report_content)

    except Exception as e:
        log_info(f"❌ Error generating report: {e}")

def main():
    parser = argparse.ArgumentParser(
        description='Generate weekly report from GitHub commits',
        epilog='Examples:\n'
               '  ./gh-weekly.py --chars 150 --filter "core|main"\n'
               '  ./gh-weekly.py --chars 800 --exclude "test|temp"\n'
               '  ./gh-weekly.py --raw --filter plugin\n'
               '  ./gh-weekly.py 简洁的 技术 总结 --chars 200\n'
               '  ./gh-weekly.py --since "2024-01-15" --chars 500\n'
               '  ./gh-weekly.py --days 30 技术 月报\n\n'
               'Environment Setup:\n'
               '  export OPENAI_API_KEY="your-key"\n'
               '  export OPENAI_BASE_URL="https://openrouter.ai/api/v1"\n'
               '  export OPENAI_MODEL="anthropic/claude-3.5-sonnet"',
        formatter_class=argparse.RawDescriptionHelpFormatter
    )

    # Filtering options
    parser.add_argument('--filter', '-f', help='Filter repositories by regex pattern (case-insensitive)')
    parser.add_argument('--exclude', '-e', help='Exclude repositories by regex pattern (case-insensitive)')

    # Output options
    parser.add_argument('--raw', action='store_true', help='Show raw commit information instead of generating report')
    parser.add_argument('--chars', '-c', type=int, default=500, help='Maximum characters for weekly report (default: 500)')

    # API configuration
    parser.add_argument('--api-key', '-k', help='API key (overrides OPENAI_API_KEY environment variable)')
    
    # Date/time options
    parser.add_argument('--since', help='Start date for commit search (e.g., "2024-01-15", "01/15/2024", "2024/01/15")')
    parser.add_argument('--days', '-d', type=int, help='Number of days ago to start search (default: 7)')
    
    # Style customization (positional arguments)
    parser.add_argument('style', nargs='*', help='Additional writing style requirements (e.g., 简洁的 技术 总结, 详细的 业务 报告)')

    args = parser.parse_args()
    
    # Validate date parameters
    if args.since and args.days:
        log_info("❌ Error: Cannot specify both --since and --days options")
        log_info("💡 Use either --since for absolute date or --days for relative days")
        sys.exit(1)

    # Check dependencies first
    if not args.raw:  # Skip dependency check for raw mode (doesn't need API)
        check_dependencies()

    fetch_recent_commits(args)

if __name__ == "__main__":
    main()
