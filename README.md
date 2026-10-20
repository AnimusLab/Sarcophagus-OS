# Sarcophagus OS

<p align="center">
  <img src="docs/assets/logo.png" alt="Sarcophagus OS Logo" width="180"/>
</p>

<p align="center">
  <strong>An Uncompromising, Privacy-First Mobile OS Architecture</strong><br/>
  Decentralized Sharding | Eidolon Obfuscation | Bus-Level Hardware Isolation
</p>

---

## 📌 Overview

**Sarcophagus OS** is an open-source, zero-telemetry mobile operating system built from the ground up for absolute privacy, anonymity, and user sovereignty.

Traditional mobile operating systems continuously transmit identity metrics, cell tower metadata, and telemetry back to central infrastructure. Sarcophagus OS cryptographically buries state leaks, enforces transparent onion routing, and shards activity across a decentralized peer network—aiming to reduce digital attribution.

---

## 🏛️ Core Architecture

```text
+-----------------------------------------------------------------------+
|                            SARCOPHAGUS OS                             |
|        [ Zero-Telemetry | #000000 OLED Shell | Airgap Control ]        |
+-----------------------------------------------------------------------+
|                                                                       |
|  +-------------------------------+--------------------------------+   |
|  |                               |                                |   |
|  v                               v                                v   |
| [ EIDOLON NETWORK CORE ]  [ PULSE PAGER SYSTEM ]  [ SHARDED CONTAINER POOL ]
|  • Default DNS Scrambler   • Dual-Transport Mesh  • Append-Only Local Ledgers
|  • Tor TransPort Isolation   (BLE 5.0 / Tor V3)   • Cross-User Decoy Activity
|  • Traffic Morphing        • Double Ratchet E2EE • K-Anonymity Forensic Shield
|  • Zero-Leak Socket Policy • Self-Destruct Links  • Ring-Signed Gossip Protocol
|                                                                       |
+-----------------------------------------------------------------------+
|                                  v                                    |
|       [ KERNEL & BUS-LEVEL HARDWARE KILL-SWITCHES ]                   |
|       • Driver Unbinding & Bus Power Isolation                        |
|       • 3-Tier Security Profiles (Standard / Stealth / Faraday)       |
+-----------------------------------------------------------------------+
```

---

## ✨ Key Features

- **Eidolon Network Engine:** Native DNS scrambler and system-wide obfuscation layer routing socket traffic through local Tor transparent proxies (`nftables`).
- **Pulse Pager System:** A serverless, phone-numberless digital pager supporting local offline Bluetooth/Wi-Fi mesh chatter and global Tor v3 onion-service routing.
- **Decoy Activity Sharding:** Local logs and background synthetic activity are hashed (`BLAKE3`), chunked, and scattered across a decentralized P2P network using ring signatures to reduce forensic attribution.
- **Hardware Bus Isolation:** Kernel-level driver unbinding and power management intended to cut power to camera, microphone, and baseband radio controllers where supported by the device hardware.
- **#000000 OLED Interface:** A minimal, low-power Wayland shell (`wlroots`) with a true-black dark theme and dynamic 15–60 FPS rendering.

> **Implementation note:** These are architectural goals, not claims of currently implemented or independently verified protections. Hardware isolation, Tor routing, anonymity, and secure messaging depend on device support, correct implementation, and threat-model testing.

---

## 🛠️ Repository Structure

```text
sarcophagus-os/
├── build/                  # Build scripts & toolchain configs
├── docs/                   # Architectural blueprints & spec docs
├── kernel/                 # Custom kernel patches & bus kill-switch drivers
├── networking/             # Eidolon engine integration & nftables firewall rules
├── services/               # System daemons (Pulse Pager engine, P2P Shard manager)
├── ui/                     # Minimalist Wayland compositor & #000000 OLED shell
└── toolkit/                # Embedded mobile terminal & cyber auditing tools
```

---

## 🚀 Getting Started (WSL2 / Linux)

### Prerequisites

- Windows 11 with **WSL2** (Ubuntu / Debian), or a native Linux environment
- `build-essential`, `clang`, `git`, `rustup`, `cmake`, `ninja-build`

### Build Setup

```bash
# Clone the repository
git clone https://github.com/AnimusLab/sarcophagus-os.git
cd sarcophagus-os

# Initialize submodules and dependencies
git submodule update --init --recursive

# Build system daemons
cargo build --release
```

> **Note:** `cargo build --release` assumes the repository contains a valid Rust workspace at its root. Additional system packages, toolchains, device-specific configuration, and build steps may be required as implementation progresses.

---

## 📄 License

Maintained by **AnimusLab**. Distributed under the [Apache 2.0 License](LICENSE).
