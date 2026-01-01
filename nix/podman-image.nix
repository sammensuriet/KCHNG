# Build a Podman container image for KCHNG frontend

{
  kchng-frontend,
  nodejs,
  pkgs,
}:
pkgs.dockerTools.buildLayeredImage {
  name = "kchng-frontend";
  tag = "latest";

  # Contents of the image
  contents = with pkgs; [
    nodejs
    coreutils
    bash
  ];

  # Copy the built application
  extraCommands = ''
    mkdir -p app
    cp -r ${kchng-frontend}/dist/* app/
    chmod -R 755 app
  '';

  # Set up the entrypoint
  config = {
    Cmd = ["node" "/app/index.js"];
    Env = [
      "NODE_ENV=production"
      "PORT=5173"
      "KCHNG_CONTRACT_ID=CAST22E2ZUBSRVFHQ3E6EOZAW33ZA3JM6NDT7RZSUIWP7RH5HC34SOKB"
      "KCHNG_RPC_URL=https://soroban-testnet.stellar.org"
      "KCHNG_NETWORK=testnet"
    ];
    ExposedPorts = {
      "5173/tcp" = {};
    };
    WorkingDir = "/app";
  };

  # Metadata
  meta = with pkgs.lib; {
    description = "KCHNG frontend container image";
  };
}
