# KCHNG Deployment Proposal for padawa.ng VPS

**Server**: 102.68.84.79 (Ubuntu 24.04.3 LTS)
**Purpose**: Deploy KCHNG frontend alongside existing Padawa PWA
**Domain**: kachi.ng
**Date**: 2025-01-01

---

## Current Infrastructure Overview

### Running Services
| Service | Type | Port | Management | Path |
|---------|------|------|------------|------|
| **Padawa PWA** | SvelteKit SSR | 3000 | PM2 cluster (2 instances) | `/opt/deployments/padawa` |
| **OKBackend** | Node/TS | 9000 | PM2 fork | `/opt/okbackend` |
| **nginx** | Reverse Proxy | 80/443 | systemd | `/etc/nginx/` |

### Domains Configured
- `padawa.ng` → localhost:3000
- `api.padawa.ng` → localhost:3000
- `www.padawa.ng` → localhost:3000
- `kachi.ng` → localhost:5173 (planned for KCHNG)

### System Resources
- **RAM**: 7.7GB (6.9GB available)
- **Disk**: 193GB (188GB available, 3% used)
- **Load**: 0.08 (very low)

---

## Proposed Architecture for KCHNG

### Option 1: Nix + Podman (Recommended - Declarative & Isolated)

```
┌─────────────────────────────────────────────────────────────┐
│                         nginx                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │ padawa.ng    │  │ kachi.ng     │  │ api.*       │        │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘        │
│         │                 │                 │                 │
│  ┌──────▼───────┐  ┌──────▼───────┐  ┌──────▼───────┐        │
│  │  PM2 Padawa  │  │  Podman     │  │ OKBackend    │        │
│  │  :3000       │  │  KCHNG:5173 │  │  :9000       │        │
│  └──────────────┘  │  (Nix build)│  └──────────────┘        │
│                    └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

**Pros:**
- ✅ Declarative package management with Nix
- ✅ Reproducible builds (same dependencies every time)
- ✅ Podman isolation (no daemon, rootless)
- ✅ Easy rollback with `nix-store --rollback`
- ✅ Works alongside existing PM2 setup
- ✅ Easy to add GbamGbam and future apps

**Cons:**
- ❌ Requires Nix installation (one-time)
- ❌ Requires Podman installation (one-time)
- ❌ Learning curve for Nix flakes

---

### Option 2: PM2 Only (Simple, Consistent with Existing)

```
┌─────────────────────────────────────────────────────────────┐
│                         nginx                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │ padawa.ng    │  │ kachi.ng     │  │ api.*       │        │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘        │
│         │                 │                 │                 │
│  ┌──────▼───────┐  ┌──────▼───────┐  ┌──────▼───────┐        │
│  │  PM2 Padawa  │  │  PM2 KCHNG   │  │ OKBackend    │        │
│  │  :3000       │  │  :5173       │  │  :9000       │        │
│  └──────────────┘  └──────────────┘  └──────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

**Pros:**
- ✅ Consistent with existing setup
- ✅ Easy to manage with PM2 commands
- ✅ Separate subdomain (`kachi.ng`)
- ✅ Independent deployment/rollback
- ✅ Minimal system changes
- ✅ No new tools to learn

**Cons:**
- ❌ No resource isolation
- ❌ Shared Node/npm versions for all apps

---

### Option 3: Podman Compose (Better Isolation, No Nix)

```
┌─────────────────────────────────────────────────────────────┐
│                         nginx                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │ padawa.ng    │  │ kachi.ng     │  │ api.*       │        │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘        │
│         │                 │                 │                 │
│  ┌──────▼──────┐  ┌──────▼─────────────────────┐             │
│  │ Podman      │  │ Podman Compose            │             │
│  │ (Padawa)    │  │ ┌────────┬────────┐       │             │
│  └─────────────┘  │ │ KCHNG  │ GbamGbam│     │             │
│                   │ │ :5173  │ :5174  │       │             │
│                   │ └────────┴────────┘       │             │
│                   └─────────────────────────────┘             │
└─────────────────────────────────────────────────────────────┘
```

