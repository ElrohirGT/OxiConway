{
  description = "Flake para el curso de gráficas por computador";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    forAllSystems = {
      pkgs ? nixpkgs,
      function,
    }:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "x86_64-macos"
        "aarch64-linux"
        "aarch64-darwin"
      ]
      (system:
        function {
          pkgs = import pkgs {
            inherit system;
            config.allowUnfree = true;
            overlays = [
              rust-overlay.overlays.default
            ];
          };
          inherit system;
        });
  in {
    packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

    devShells = forAllSystems {
      function = {
        system,
        pkgs,
      }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.stable.latest.default
            openssl
            pkg-config
          ];

          shellHook = ''
            alias gs="git status"
            alias e="exit"
          '';
        };
      };
    };
  };
}
