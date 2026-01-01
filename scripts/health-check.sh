#!/bin/bash
set -e

# KCHNG Health Check Script
# Checks if the application is responding correctly

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
if ! nc -z localhost $PORT 2>/dev/null; then
  error "Port $PORT is not accessible!"
fi
log "✓ Port $PORT is accessible"

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
