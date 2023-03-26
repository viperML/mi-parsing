{pkgs, ...}: {
  devShells.default = pkgs.mkShell rec {
    myPython = pkgs.python3.withPackages (p:
      with p; [
        jupyter
        openai
      ]);
    packages = [
      myPython
    ];
    LD_LIBRARY_PATH = with pkgs;
      lib.makeLibraryPath [
        stdenv.cc.cc
      ];
    shellHook = ''
      venv="$(cd $(dirname $(which python)); cd ..; pwd)"
      ln -Tsf "$venv" .venv
    '';
  };
}
