#!/bin/bash
##############################################################################
# KCHNG Frontend Deployment Script
# File: deploy.sh
#
# This script deploys the KCHNG frontend to production.
#
# Usage:
#   ./deploy.sh [environment]
#
# Environments:
#   production  - Deploy to production server (default)
#
# Configuration:
#   Set your server details in the CONFIG section below
##############################################################################

set -euo pipefail

# ============================================================================
# CONFIGURATION - Update these values for your environment
# ============================================================================

# Production server - UPDATE THESE VALUES
PROD_HOST=""  # e.g., "your-server.com" or IP address
PROD_USER="root"
PROD_DEPLOY_DIR="/var/www/kchng.org"
PROD_SSH_PORT="22"

# ============================================================================
# SCRIPT SETTINGS - Usually no need to change these
# ============================================================================

PROJECT_NAME="kchng.org"
LOCAL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FRONTEND_DIR="$LOCAL_DIR/packages/frontend"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# LOGGING FUNCTIONS
# ============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

# ============================================================================
# FUNCTIONS
# ============================================================================

show_usage() {
    echo "Usage: $0 [environment]"
    echo ""
    echo "Environments:"
    echo "  production  - Deploy to production server (default)"
    echo ""
    echo "Configuration required:"
    echo "  Update PROD_HOST in the script with your server details"
    echo ""
    echo "Examples:"
    echo "  $0              # Deploy to production"
    echo "  $0 production   # Deploy to production"
}

# Validate local environment
validate_local() {
    log_info "Validating local environment..."

    # Check if we're in the project root
    if [ ! -f "$LOCAL_DIR/package.json" ]; then
        log_error "package.json not found. Are you in the project root?"
        log_info "Current directory: $LOCAL_DIR"
        exit 1
    fi

    # Check if frontend exists
    if [ ! -d "$FRONTEND_DIR" ]; then
        log_error "Frontend directory not found: $FRONTEND_DIR"
        exit 1
    fi

    # Check if build exists
    if [ ! -d "$FRONTEND_DIR/build" ]; then
        log_error "Build output not found. Run 'pnpm --filter frontend build' first."
        exit 1
    fi

    log_success "Local environment validated"
}

# Deploy static files to server
deploy_static_site() {
    local host="$1"
    local user="$2"
    local port="$3"
    local deploy_dir="$4"

    log_info "Deploying static files to ${user}@${host}:${deploy_dir}..."

    # Create remote directory if it doesn't exist
    log_info "Creating remote directory structure..."
    ssh -p "$port" "${user}@${host}" "mkdir -p $deploy_dir"

    # Copy build output using rsync
    log_info "Copying build files..."
    rsync -avz --delete \
        -e "ssh -p $port" \
        --exclude '.DS_Store' \
        --exclude 'node_modules' \
        "$FRONTEND_DIR/build/" \
        "${user}@${host}:${deploy_dir}/"

    log_success "Files deployed"
}

# Configure web server (nginx)
configure_nginx() {
    local host="$1"
    local user="$2"
    local port="$3"
    local deploy_dir="$4"

    log_info "Configuring nginx..."

    ssh -p "$port" "${user}@${host}" << 'ENDSSH'
# Create nginx config if it doesn't exist
cat > /etc/nginx/sites-available/kchng.org << 'EOF'
server {
    listen 80;
    listen [::]:80;
    server_name kchng.org www.kchng.org;

    root /var/www/kchng.org;
    index index.html;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "no-referrer-when-downgrade" always;

    # PWA support
    location / {
        try_files $uri $uri/ $uri.html /index.html =404;
    }

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Service worker
    location /service-worker.js {
        expires 1h;
        add_header Cache-Control "public, must-revalidate";
    }
}
EOF

# Enable site
if [ ! -L /etc/nginx/sites-enabled/kchng.org ]; then
    ln -s /etc/nginx/sites-available/kchng.org /etc/nginx/sites-enabled/kchng.org
fi

# Test nginx configuration
nginx -t

# Reload nginx
systemctl reload nginx
ENDSSH

    log_success "Nginx configured"
}

# Health check
health_check() {
    local host="$1"
    local user="$2"
    local port="$3"

    log_info "Running health check..."

    if command -v curl &>/dev/null; then
        local response
        response=$(curl -s -o /dev/null -w "%{http_code}" --max-time 10 "https://kchng.org" 2>/dev/null || echo "000")

        if [ "$response" = "200" ] || [ "$response" = "304" ]; then
            log_success "Health check passed (HTTP $response)"
            return 0
        else
            log_warning "Health check returned HTTP $response"
            log_info "Check the site manually at https://kchng.org"
        fi
    else
        log_warning "curl not available. Skipping health check."
    fi
}

# Deploy to production
deploy_production() {
    if [ -z "$PROD_HOST" ]; then
        log_error "Production server not configured."
        echo ""
        echo "Please update the CONFIG section at the top of this script:"
        echo "  PROD_HOST=\"your-server.com\""
        echo "  PROD_USER=\"root\"  # or your deploy user"
        echo "  PROD_DEPLOY_DIR=\"/var/www/kchng.org\""
        echo ""
        echo "Then run the deploy script again."
        exit 1
    fi

    log_info "Deploying to PRODUCTION..."
    echo "  Host: $PROD_HOST"
    echo "  Directory: $PROD_DEPLOY_DIR"
    echo ""

    validate_local
    deploy_static_site "$PROD_HOST" "$PROD_USER" "$PROD_SSH_PORT" "$PROD_DEPLOY_DIR"
    configure_nginx "$PROD_HOST" "$PROD_USER" "$PROD_SSH_PORT" "$PROD_DEPLOY_DIR"
    health_check "$PROD_HOST" "$PROD_USER" "$PROD_SSH_PORT"

    echo ""
    log_success "Production deployment completed!"
    echo ""
    echo "Visit: https://kchng.org"
}

# ============================================================================
# MAIN
# ============================================================================

main() {
    local environment="${1:-production}"

    case "$environment" in
        production|prod)
            deploy_production
            ;;
        help|--help|-h)
            show_usage
            exit 0
            ;;
        *)
            log_error "Unknown environment: $environment"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

main "$@"
