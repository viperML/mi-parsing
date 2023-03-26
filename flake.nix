{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs; } {
    systems = ["aarch64-linux" "x86_64-linux"];

    perSystem.imports = [./perSystem.nix];
  };
}
