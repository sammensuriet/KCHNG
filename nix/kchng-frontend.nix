# Nix build for KCHNG frontend (SvelteKit)

{
  pkgs,
  nodejs,
  pnpm,
  shared-pkg,
}:
pkgs.buildNpmPackage rec {
  pname = "kchng-frontend";
  version = "0.1.0";

  # Source from local path
  src = ../packages/frontend;

  # Shared package dependency
  buildInputs = with pkgs; [nodejs shared-pkg];

  # Environment variables
  NODE_ENV = "production";
  PORT = "5173";
  KCHNG_CONTRACT_ID = "CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB";
  KCHNG_RPC_URL = "https://soroban-testnet.stellar.org";
  KCHNG_NETWORK = "testnet";

  # Build phase
  buildPhase = ''
    echo "Building KCHNG frontend..."
    echo "Node.js version: $(node --version)"
    echo "pnpm version: $(pnpm --version)"

    # Set up shared package symlink
    mkdir -p node_modules
    ln -sf ${shared-pkg}/lib node_modules/@kchng/shared

    # Install dependencies
    pnpm install --frozen-lockfile

    # Build shared package first
    cd ../../shared
    pnpm run build
    cd ../frontend

    # Build frontend
    pnpm run build
  '';

  # Install phase
  installPhase = ''
    mkdir -p $out/bin $out/dist

    # Copy built application
    cp -r build/* $out/dist/

    # Create startup script
    cat > $out/bin/start <<EOF
    #!/bin/sh
    export NODE_ENV=production
    export PORT=5173
    export KCHNG_CONTRACT_ID=${KCHNG_CONTRACT_ID}
    export KCHNG_RPC_URL=${KCHNG_RPC_URL}
    export KCHNG_NETWORK=${KCHNG_NETWORK}
    cd $out/dist
    exec node index.js
    EOF
    chmod +x $out/bin/start
  '';

  # Metadata
  meta = with pkgs.lib; {
    description = "KCHNG Community Currency Frontend";
    homepage = "https://kachi.ng";
    license = licenses.mit;
    platforms = platforms.linux;
  };
}
