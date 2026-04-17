{
  description = "A TUI to make copying terminal history from tmux easier";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "aarch64-darwin" "x86_64-linux" "aarch64-linux" ];
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          lib = pkgs.lib;
        in
        {
          tmux-snaglord = pkgs.rustPlatform.buildRustPackage {
            pname = "tmux-snaglord";
            version = self.shortRev or self.dirtyShortRev or "dev";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = [ pkgs.pkg-config ];

            # `arboard` needs X11 libs at link time on Linux. On modern nixpkgs
            # darwin, AppKit is provided implicitly via the stdenv apple-sdk.
            buildInputs = lib.optionals pkgs.stdenv.isLinux [
              pkgs.xorg.libxcb
            ];

            meta = {
              description = "TUI for snagging commands and output from tmux scrollback";
              homepage = "https://github.com/raine/tmux-snaglord";
              license = lib.licenses.mit;
              mainProgram = "tmux-snaglord";
            };
          };
          default = self.packages.${system}.tmux-snaglord;
        }
      );
    };
}
