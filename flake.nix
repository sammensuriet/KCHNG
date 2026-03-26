{
  description = "KCHNG - Community Currency with Demurrage";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }: let
    # Shared configuration for all apps in the monorepo
    mkApp = {
      name,
      package,
      port,
      contractId,
    }: {
      type = "app";
      program = "${package}/bin/start";
      metadata = {
        inherit port contractId;
      };
    };
  in
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };

      # Node.js version for KCHNG frontend
      nodejs = pkgs.nodejs_20;

      # pnpm version
      pnpm = pkgs.nodePackages.pnpm;

      # Shared package - contains types, networks config, demurrage utilities
      shared-pkg = pkgs.callPackage ./nix/shared.nix {
        inherit pkgs nodejs;
      };

      # KCHNG frontend package
      kchng-frontend = pkgs.callPackage ./nix/kchng-frontend.nix {
        inherit pkgs nodejs pnpm shared-pkg;
      };

      # Development shell with all tools
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Node.js ecosystem
          nodejs
          pnpm

          # Rust is managed via rustup (not from nixpkgs)
          # Install via: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
          # Then: rustup target add wasm32-unknown-unknown

          # Stellar CLI tools
          soroban-env-cli
          wasm-bindgen-cli

          # Container tools
          podman
          podman-compose

          # Deployment tools
          openssh
        ];

        shellHook = ''
          echo "🦊 KCHNG Development Environment"
          echo "=================================="
          echo "Node.js: $(node --version)"
          echo "pnpm: $(pnpm --version)"
          echo ""
          echo "Available commands:"
          echo "  pnpm install          - Install dependencies"
          echo "  pnpm --filter shared build   - Build shared package"
          echo "  pnpm --filter frontend dev    - Start frontend dev server"
          echo "  pnpm --filter frontend build  - Build frontend for production"
          echo ""
          echo "Nix commands:"
          echo "  nix build .#kchng-frontend     - Build KCHNG frontend"
          echo "  nix develop                    - Enter dev shell"
          echo ""
          echo "Deployment:"
          echo "  See deployment documentation in docs/"
        '';
      };

      # Podman image
      podman-image = pkgs.callPackage ./nix/podman-image.nix {
        inherit kchng-frontend nodejs;
      };
    in {
      # Packages
      packages = {
        inherit
          shared-pkg
          kchng-frontend
          podman-image
          devShell
          ;

        # Default package
        default = kchng-frontend;
      };

      # Development shell
      devShells.default = devShell;

      # Apps for deployment
      apps = {
        kchng = mkApp {
          name = "kchng";
          package = kchng-frontend;
          port = 5173;
          contractId = "CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB";
        };
      };
    });
}
