{pkgs, ...}: {
  devShells.default = with pkgs;
    mkShell {
      packages = [
        cargo
        rustc
        rustfmt
        rust-analyzer-unwrapped
      ];
      RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
    };
}
