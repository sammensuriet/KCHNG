#!/bin/bash
set -e

# KCHNG Rollback Script (Nix + Podman)
# Usage: ./rollback-nix.sh [previous-version-hash]

CONTAINER_NAME="kchng-frontend"
BACKUP_NAME="kchng-frontend-backup"
PREVIOUS_HASH=${1:-auto}

log() {
  echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

error() {
  echo "[ERROR] $1"
  exit 1
}

success() {
  echo "[SUCCESS] $1"
}

log "=== KCHNG Rollback ==="

# Stop current container
log "Stopping current container..."
podman stop $CONTAINER_NAME 2>/dev/null || true
podman rm $CONTAINER_NAME 2>/dev/null || true

# Check if we have a backup container
if podman ps -a --format "{{.Names}}" | grep -q "^$BACKUP_NAME$"; then
  log "Restoring from backup container..."

  # Remove current if it still exists
  if podman ps -a --format "{{.Names}}" | grep -q "^$CONTAINER_NAME$"; then
    podman rm -f $CONTAINER_NAME
  fi

  # Restore backup
  podman rename $BACKUP_NAME $CONTAINER_NAME
  podman start $CONTAINER_NAME

  # Regenerate systemd service
  if [ -f /etc/systemd/system/kchng.service ]; then
    log "Regenerating systemd service..."
    podman generate systemd --name $CONTAINER_NAME | \
      sudo tee /etc/systemd/system/kchng.service > /dev/null
    sudo systemctl daemon-reload
    sudo systemctl enable kchng.service
  fi

  success "Rollback complete!"
else
  # No backup container, try Nix store rollback
  log "No backup container found, attempting Nix store rollback..."

  if [ "$PREVIOUS_HASH" == "auto" ]; then
    # List previous generations
    log "Available Nix generations:"
    nix-store --query --roots /nix/var/nix/profiles/per-user/$USER/profile \
      | grep "kchng" || error "No previous KCHNG builds found"

    log "Please specify a previous store path to rollback to:"
    echo "  ./rollback-nix.sh /nix/store/xxxxx-kchng-frontend-0.1.0"
    exit 1
  else
    # Rollback to specific Nix store path
    log "Rolling back to $PREVIOUS_HASH..."

    # Rebuild with previous store path
    nix build --override-input kchng-frontend $PREVIOUS_HASH

    # Redeploy
    ./scripts/deploy-nix.sh
  fi
fi

# Health check
log "Running health check..."
sleep 5
if bash scripts/health-check.sh; then
  success "Rollback successful! Application is healthy."
else
  error "Rollback completed but health check failed!"
fi

log "Container status:"
podman ps --filter "name=$CONTAINER_NAME"
