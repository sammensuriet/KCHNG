# KCHNG Nix Deployment Guide

This directory contains the complete Nix + Podman deployment configuration for KCHNG.

## Quick Start

### Prerequisites (One-time Setup)

```bash
# 1. Install Nix
curl -L https://nixos.org/nix/install | sh
source ~/.nix-profile/etc/profile.d/nix.sh

# 2. Enable flakes (required for this project)
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf

# 3. Install Podman (on Ubuntu/Debian)
sudo apt update
sudo apt install -y podman podman-compose

# 4. Enter the Nix development shell
nix develop
```

### Local Development

```bash
# Enter dev shell with all tools
nix develop

# Install dependencies
pnpm install

# Build shared package
pnpm --filter shared build

# Start frontend dev server
pnpm --filter frontend dev

# Build for production
pnpm --filter frontend build
```

### Production Deployment

```bash
# Deploy to VPS
./scripts/deploy-nix.sh

# Deploy specific version
./scripts/deploy-nix.sh v1.0.0

# Rollback if needed
./scripts/rollback-nix.sh
```

## File Structure

```
KCHNG/
├── flake.nix                    # Main Nix flake configuration
├── Containerfile                # Podman container definition
├── podman-compose.yml           # Container orchestration
├── nix/
│   ├── shared.nix              # Shared package build
│   ├── kchng-frontend.nix      # Frontend build
│   └── podman-image.nix        # Container image build
├── scripts/
│   ├── deploy-nix.sh           # Deployment script
│   ├── rollback-nix.sh         # Rollback script
│   └── health-check.sh         # Health check script
└── packages/
    ├── shared/                 # Shared utilities
    │   ├── src/
    │   │   ├── demurrage.ts
    │   │   └── networks.ts
    │   └── package.json
    └── frontend/               # SvelteKit frontend
        ├── src/
        │   ├── routes/
        │   ├── lib/
        │   │   ├── components/
        │   │   ├── stores/
        │   │   └── contracts/
        │   └── app.html
        └── package.json
```

## Nix Commands

### Building

```bash
# Build default package (kchng-frontend)
nix build

# Build specific package
nix build .#kchng-frontend
nix build .#shared-pkg
nix build .#podman-image

# Build for specific system
nix build .#packages.x86_64-linux.kchng-frontend
```

### Development

```bash
# Enter development shell
nix develop

# Run command in dev shell
nix develop --command pnpm install

# Run command without entering shell
nix develop --command bash -c "pnpm --filter frontend build"
```

### Deployment

```bash
# Build and deploy
nix build .#podman-image
podman-compose up -d

# Check container status
podman ps

# View logs
podman logs kchng-frontend

# Stop container
podman-compose down
```

## Multi-App Deployment (Future)

When deploying GbamGbam, add it to `podman-compose.yml`:

```yaml
services:
  kchng-frontend:
    # ... existing config ...

  gbamgbam-frontend:
    build: ../gbamgbam
    container_name: gbamgbam-frontend
    ports:
      - "5174:5174"
    restart: unless-stopped
```

Each app can use a different Node.js version in its Nix package:

```nix
# In gbamgbam flake.nix
packages.gbamgbam = pkgs.callPackage ./gbamgbam.nix {
  nodejs = pkgs.nodejs_18;  # Different version!
};
```

## Troubleshooting

### Nix build fails

```bash
# Clear Nix cache and retry
nix-collect-garbage -d
nix build --refresh
```

### Container won't start

```bash
# Check logs
podman logs kchng-frontend

# Check container status
podman ps -a

# Restart container
podman restart kchng-frontend
```

### Health check fails

```bash
# Manually run health check
./scripts/health-check.sh

# Check if port is accessible
curl http://localhost:5173
```

## Advantages of Nix + Podman

1. **Reproducible Builds** - Same dependencies every time
2. **Version Isolation** - Each app can use different Node versions
3. **Easy Rollbacks** - Nix store + Podman containers
4. **No Daemon** - Podman is daemonless (more secure)
5. **Declarative** - Everything in code, no manual steps

## Next Steps

- [ ] Configure DNS for `kachi.ng` → `102.68.84.79`
- [ ] Setup nginx reverse proxy
- [ ] Obtain SSL certificate with certbot
- [ ] Deploy to production VPS
- [ ] Add GbamGbam app when ready
