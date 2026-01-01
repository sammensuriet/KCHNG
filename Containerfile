# Containerfile for KCHNG Frontend
# Built with Nix, deployed with Podman

FROM localhost/kchng-frontend:latest

# Install runtime dependencies
RUN apk add --no-cache \
    nodejs-20 \
    npm \
    curl

# Create app directory
WORKDIR /app

# Copy built application from Nix build
COPY --from=builder /nix/store/*-kchng-frontend-0.1.0/dist /app

# Expose port
EXPOSE 5173

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:5173 || exit 1

# Run the application
CMD ["node", "index.js"]