**Pros:**
- ✅ Complete resource isolation
- ✅ No daemon (daemonless architecture)
- ✅ Rootless containers (more secure)
- ✅ Can run different Node versions per app
- ✅ `podman-compose` works like docker-compose
- ✅ Systemd integration: `podman generate systemd`

**Cons:**
- ❌ Need to install Podman
- ❌ No declarative package management (unless combined with Nix)

---

### Option 4: Systemd Services (Native Ubuntu, Declarative)

**Pros:**
- ✅ Native to Ubuntu
- ✅ Declarative service files
- ✅ Integrated journalctl logging
- ✅ Automatic restart on failure
- ✅ Resource limits built-in

**Cons:**
- ❌ More complex than PM2
- ❌ Doesn't match existing pattern

---

## Recommended: Option 1 (Nix + Podman) for Multi-App Scale

### Why This is Best for KCHNG + GbamGbam + Future Apps

**Nix Package Manager Benefits:**
```bash
# One command to install ALL dependencies for all apps
nix-shell  # Enters environment with exact Node 20.10.0, pnpm 8.15.0, etc.

# Reproducible builds - same result every time
nix build .#kchng  # Builds KCHNG container image
nix build .#gbamgbam  # Builds GbamGbam container image

# Easy rollback
nix-store --rollback  # Revert to previous package set
```

**Podman Benefits:**
```bash
# Rootless containers - more secure
podman run -d --name kchng -p 5173:5173 localhost/kchng:latest

# Generate systemd service automatically
podman generate systemd --name kchng > /etc/systemd/system/kchng.service

# Podman compose for multi-app
podman-compose up -d  # Starts KCHNG + GbamGbam together
```

**Together - Best of Both Worlds:**
```
Nix builds → Podman images → Systemd services → nginx reverse proxy
   ↓            ↓                  ↓                 ↓
Reproducible   Isolated         Auto-restart    kachi.ng
builds        containers        on failure      gbamgbam.ng
```

### Deployment Structure

```
/opt/deployments/
├── padawa/                    # Existing
│   ├── ecosystem.config.cjs
│   ├── build/
│   └── logs/
├── kchng/                     # New
│   ├── ecosystem.config.cjs
│   ├── current -> releases/kchng-v1.0.0
│   ├── releases/
│   │   ├── kchng-v1.0.0/
│   │   └── kchng-v1.0.1/
│   ├── scripts/
│   │   ├── deploy.sh
│   │   ├── rollback.sh
│   │   └── health-check.sh
│   └── logs/
└── shared/
    └── rollback-logs/
```

### Key Files

#### 1. PM2 Ecosystem (`/opt/deployments/kchng/ecosystem.config.cjs`)

```javascript
module.exports = {
  apps: [
    {
      name: 'kchng-frontend',
      script: './build/index.js',
      cwd: '/opt/deployments/kchng/current',
      instances: 1,
      exec_mode: 'fork',
      env: {
        NODE_ENV: 'production',
        PORT: 5173,
        // Contract configuration
        KCHNG_CONTRACT_ID: process.env.KCHNG_CONTRACT_ID || 'CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB',
        KCHNG_RPC_URL: process.env.KCHNG_RPC_URL || 'https://soroban-testnet.stellar.org',
        KCHNG_NETWORK: 'testnet'
      },
      error_file: '/opt/deployments/kchng/logs/error.log',
      out_file: '/opt/deployments/kchng/logs/out.log',
      log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
      merge_logs: true,
      autorestart: true,
      max_restarts: 10,
      min_uptime: '10s',
      max_memory_restart: '500M',
      shutdown_with_message: true
    }
  ]
};
```

#### 2. Deploy Script (`/opt/deployments/kchng/scripts/deploy.sh`)

