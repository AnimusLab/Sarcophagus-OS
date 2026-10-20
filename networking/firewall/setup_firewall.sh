#!/usr/bin/env bash

# Sarcophagus OS - Firewall &  Networking isolation Setup Script
set -e

echo "[*] initializing Sarcophagus OS network Core..."

# !. ensure 'eidolon' system user exisits for socket filtering
if ! id "eidolon" &>/dev/null; then
    echo "[+] Creating system user 'eidolon' for proxy exemption..."
    sudo useradd -r -s /bin.false eidolon
fi

# 2. Apply nftables ruleset
echo "[+] Loading nftables ruleset from networking/firewall/nftables.conf..."
sudo nft -f networking/firewall/nftables.conf

# 3. Verify active ruleset
echo "[+] Active Firewall Chains:"
sudo nft list ruleset | grep -E "chain|redirect|policy"

echo "[✓] Network Core locked down successfully. All non-Tor traffic blocked."
