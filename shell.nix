# I don't want to cook my system's dependencies
# 
{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell {
  buildInputs = [
    cargo
    rustc
    rustup
    rust-analyzer
    cmake
    glslang
    vulkan-headers
    vulkan-loader
    vulkan-tools
    vulkan-validation-layers
    glfw
    glfw-wayland  
    freetype
    clang
    libclang.lib
    wayland
  ];

  # Env variabels
  shellHook = ''
      export LD_LIBRARY_PATH="${glfw}/lib:${freetype}/lib:${vulkan-loader}/lib:${vulkan-validation-layers}/lib:${wayland}/lib"
      export VULKAN_SDK="${vulkan-headers}"
      export VK_LAYER_PATH="${vulkan-validation-layers}/share/vulkan/explicit_layer.d"
      export LIBCLANG_PATH="${libclang.lib}/lib"
      export CMAKE_PREFIX_PATH="${glfw}/lib/cmake/glfw3"
      export CMAKE_CXX_COMPILER="/run/current-system/sw/bin/c++"
      export CMAKE_C_COMPILER="/run/current-system/sw/bin/cc"
  '';
}