```bash
#!/bin/bash
set -e

# KCHNG Deployment Script
# Usage: ./deploy.sh <version> [--rollback-on-failure]

VERSION=${1:-$(date +%Y%m%d-%H%M%S)}
ROLLBACK_ON_FAILURE=${2:-true}
DEPLOY_DIR="/opt/deployments/kchng"
RELEASE_DIR="$DEPLOY_DIR/releases/kchng-$VERSION"
CURRENT_LINK="$DEPLOY_DIR/current"
BACKUP_LINK="$DEPLOY_DIR/previous"
ROLLBACK_LOG="$DEPLOY_DIR/shared/rollback-logs/deploy.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
  echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$ROLLBACK_LOG"
}

error() {
  echo -e "${RED}[ERROR]${NC} $1" | tee -a "$ROLLBACK_LOG"
}

warn() {
  echo -e "${YELLOW}[WARN]${NC} $1" | tee -a "$ROLLBACK_LOG"
}

rollback() {
  error "Deployment failed! Rolling back..."

  if [[ -L "$BACKUP_LINK" ]]; then
    PREV_VERSION=$(readlink "$BACKUP_LINK")

    # Stop current
    pm2 stop kchng-frontend || true

    # Switch to previous
    rm -f "$CURRENT_LINK"
    ln -s "$PREV_VERSION" "$CURRENT_LINK"

    # Restart
    pm2 reload kchng-frontend --update-env

    log "Rolled back to: $PREV_VERSION"
    exit 1
  else
    error "No previous version to rollback to!"
    exit 1
  fi
}

# Check if version exists
if [[ -d "$RELEASE_DIR" ]]; then
  error "Release $VERSION already exists!"
  exit 1
fi

log "=== Starting KCHNG Deployment v$VERSION ==="

# 1. Clone/Pull code
log "Cloning repository..."
GIT_REPO="${KCHNG_REPO:-git@github.com:pokho/kchng.git}"
GIT_BRANCH="${KCHNG_BRANCH:-main}"

git clone --branch "$GIT_BRANCH" --single-branch "$GIT_REPO" "$RELEASE_DIR" || {
  error "Failed to clone repository"
  exit 1
}

cd "$RELEASE_DIR"

# 2. Install dependencies
log "Installing dependencies..."
npm ci || {
  error "Failed to install dependencies"
  exit 1
}

# 3. Build
log "Building application..."
npm run build || {
  error "Failed to build"
  exit 1
}

# 4. Pre-deployment health check
log "Running health check..."
if pm2 describe kchng-frontend > /dev/null 2>&1; then
  bash "$DEPLOY_DIR/scripts/health-check.sh" || {
    warn "Pre-deployment health check failed, but continuing..."
  }
fi

# 5. Backup current release
if [[ -L "$CURRENT_LINK" ]]; then
  CURRENT_VER=$(readlink "$CURRENT_LINK" | xargs basename)
  log "Backing up current release: $CURRENT_VER"
  rm -f "$BACKUP_LINK"
  ln -s "$CURRENT_LINK" "$BACKUP_LINK"
fi

# 6. Switch to new release
log "Switching to new release..."
rm -f "$CURRENT_LINK"
ln -s "$RELEASE_DIR" "$CURRENT_LINK"

# 7. Start/Reload PM2
log "Starting PM2 process..."
if pm2 describe kchng-frontend > /dev/null 2>&1; then
  pm2 reload kchng-frontend --update-env || {
    error "Failed to reload PM2"
    if [[ "$ROLLBACK_ON_FAILURE" == "true" ]]; then
      rollback
    fi
    exit 1
  }
else
  pm2 start ecosystem.config.cjs --env production || {
    error "Failed to start PM2"
    if [[ "$ROLLBACK_ON_FAILURE" == "true" ]]; then
      rollback
    fi
    exit 1
  }
fi

# 8. Save PM2 config
pm2 save || warn "Failed to save PM2 config"

# 9. Post-deployment health check
sleep 5
log "Running post-deployment health check..."
if ! bash "$DEPLOY_DIR/scripts/health-check.sh"; then
  error "Post-deployment health check failed!"
  if [[ "$ROLLBACK_ON_FAILURE" == "true" ]]; then
    rollback
  fi
  exit 1
fi

# 10. Cleanup old releases (keep last 3)
log "Cleaning up old releases..."
cd "$DEPLOY_DIR/releases"
ls -t | tail -n +4 | xargs -I {} rm -rf {}

log "=== Deployment Successful! ==="
log "Version: $VERSION"
log "URL: https://kachi.ng (once DNS is configured)"

# Show PM2 status
pm2 show kchng-frontend
```

#### 3. Rollback Script (`/opt/deployments/kchng/scripts/rollback.sh`)

