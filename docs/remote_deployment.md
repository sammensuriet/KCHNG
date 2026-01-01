# Remote Nix Deployment Guide for KCHNG

This guide explains different approaches to deploy KCHNG from your local machine to the VPS without manually logging in.

## Quick Answer

**Yes!** You can deploy from your local machine. Choose one of these approaches:

| Approach | Complexity | Best For | Command |
|----------|-----------|----------|---------|
| **Shell Script** | ⭐ Simple | Quick deployments | `./scripts/deploy-remote.sh` |
| **deploy-rs** | ⭐⭐ Medium | Nix-native workflows | `nix run .#deploy` |
| **Colmena** | ⭐⭐ Medium | Multi-server fleets | `colmena apply` |

---

## Option 1: Shell Script (Simplest - Recommended)

**File:** `scripts/deploy-remote.sh`

```bash
# From local machine
./scripts/deploy-remote.sh
```

**What it does:**
1. Builds frontend locally
2. Syncs files via rsync
3. Builds on VPS
4. Reloads nginx
5. Runs health check

**Pros:**
- ✅ Simple to understand
- ✅ Works with current setup (static site + nginx)
- ✅ Easy to customize

**Cons:**
- ❌ Not pure Nix
- ❌ Requires manual SSH setup

---

## Option 2: deploy-rs (Nix-Native)

**Setup:**

```bash
# 1. Install deploy-rs CLI
nix profile install nixpkgs#deploy-rs

# 2. Configure SSH keys (passwordless login)
ssh-copy-id deployman@102.68.84.79

# 3. Deploy from local machine
nix run .#deploy
```

**What it does:**
- Builds packages via Nix
- Uploads to remote via SSH
- Activates new profile
- Rollback on failure

**Pros:**
- ✅ Pure Nix workflow
- ✅ Atomic deployments
- ✅ Automatic rollbacks
- ✅ Declarative

**Cons:**
- ❌ More complex setup
- ❌ Overkill for simple static sites

---

## Option 3: Colmena (Alternative to deploy-rs)

**Setup:**

```bash
# 1. Install Colmena
nix profile install nixpkgs#colmena

# 2. Create hive.nix (deployment config)
# 3. Deploy
colmena apply
```

**Pros:**
- ✅ Good for multiple servers
- ✅ Parallel deployments
- ✅ NixOS native

**Cons:**
- ❌ Steeper learning curve
- ❌ Designed for NixOS

---

## Option 4: Manual SSH + Nix Build

**Simple ad-hoc approach:**

```bash
# Build and deploy in one command
ssh deployman@102.68.84.79 "cd /opt/deployments/kchng && nix build .#kchng-frontend"

# Or build locally, transfer, activate
nix build .#kchng-frontend
nix copy --to ssh://deployman@102.68.84.79 .#kchng-frontend
ssh deployman@102.68.84.79 "nix-env -p /nix/var/nix/profiles/per-user/deployman/kchng -iA kchng-frontend"
```

---

## Recommended Approach for KCHNG

Since KCHNG is a **static site served by nginx on Ubuntu**, use:

### Shell Script Workflow

```bash
# Initial setup (one-time)
ssh-copy-id deployman@102.68.84.79  # For passwordless SSH

# Deploy changes
./scripts/deploy-remote.sh
```

### Why This Works Best

1. **Simple** - No complex Nix deployment tools needed
2. **Fast** - rsync transfers only changed files
3. **Reliable** - Works with existing nginx setup
4. **Easy to debug** - Can see each step

---

## Setting Up SSH Keys (One-Time)

For passwordless deployment:

```bash
# 1. Generate SSH key (if you don't have one)
ssh-keygen -t ed25519 -C "your@email.com"

# 2. Copy key to VPS
ssh-copy-id deployman@102.68.84.79

# 3. Test passwordless login
ssh deployman@102.68.84.79

# 4. Add to ~/.ssh/config (optional)
Host kachi-ng
    HostName 102.68.84.79
    User deployman
    IdentityFile ~/.ssh/id_ed25519
```

---

## Multi-App Deployment (Future: GbamGbam)

When you add GbamGbam, you can deploy multiple apps:

```bash
# Using deploy-rs
nix run .#deploy  # Deploys all configured nodes

# Or selectively
nix run .#deploy.kachi-ng    # Just KCHNG
nix run .#deploy.gbamgbam-ng # Just GbamGbam
```

**flake.nix configuration:**
```nix
deploy = {
  nodes = {
    kachi-ng = {
      hostname = "102.68.84.79";
      profiles.kchng = { ... };
    };
    gbamgbam-ng = {
      hostname = "102.68.84.79";
      profiles.gbamgbam = { ... };
    };
  };
};
```

---

## Comparison Summary

| Feature | Shell Script | deploy-rs | Colmena |
|---------|--------------|-----------|---------|
| **Setup complexity** | Low | Medium | High |
| **Learning curve** | Low | Medium | High |
| **Nix-native** | ❌ | ✅ | ✅ |
| **Rollbacks** | Manual | Auto | Auto |
| **Multi-server** | Manual | Built-in | Built-in |
| **Current setup** | ✅ Works | ⚠️ Requires config | ❌ Overkill |

---

## Quick Reference

### Deploy from local machine:
```bash
# Simple (recommended)
./scripts/deploy-remote.sh

# Nix-native (complex)
nix run .#deploy

# Manual SSH
ssh deployman@102.68.84.79 "cd /opt/deployments/kchng && pnpm --filter frontend build"
```

### Check deployment status:
```bash
curl https://kachi.ng/
```

### View logs:
```bash
ssh deployman@102.68.84.79 "tail -f /var/log/nginx/kachi-access.log"
```

---

## Conclusion

**For now:** Use `scripts/deploy-remote.sh` - it's simple and works with your current setup.

**Future:** Consider deploy-rs if you:
- Need automatic rollbacks
- Deploy to multiple servers
- Want pure Nix workflow
- Add GbamGbam and more apps
