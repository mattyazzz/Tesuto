{
  description = "Leaf-Chat dev environment";
  # con este flake.nix tienes el entorno de desarrollo para leaf-chat, obvio para
  # usar este flake necesitas nixOs (la mejor distro btw), en caso de usar otra distro o OS
  # puedes instalar manualmente los pkgs que estan aqui abajo usando pacman, apt, dnf, etc.

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          pkg-config
          fontconfig
          wayland
          libxkbcommon
        ];

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.fontconfig
          pkgs.wayland
          pkgs.libxkbcommon
        ];
      # (cuando sera el dia q no tenga problema con las rutas en nix? ToT)
      shellHook = ''
        export SLINT_BACKEND=winit-software
      '';
      # maldita laptop con gpu de la era del caldo
      };
    };
}