# GitHub Copilot Instructions for SentinelPurge

## Project Overview
SentinelPurge is an advanced, cross-platform APT (Advanced Persistent Threat) removal tool designed for thorough, stealth-aware malware detection and remediation. The tool operates across Windows, macOS, and Linux systems with the capability to run extended scanning operations over days or weeks.

## Core Architecture
- **Primary Language**: Rust (for memory safety and performance)
- **Secondary Language**: Go (for specific modules)
- **Target Platforms**: Windows, macOS, Linux
- **Deployment**: Cross-platform agent with modular design

## Development Guidelines for Copilot

### Security-First Approach
When suggesting code, prioritize:
- Memory safety (avoid unsafe blocks unless absolutely necessary)
- Input validation and sanitization
- Secure error handling (avoid information disclosure)
- Privilege escalation prevention
- Cryptographic best practices
- Side-channel attack resistance

### Code Style Preferences
- **Rust**: Follow Rust standard conventions, use `cargo fmt` and `cargo clippy`
- **Error Handling**: Prefer `Result<T, E>` over `unwrap()` or `expect()`
- **Logging**: Use structured logging, avoid logging sensitive information
- **Documentation**: Include comprehensive rustdoc comments for public APIs
- **Testing**: Write unit tests, integration tests, and security-focused tests

### Architecture Patterns
- **Modular Design**: Suggest plugin/module architectures
- **Cross-Platform**: Use platform-specific code only when necessary
- **Async/Await**: Prefer async patterns for I/O operations
- **Resource Management**: Implement proper RAII patterns
- **Configuration**: Support both file-based and environment-based config

### Security Modules to Consider
1. **Scanner Engine**: Signature-based, behavioral, and ML-driven detection
2. **Memory Analysis**: Live memory snapshotting and analysis
3. **Network Monitor**: C2 detection and traffic analysis
4. **Forensic Tools**: Baseline mapping and timeline analysis
5. **Remediation Engine**: Gradual, stealth-aware threat removal
6. **Isolation**: Process sandboxing and system quarantine

### Platform-Specific Considerations

#### Windows
- Use Windows APIs appropriately (WinAPI, ETW, WMI)
- Handle registry operations securely
- Implement proper service/daemon patterns
- Consider UAC and privilege requirements

#### Linux
- Leverage systemd integration
- Handle various init systems
- Monitor cron jobs and system services
- Respect Linux security modules (SELinux, AppArmor)

#### macOS
- Use Cocoa/Core Foundation when needed
- Handle macOS security features (SIP, Gatekeeper)
- Monitor LaunchAgents and LaunchDaemons
- Respect macOS privacy permissions

### Performance Considerations
- **Background Operations**: Design for long-running, low-impact scanning
- **Memory Efficiency**: Optimize for extended runtime periods
- **CPU Usage**: Implement adaptive throttling based on system load
- **I/O Optimization**: Minimize filesystem impact during scanning
- **Battery Awareness**: Consider power consumption on mobile devices

### Privacy and Stealth Requirements
- **Covert Operation**: Suggest patterns for stealth operation
- **Process Naming**: Support dynamic process renaming
- **Signature Morphing**: Enable hash changing capabilities
- **Sleep Modes**: Implement hibernation/wake patterns
- **Network Anonymization**: Suggest secure communication patterns

### Dependencies and Libraries
Prefer well-maintained, security-audited crates:
- `tokio` for async runtime
- `serde` for serialization
- `clap` for CLI interfaces
- `tracing` for structured logging
- `secrecy` for sensitive data handling
- `ring` or `rustls` for cryptography
- `winapi` for Windows-specific operations
- Platform-specific crates as needed

### Testing Strategy
- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test cross-module functionality
- **Security Tests**: Include vulnerability and penetration testing
- **Platform Tests**: Verify cross-platform compatibility
- **Performance Tests**: Validate long-running operation efficiency
- **Stealth Tests**: Verify detection avoidance capabilities

### Code Review Focus Areas
When generating code, consider:
- Buffer overflow prevention
- Integer overflow/underflow checks
- Race condition avoidance
- Secure random number generation
- Proper credential handling
- Time-of-check vs time-of-use issues
- Directory traversal prevention
- SQL injection prevention (if applicable)
- Command injection prevention

### Suggested Project Structure
```
src/
├── main.rs
├── lib.rs
├── config/
├── scanner/
│   ├── signatures/
│   ├── behavioral/
│   └── ml/
├── memory/
├── network/
├── forensic/
├── remediation/
├── isolation/
├── stealth/
├── platform/
│   ├── windows/
│   ├── linux/
│   └── macos/
└── utils/
```

### Anti-Patterns to Avoid
- Hardcoded credentials or paths
- Excessive logging of sensitive information
- Blocking I/O in critical paths
- Unsafe memory operations without justification
- Platform-specific code in generic modules
- Unvalidated user input processing
- Synchronous operations in async contexts

## Usage Notes
This tool handles sensitive security operations. All suggestions should consider the adversarial environment in which this software will operate, including potential APT countermeasures and evasion techniques.
