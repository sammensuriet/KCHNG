#!/bin/bash
set -e

# KCHNG Deployment Script (Nix + Podman)
# This script builds the application with Nix and deploys it with Podman
# Usage: ./deploy-nix.sh [version]

VERSION=${1:-latest}
DEPLOY_DIR="/opt/deployments/kchng"
CONTAINER_NAME="kchng-frontend"
BACKUP_NAME="kchng-frontend-backup"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

error() {
  echo -e "${RED}[ERROR]${NC} $1"
}

rollback() {
  error "Deployment failed! Rolling back..."

  # Remove failed container
  podman stop $CONTAINER_NAME 2>/dev/null || true
  podman rm $CONTAINER_NAME 2>/dev/null || true

  # Restore backup
  if podman ps -a --format "{{.Names}}" | grep -q "^$BACKUP_NAME$"; then
    log "Restoring backup container..."
    podman rename $BACKUP_NAME $CONTAINER_NAME
    podman start $CONTAINER_NAME

    # Restore systemd service
    if [ -f /etc/systemd/system/kchng.service ]; then
      log "Restoring systemd service..."
      podman generate systemd --name $CONTAINER_NAME | \
        sudo tee /etc/systemd/system/kchng.service > /dev/null
      sudo systemctl daemon-reload
      sudo systemctl enable kchng.service
    fi

    success "Rollback complete!"
  else
    error "No backup found to restore!"
  fi
  exit 1
}

# Check if Nix is installed
if ! command -v nix &> /dev/null; then
  error "Nix is not installed. Please install Nix first:"
  echo "  curl -L https://nixos.org/nix/install | sh"
  exit 1
fi

# Check if Podman is installed
if ! command -v podman &> /dev/null; then
  error "Podman is not installed. Please install Podman first:"
  echo "  sudo apt install podman podman-compose"
  exit 1
fi

log "=== KCHNG Deployment with Nix + Podman ==="
log "Version: $VERSION"
log "Deployment directory: $DEPLOY_DIR"
echo ""

# 1. Create deployment directory
log "Setting up deployment directory..."
sudo mkdir -p $DEPLOY_DIR
sudo chown -R $USER:$USER $DEPLOY_DIR

# 2. Copy project files to deployment directory
log "Copying project files..."
rsync -av --exclude='node_modules' \
  --exclude='.git' \
  --exclude='dist' \
  --exclude='build' \
  $(pwd)/ $DEPLOY_DIR/

# 3. Build with Nix
log "Building with Nix..."
cd $DEPLOY_DIR
if ! nix build .#kchng-frontend; then
  error "Nix build failed!"
  rollback
fi

# 4. Build Podman image
log "Building Podman image..."
if ! nix build .#podman-image; then
  error "Podman image build failed!"
  rollback
fi

# 5. Backup existing container
if podman ps -a --format "{{.Names}}" | grep -q "^$CONTAINER_NAME$"; then
  log "Backing up existing container..."
  podman stop $CONTAINER_NAME 2>/dev/null || true

  # Remove old backup if exists
  if podman ps -a --format "{{.Names}}" | grep -q "^$BACKUP_NAME$"; then
    podman rm $BACKUP_NAME 2>/dev/null || true
  fi

  podman rename $CONTAINER_NAME $BACKUP_NAME
fi

# 6. Deploy with podman-compose
log "Deploying with Podman Compose..."
cd $DEPLOY_DIR
if ! podman-compose up -d; then
  error "Podman compose failed!"
  rollback
fi

# 7. Generate systemd service
log "Generating systemd service..."
if [ -f /etc/systemd/system/kchng.service ]; then
  sudo systemctl disable kchng.service 2>/dev/null || true
fi

podman generate systemd --name $CONTAINER_NAME --files --new
if [ -f "container-${CONTAINER_NAME}.service" ]; then
  sudo mv "container-${CONTAINER_NAME}.service" /etc/systemd/system/kchng.service
  sudo systemctl daemon-reload
  sudo systemctl enable kchng.service
  success "Systemd service configured"
fi

# 8. Wait for container to be ready
log "Waiting for container to start..."
sleep 10

# 9. Health check
log "Running health check..."
MAX_ATTEMPTS=30
ATTEMPT=1

while [ $ATTEMPT -le $MAX_ATTEMPTS ]; do
  if curl -f -s http://localhost:5173 > /dev/null 2>&1; then
    success "Health check passed!"

    # Check for KCHNG content
    CONTENT=$(curl -s http://localhost:5173)
    if echo "$CONTENT" | grep -q "KCHNG"; then
      success "Page content verified!"
      break
    else
      warn "Page accessible but content check failed"
    fi
  fi

  warn "Health check attempt $ATTEMPT/$MAX_ATTEMPTS failed, retrying..."
  sleep 2
  ATTEMPT=$((ATTEMPT + 1))
done

if [ $ATTEMPT -gt $MAX_ATTEMPTS ]; then
  error "Health check failed after $MAX_ATTEMPTS attempts!"
  rollback
fi

# 10. Cleanup old images (keep last 3)
log "Cleaning up old images..."
podman images kchng-frontend --format "{{.ID}} {{.CreatedAt}}" | \
  tail -n +4 | awk '{print $1}' | \
  xargs -I {} podman rmi -f {} 2>/dev/null || true

# 11. Display status
success "=== Deployment Successful! ==="
log "Container: $CONTAINER_NAME"
log "Port: 5173"
log "URL: https://kachi.ng (once DNS is configured)"
echo ""

log "Container status:"
podman ps --filter "name=$CONTAINER_NAME" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

echo ""
log "Logs (last 20 lines):"
podman logs --tail 20 $CONTAINER_NAME

echo ""
success "Deployment complete! 🚀"