```bash
#!/bin/bash
set -e

# KCHNG Rollback Script
# Usage: ./rollback.sh [version]

VERSION=${1:-auto}
DEPLOY_DIR="/opt/deployments/kchng"
CURRENT_LINK="$DEPLOY_DIR/current"
BACKUP_LINK="$DEPLOY_DIR/previous"
ROLLBACK_LOG="$DEPLOY_DIR/shared/rollback-logs/rollback.log"

log() {
  echo -e "[$(date +'%Y-%m-%d %H:%M:%S')] $1" | tee -a "$ROLLBACK_LOG"
}

error() {
  echo "[ERROR] $1" | tee -a "$ROLLBACK_LOG"
  exit 1
}

# If version not specified, use backup
if [[ "$VERSION" == "auto" ]]; then
  if [[ ! -L "$BACKUP_LINK" ]]; then
    error "No previous version to rollback to!"
  fi
  TARGET_LINK="$BACKUP_LINK"
else
  TARGET_DIR="$DEPLOY_DIR/releases/kchng-$VERSION"
  if [[ ! -d "$TARGET_DIR" ]]; then
    error "Version $VERSION not found!"
  fi
fi

log "=== Starting KCHNG Rollback ==="

# Stop current
log "Stopping current release..."
pm2 stop kchng-frontend || true

# Switch
if [[ "$VERSION" == "auto" ]]; then
  # Swap current and backup
  TEMP_LINK=$(mktemp -d)
  mv "$CURRENT_LINK" "$TEMP_LINK/current-backup"
  mv "$BACKUP_LINK" "$CURRENT_LINK"
  mv "$TEMP_LINK/current-backup" "$BACKUP_LINK"
else
  # Move current to backup, then point to target
  rm -f "$BACKUP_LINK"
  mv "$CURRENT_LINK" "$BACKUP_LINK" 2>/dev/null || true
  ln -s "$TARGET_DIR" "$CURRENT_LINK"
fi

# Restart
log "Restarting PM2..."
pm2 reload kchng-frontend --update-env || {
  error "Failed to restart PM2"
}

# Save PM2 config
pm2 save

log "=== Rollback Successful! ==="
pm2 show kchng-frontend
```

#### 4. Health Check Script (`/opt/deployments/kchng/scripts/health-check.sh`)

```bash
#!/bin/bash

# KCHNG Health Check Script
# Checks if the application is responding correctly

DEPLOY_DIR="/opt/deployments/kchng"
PORT=5173
HEALTH_URL="http://localhost:$PORT"
TIMEOUT=10

log() {
  echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

error() {
  echo "[ERROR] $1"
  exit 1
}

# Check if port is listening
log "Checking if port $PORT is listening..."
if ! nc -z localhost $PORT; then
  error "Port $PORT is not accessible!"
fi

# Check HTTP response
log "Checking HTTP response..."
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" --max-time $TIMEOUT "$HEALTH_URL" || echo "000")

if [[ "$HTTP_CODE" == "000" ]]; then
  error "Cannot connect to application!"
elif [[ "$HTTP_CODE" == "200" ]]; then
  log "✓ Application is healthy (HTTP 200)"
elif [[ "$HTTP_CODE" =~ "5[0-9]{2}" ]]; then
  error "Application error (HTTP $HTTP_CODE)"
else
  log "⚠ Application returned HTTP $HTTP_CODE (continuing)"
fi

# Check for specific content
log "Checking page content..."
CONTENT=$(curl -s --max-time $TIMEOUT "$HEALTH_URL")
if echo "$CONTENT" | grep -q "KCHNG"; then
  log "✓ Page contains 'KCHNG'"
else
  error "Page does not contain expected content!"
fi

log "✓ All health checks passed!"
```

---

## Nginx Configuration

### Create `/etc/nginx/sites-available/kachi`

```nginx
server {
    server_name kachi.ng;

    # Client max body size
    client_max_body_size 10M;

    # Logging
    access_log /var/log/nginx/kchng-access.log;
    error_log /var/log/nginx/kchng-error.log;

    # Reverse proxy to KCHNG app
    location / {
        proxy_pass http://localhost:5173;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;

        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }

    # Static files caching
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        proxy_pass http://localhost:5173;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

### Enable the site:

```bash
sudo ln -s /etc/nginx/sites-available/kachi /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## SSL Certificate Setup

