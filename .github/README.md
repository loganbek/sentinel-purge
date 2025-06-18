# AI Development Tools Configuration

This directory contains configuration files for AI-powered development tools used in the SentinelPurge project.

## Files Overview

### `.coderabbit.yaml`
Configuration for CodeRabbit AI code review tool:
- **Security-focused reviews** for APT removal tool development
- **High-priority patterns** for security-critical files
- **Custom rules** for memory safety, error handling, and privacy
- **Platform-specific** review guidelines for Windows, macOS, and Linux
- **Performance considerations** for long-running background operations

Key features:
- Comprehensive security review settings
- Memory safety enforcement for Rust code
- Cross-platform compatibility checks
- Stealth operation considerations
- Privacy and logging security

### `copilot-instructions.md`
GitHub Copilot instructions for the SentinelPurge project:
- **Project context** and architecture overview
- **Security-first** development guidelines
- **Platform-specific** implementation notes
- **Performance requirements** for extended operation
- **Recommended dependencies** and project structure

### `.copilotignore`
Excludes sensitive files from Copilot suggestions:
- Security configuration files and credentials
- Threat intelligence and signature databases
- Forensic evidence and memory dumps
- Log files with potentially sensitive information
- Build artifacts and temporary files

### VS Code Configuration (`.vscode/`)

#### `settings.json`
- GitHub Copilot optimization for security development
- Rust-analyzer configuration with security linting
- Editor settings for consistent code formatting
- File associations for security-related formats

#### `launch.json`
Debug configurations for:
- Main SentinelPurge application
- Unit and integration tests
- Individual security modules

#### `extensions.json`
Recommended VS Code extensions:
- Rust development tools
- GitHub Copilot and Copilot Chat
- Security-focused development aids
- Debugging and analysis tools

## Usage Notes

These configurations are designed specifically for developing a security tool that:
- Operates in adversarial environments
- Handles sensitive security data
- Requires high reliability and stealth
- Must work across multiple platforms
- Needs robust error handling and logging

The AI tools are configured to prioritize security best practices, memory safety, and privacy considerations appropriate for an APT removal tool.