# KCHNG Deployment Proposal for padawa.ng VPS

**Server**: 102.68.84.79 (Ubuntu 24.04.3 LTS)
**Purpose**: Deploy KCHNG frontend alongside existing Padawa PWA
**Domain**: kchng.org
**Date**: 2025-01-01

---

## ✅ Deployment Status

**KCHNG is LIVE at https://kchng.org**

- ✅ Frontend built and deployed
- ✅ nginx configured
- ✅ SSL certificate installed (Let's Encrypt)
- ✅ HTTP to HTTPS redirect active

---

## Current Infrastructure Overview

### Running Services
| Service | Type | Port | Management | Path |
|---------|------|------|------------|------|
| **Padawa PWA** | SvelteKit SSR | 3000 | PM2 cluster (2 instances) | `/opt/deployments/padawa` |
| **OKBackend** | Node/TS | 9000 | PM2 fork | `/opt/okbackend` |
| **KCHNG** | Static Site | 443 | nginx | `/opt/deployments/kchng` |
| **nginx** | Reverse Proxy | 80/443 | systemd | `/etc/nginx/` |

### Domains Configured
- `padawa.ng` → localhost:3000
- `api.padawa.ng` → localhost:3000
- `www.padawa.ng` → localhost:3000
- `kchng.org` → static files via nginx ✅

### System Resources
- **RAM**: 7.7GB (6.9GB available)
- **Disk**: 193GB (188GB available, 3% used)
- **Load**: 0.08 (very low)

---

## Deployment Details

### How KCHNG Was Deployed

KCHNG was built as a **static site** (using `@sveltejs/adapter-static`) and served directly by nginx. This is the simplest and most efficient approach for the current frontend.

### Build Process

```bash
# On VPS
cd /opt/deployments/kchng
npx pnpm --filter shared build
npx pnpm --filter frontend build
```

### Deployment Location

```
/opt/deployments/kchng/
├── packages/
│   ├── frontend/
│   │   └── build/          # Static files served by nginx
│   └── shared/
│       └── dist/           # Shared utilities
```

### nginx Configuration

```nginx
server {
    server_name kchng.org www.kchng.org;
    root /opt/deployments/kchng/packages/frontend/build;
    index index.html;

    # SPA routing
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|json)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
```

---

## Future Deployments

### Recommended Tool: okdeployman

For deploying KCHNG and future apps (GbamGbam, etc.), use **okdeployman**:

```bash
# Install okdeployman
git clone https://github.com/pokho/okdeployman.git ~/okdeployman
nix profile install ~/okdeployman#default

# Deploy KCHNG updates
deploy-static kchng.org deployman@102.68.84.79 ./packages/frontend/build
```

**okdeployman features:**
- ✅ Remote deployment from local machine
- ✅ SSH-based (no manual login required)
- ✅ Health checks after deployment
- ✅ Supports multiple servers
- ✅ Reusable across projects

### Deployment Options

| Method | When to Use | Command |
|--------|-------------|---------|
| **okdeployman** | Regular updates | `deploy-static kchng.org ...` |
| **Manual SSH** | Quick fixes | `ssh deployman@102.68.84.79 "cd /opt/deployments/kchng && pnpm build"` |
| **deploy-rs** | Complex deployments | `nix run .#deploy` |

---

## Updating KCHNG

### Quick Update

```bash
# From your local machine
cd /home/pokho/dev/KCHNG
pnpm --filter frontend build
rsync -avz ./packages/frontend/build/ deployman@102.68.84.79:/opt/deployments/kchng/packages/frontend/build/
```

### Using okdeployman (Recommended)

```bash
# Build and deploy in one command
deploy-static kchng.org deployman@102.68.84.79 ./packages/frontend/build
```

---

## Rollback Strategy

If something goes wrong:

### 1. Quick File Rollback

```bash
# Restore previous build from backup
ssh deployman@102.68.84.79
cd /opt/deployments/kchng
mv packages/frontend/build packages/frontend/build.failed
mv packages/frontend/build.backup packages/frontend/build
```

### 2. Git Rollback

```bash
# Checkout previous version
git checkout <previous-commit>
pnpm --filter frontend build
deploy-static kchng.org ...
```

### 3. Nix Store Rollback (if using Nix builds)

```bash
nix-store --rollback
nix build .#kchng-frontend
```

---

## Monitoring

### Check Site Status

```bash
# HTTP check
curl -I https://kchng.org/

# Health check
curl https://kchng.org/ | grep -i "kchng"

# nginx logs
ssh deployman@102.68.84.79 "tail -f /var/log/nginx/kachi-access.log"
```

### SSL Certificate

```bash
# Check expiry
ssh deployman@102.68.84.79 "sudo certbot certificates"

# Renew manually
ssh deployman@102.68.84.79 "sudo certbot renew"

# Auto-renews via cron (already configured)
```

---

## Adding GbamGbam

When deploying GbamGbam to the same VPS:

### Option 1: Static Site (Recommended if GbamGbam is static)

```bash
# Build GbamGbam locally
cd /home/pokho/dev/gbamgbam
pnpm build

# Deploy with okdeployman
deploy-static gbamgbam.ng deployman@102.68.84.79 ./build
```

### Option 2: Node.js App (if it needs server-side logic)

```bash
deploy-node gbamgbam deployman@102.68.84.79 ./gbamgbam
```

### nginx Configuration

Add a new server block for GbamGbam:

```nginx
server {
    server_name gbamgbam.ng www.gbamgbam.ng;
    root /opt/deployments/gbamgbam/build;
    # ... same config as kchng.org
}
```

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| **Deployment fails** | Automatic rollback in okdeployman |
| **SSL expires** | Auto-renewal via certbot |
| **High traffic** | nginx handles static files efficiently |
| **Disk space** | Plentiful (188GB available) |
| **Memory** | Static site uses minimal memory |

---

## Summary

**Current Status:**
- ✅ KCHNG live at https://kchng.org
- ✅ SSL configured
- ✅ nginx serving static files
- ✅ Auto-renewing SSL certificate

**For Future Updates:**
- Use okdeployman for remote deployment
- Or manual rsync for quick updates
- Or deploy-rs for complex multi-server setups

**Key URLs:**
- Site: https://kchng.org
- Deploy tool: https://github.com/pokho/okdeployman
- Documentation: See okdeployman README

---

*Last updated: 2025-01-01*
*Prepared for: deployman@102.68.84.79*
