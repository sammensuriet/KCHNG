#!/bin/bash
set -e

# KCHNG Remote Deployment Script
# Deploys KCHNG to VPS from local machine
# Usage: ./deploy-remote.sh

VPS_HOST="deployman@102.68.84.79"
DEPLOY_DIR="/opt/deployments/kchng"
LOCAL_DIR="/home/pokho/dev/KCHNG"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
  echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

success() {
  echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
  echo -e "${YELLOW}[WARN]${NC} $1"
}

log "=== KCHNG Remote Deployment ==="

# 1. Build locally (optional - can also build on VPS)
log "Building frontend locally..."
cd "$LOCAL_DIR"
pnpm --filter shared build
pnpm --filter frontend build

# 2. Sync source files to VPS
log "Syncing source files to VPS..."
rsync -avz --delete \
  --exclude='node_modules' \
  --exclude='.git' \
  --exclude='dist' \
  --exclude='build' \
  --exclude='.svelte-kit' \
  --exclude='target' \
  --exclude='.pnpm-store' \
  "$LOCAL_DIR/" \
  "$VPS_HOST:$DEPLOY_DIR/"

# 3. Build on VPS
log "Building on VPS..."
ssh "$VPS_HOST" "cd $DEPLOY_DIR && npx pnpm --filter shared build && npx pnpm --filter frontend build"

# 4. Reload nginx
log "Reloading nginx..."
ssh "$VPS_HOST" "sudo nginx -t && sudo systemctl reload nginx"

# 5. Health check
log "Running health check..."
sleep 2
if curl -sf https://kachi.ng/ > /dev/null; then
  success "Health check passed!"
else
  warn "Health check failed - please verify manually"
fi

success "=== Deployment Complete! ==="
log "Site: https://kachi.ng"