```bash
# Install certbot if not present
sudo apt update
sudo apt install -y certbot python3-certbot-nginx

# Obtain certificate for kachi.ng
sudo certbot --nginx -d kachi.ng -d www.kachi.ng
```

---

## Deployment Steps

### Initial Setup (One-time)

```bash
# 1. Create deployment directory
sudo mkdir -p /opt/deployments/kchng/{releases,scripts,logs,shared/rollback-logs}
sudo chown -R deployman:deployman /opt/deployments/kchng

# 2. Copy deployment scripts
scp scripts/*.sh deployman@102.68.84.79:/opt/deployments/kchng/scripts/

# 3. Make scripts executable
ssh deployman@102.68.84.79 "chmod +x /opt/deployments/kchng/scripts/*.sh"

# 4. Setup nginx
ssh deployman@102.68.84.79 "sudo tee /etc/nginx/sites-available/kachi < /tmp/kachi.nginx.conf"
ssh deployman@102.68.84.79 "sudo ln -s /etc/nginx/sites-available/kachi /etc/nginx/sites-enabled/"
ssh deployman@102.68.84.79 "sudo nginx -t && sudo systemctl reload nginx"

# 5. Install PM2 module (if not present)
ssh deployman@102.68.84.79 "pm2 install pm2-logrotate"
```

### Deploy KCHNG

```bash
# From local machine, deploy to server
ssh deployman@102.68.84.79 "cd /opt/deployments/kchng && ./scripts/deploy.sh v1.0.0"
```

### Rollback (if needed)

```bash
# Automatic rollback to previous version
ssh deployman@102.68.84.79 "cd /opt/deployments/kchng && ./scripts/rollback.sh"

# Rollback to specific version
ssh deployman@102.68.84.79 "cd /opt/deployments/kchng && ./scripts/rollback.sh v1.0.0"
```

---

## Monitoring and Maintenance

### View Logs

```bash
# PM2 logs
pm2 logs kchng-frontend

# Application logs
tail -f /opt/deployments/kchng/logs/out.log
tail -f /opt/deployments/kchng/logs/error.log

# Nginx logs
tail -f /var/log/nginx/kchng-access.log
tail -f /var/log/nginx/kchng-error.log

# Rollback logs
tail -f /opt/deployments/kchng/shared/rollback-logs/deploy.log
```

### PM2 Commands

```bash
# Status
pm2 status
pm2 show kchng-frontend

# Restart
pm2 restart kchng-frontend

# Reload (zero-downtime)
pm2 reload kchng-frontend

# Stop
pm2 stop kchng-frontend

# Start
pm2 start kchng-frontend

# Monitor
pm2 monit
```

---

## Rollback Decision Tree

```
Issue Detected
       │
       ▼
    Can you fix it?
       │
   ┌───┴───┐
   │      │
  Yes    No
   │      │
  │      ▼
  │   Is it critical?
  │   │
  │   ├─Yes─→ Rollback immediately
  │   │
   │   └─No───→ Monitor for 30 min
   │            │
   │         ┌───┴───┐
   │         │       │
   │      Fixed    Worsening
   │         │       │
   │         │       ▼
   │         │   Rollback
   │         │
   ▼         ▼
Deploy fix    Deploy fix
```

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| **Deployment fails** | Automatic rollback on failure |
| **New version has bugs** | One-command rollback to previous |
| **DNS not configured** | Can use IP:port for testing |
| **Port conflict** | Using port 5173 (different from Padawa) |
| **Resource exhaustion** | Low resource usage, plenty of headroom |
| **PM2 crashes** | `autorestart: true` configured |

---

## Summary

**Recommended Approach**: PM2-based deployment with rollback scripts

**Advantages:**
- ✅ Consistent with existing Padawa deployment
- ✅ Minimal learning curve (same commands)
- ✅ Atomic deployments with symlink switching
- ✅ One-command rollback
- ✅ Automated health checks
- ✅ Comprehensive logging

**Deployment Time**: ~5 minutes
**Rollback Time**: ~30 seconds
**DNS Downtime**: Zero (nginx stays up)

---

*Prepared for: deployman@102.68.84.79*
*Date: 2025-01-01*
