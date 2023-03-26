{pkgs, ...}: {
  devShells.default = with pkgs;
    mkShell {
      packages = [
        python3
        poetry
      ];
      # LD_LIBRARY_PATH = lib.makeLibraryPath [
      #   stdenv.cc.cc
      # ];
    };
}
