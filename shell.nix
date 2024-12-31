# I don't want to cook my system's dependencies
# 
{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell {
  buildInputs = [
    rustup
    raylib
    cmake
    clang
    pkg-config
    wayland
    glfw
    libGL
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
  ];

  # Env variabels
  shellHook = ''
      export LD_LIBRARY_PATH="${glfw}/lib:${freetype}/lib:${wayland}/lib:${raylib}/lib"
      export LIBCLANG_PATH="${libclang.lib}/lib"
      export CMAKE_PREFIX_PATH="${glfw}/lib/cmake/glfw3"
      export CMAKE_CXX_COMPILER="/run/current-system/sw/bin/c++"
      export CMAKE_C_COMPILER="/run/current-system/sw/bin/cc"
      export RL="${raylib}"
  '';
}