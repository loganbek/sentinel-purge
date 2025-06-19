# sentinel-purge

Here's a brainstorm for an advanced, cross-platform APT (Advanced Persistent Threat) Removal Tool, designed to be thorough, stealth-aware, and capable of deep remediation over days or weeks. Let's call it:

🛡️ "SentinelPurge" — APT Hunting & Eradication Framework
🔧 Core Goals
Identify and remove APTs without compromising system functionality.

Work on Windows, macOS, and Linux.

Remain stealthy to avoid detection by the APT.

Time-flexible: Operate over long periods for deep scanning (days/weeks).

Minimal user input but with advanced controls for power users.

Can recover, restore, or rollback compromised systems.

🧠 Key Features
1. Cross-Platform Agent (Rust + Golang Core)
Lightweight background agent with modular design.

Hooks into system-level APIs for detection.

Executes sandboxed modules to reduce infection risk.

2. Hybrid Scanning Engine
Combines signature-based, behavioral, heuristic, and ML-driven methods.

Pulls from open threat intel (MISP, VirusTotal, YARA rules, Sigma rules).

Conducts longitudinal scanning (e.g., compare process/network anomalies over time).

3. Forensic Baseline Mapping
Builds a known-good baseline of system files, autoruns, registry settings, daemons, etc.

Compares against a global reputation DB and anomaly patterns.

Supports timeline and graph-based forensic analysis.

4. Memory & Kernel Analysis
Live memory snapshotting and periodic diffing.

Detects rootkits, kernel-level hooks, injected code.

Leverages volatility framework (extended cross-platform).

5. Network Sinkhole Monitor
Detects and optionally reroutes traffic to suspected C2 servers to a local sinkhole for analysis.

DNS poisoning detection, DoH/TOR proxy detection.

Built-in firewall rules enforcement for containment.

6. Slow Burn Remediation Engine
Gradual, stealthy removal to prevent retaliation or self-deletion triggers.

Optional live kill-switch to quarantine everything instantly.

Support for staging removal steps (e.g., "delay registry removal by 2 days").

7. Isolation & Recovery
Can snapshot entire systems (VM-style) before starting removal.

Sandboxes suspicious processes in real time.

Integration with cloud backups (optional) to restore uninfected states.

8. Covert Mode
Appears as a generic background service/process.

Can rename its components and change hashes to avoid APT targeting.

Optional "sleep mode" to remain undetectable for hours/days.

💻 OS-Specific Adaptations
🪟 Windows
Sysinternals integration, ETW telemetry, WMI analysis.

LSASS and SAM monitoring.

Automatic group policy reset.

🐧 Linux
Analyzes systemd, crontabs, /etc/rc.local, and system binaries.

Looks for LD_PRELOAD and .so injection.

Checks init scripts, SSH authorized_keys, sudo abuse.

🍎 macOS
Monitors LaunchAgents, LaunchDaemons, kernel extensions.

Detects TCC.db abuse, AppleScript backdoors, hidden apps.

📊 User Interface & Logging
CLI + GUI frontend.

Centralized dashboard (optional web interface or local-only).

Audit trail for every action (for forensics or compliance).

Log anonymization/sharing option for crowd-sourced detection.

🔒 Security Considerations
Open Source Core with optional hardened binaries.

No cloud dependency by default (can run fully air-gapped).

Signed updates with reproducible builds.

🧰 Optional Modules
Active Deception: Lure the APT into revealing itself (fake credentials, honeypots).

Auto Threat Reporting: Submits unknown indicators to hybrid analysis backends.

SIEM Export: Splunk/ELK/Sentinel one-click integration.

## 📦 Installation

### Prerequisites
- **Rust 1.56+**: SentinelPurge is built with Rust 2021 edition. Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Building from Source
```bash
# Clone the repository
git clone https://github.com/loganbek/sentinel-purge.git
cd sentinel-purge

# Build the project (release build recommended)
cargo build --release

# Alternative: Build for development (faster compilation, larger binary)
cargo build

# Run tests to verify installation
cargo test
```

The compiled binary will be available at:
- Release build: `target/release/sentinel-purge`
- Development build: `target/debug/sentinel-purge`

### Quick Installation Test
```bash
# Verify the binary works
./target/release/sentinel-purge --version
./target/release/sentinel-purge --help
```

## 🚀 Usage

### Basic Usage
```bash
# Show help and available options
./target/release/sentinel-purge --help

# Run in interactive mode with default settings
./target/release/sentinel-purge

# Run in daemon mode
./target/release/sentinel-purge --daemon

# Run with specific stealth mode
./target/release/sentinel-purge --stealth adaptive

# Run with custom configuration file
./target/release/sentinel-purge --config /path/to/config.json

# Run with debug logging
./target/release/sentinel-purge --log-level debug
```

**Note**: Replace `target/release/sentinel-purge` with `target/debug/sentinel-purge` if using a development build.

### Command Line Options
- `-c, --config <FILE>`: Specify configuration file path
- `-s, --stealth <MODE>`: Set stealth operation mode (silent, hibernation, mimicry, ghost, adaptive)
- `-l, --log-level <LEVEL>`: Set log level (trace, debug, info, warn, error)
- `-d, --daemon`: Run as daemon/service
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Stealth Modes
- **Silent**: Minimal system footprint (default)
- **Hibernation**: Periodically sleeps to avoid detection
- **Mimicry**: Disguises as common system processes
- **Ghost**: Advanced evasion techniques
- **Adaptive**: Dynamically adjusts based on environment

⚠️ **Security Note**: This tool requires elevated privileges for deep system analysis. Ensure you understand the security implications before use.

## 🚀 Contributing & Issue Management

### Issue Templates
This project uses structured GitHub issue templates to ensure comprehensive requirements capture and consistent development tracking. When creating new issues, please use the appropriate template:

- **🏗️ Cross-Platform Agent**: Core agent infrastructure, module system, IPC
- **🔍 Detection Engine**: Threat detection, ML models, signature-based scanning  
- **🔒 Security Implementation**: Cryptography, hardening, compliance features
- **🖥️ Platform-Specific**: Windows/Linux/macOS specific implementations
- **🥷 Covert Operations**: Stealth features, anti-detection, operational security
- **✨ Feature Request**: General feature requests and enhancements
- **🐛 Bug Report**: Software defects and unexpected behavior

### Quick Start for Contributors
1. **Browse Issues**: Check existing [issues](https://github.com/sentinel-purge/sentinel-purge/issues) and [project board](https://github.com/sentinel-purge/sentinel-purge/projects)
2. **Select Template**: Use appropriate issue template for new requirements
3. **Follow Guidelines**: See [`.github/ISSUE_TEMPLATE/README.md`](./.github/ISSUE_TEMPLATE/README.md) for detailed guidance
4. **Security First**: All contributions must follow security-first development principles

### Development Roadmap
For detailed development specifications and implementation guides, see the [issues directory](./issues/README.md) which contains the original development requirements that informed these GitHub issue templates.
