{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, flake-utils }:
    {
      overlays.default = final: prev: with final; {
        todomine = rustPlatform.buildRustPackage rec {
          name = "todomine";
          src = self;
          nativeBuildInputs = [
            pkg-config
          ];
          buildInputs = [ openssl ];
          cargoHash = "sha256-/sUt7QLZT5iBzWZAFz0tUZiFjm6o5uAck8x+yP3sL28=";
        };
      };
      overlay = self.overlays.default;
    }
    //
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ self.overlay ]; };
        deps = with pkgs; [
          rustc
          rustfmt
          cargo
          cargo-outdated
          rust-analyzer
          pkg-config
          openssl
          taplo-cli
          nodejs_latest
          nodePackages_latest.typescript-language-server
          python3
        ];
        env = ''
          export NIXPKGS_ALLOW_UNFREE=1
        '';
      in
      with pkgs; rec {
        packages = rec {
          inherit (pkgs) todomine;
          devShell = mkShell {
            buildInputs = deps;
            shellHook = ''
              ${env}
              tmux-ui() {
                PROJECT=$(basename $(pwd))
                tmux at -t $PROJECT || tmux new -s $PROJECT -n term \; \
                  splitw -v -p 50 \; \
                  neww -n tig \; send "tig" C-m \; \
                  neww -n nvim \; send "nvim" C-m \; \
                  selectw -t 1\; selectp -t 1 \;
              }
            '';
          };
        };
        defaultPackage = packages.devShell;
      }
    );
}
