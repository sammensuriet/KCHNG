# Nix build for shared package
# Contains types, network configs, and demurrage utilities

{
  pkgs,
  nodejs,
}:
pkgs.buildNpmPackage rec {
  pname = "kchng-shared";
  version = "0.1.0";

  # Source from local path
  src = ../packages/shared;

  # Node.js version
  nodejs = nodejs;

  # pnpm setup
  npmPack = pkgs.writeShellScript "pack-shared" ''
    cd ${src}
    ${pkgs.nodePackages.pnpm}/bin/pnpm install
    ${pkgs.nodePackages.pnpm}/bin/pnpm pack
  '';

  # Build phase
  buildPhase = ''
    echo "Building KCHNG shared package..."
    cp -r ${src}/* .
    ${pkgs.nodePackages.pnpm}/bin/pnpm install --frozen-lockfile
    ${pkgs.nodePackages.pnpm}/bin/pnpm run build
  '';

  # Install phase
  installPhase = ''
    mkdir -p $out/lib
    cp -r dist $out/lib/
    cp package.json $out/
  '';

  # Metadata
  meta = with pkgs.lib; {
    description = "KCHNG shared utilities, types, and network configurations";
    homepage = "https://github.com/pokho/kchng";
    license = licenses.mit;
    platforms = platforms.linux;
  };
}
