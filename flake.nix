{
  description = "Bevy WASM game development environment";

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, /* rust-overlay, */ ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  # https://devenv.sh/reference/options/
                  packages = [ 
                    pkgs.trunk
                    pkgs.binaryen
                    pkgs.nodePackages.http-server
                    pkgs.pkg-config
                    pkgs.alsaLib
                    pkgs.udev
                    pkgs.xorg.libX11
                    pkgs.xorg.libXcursor
                    pkgs.xorg.libXi
                    pkgs.xorg.libXrandr
                    pkgs.libGL
                    pkgs.vulkan-loader
                    pkgs.vulkan-headers
                    pkgs.python312Packages.httpserver
                 ];

                  languages.rust.enable = true;
                  languages.rust.channel = "nightly";
                  languages.rust.targets = [ "wasm32-unknown-unknown" ];

                  env.LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
                    pkgs.alsaLib
                    pkgs.udev
                    pkgs.vulkan-loader
                  ]}";

                  enterShell = ''
                    cargo version
                  '';
                }
              ];
            };
          });
    };
}
