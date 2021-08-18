{
  description = "Human-friendly HSL (revision 4)";

  inputs.flake-compat = { url = "github:edolstra/flake-compat"; flake = false; };
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk = { url = "github:nmattia/naersk"; flake = false; };
  inputs.nixpkgs-mozilla = { url = "github:mozilla/nixpkgs-mozilla"; flake = false; };

  outputs = { self, flake-compat, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlay = final: prev: {
        rust-hsluv = final.callPackage ({ lib, naersk }: naersk.buildPackage {
          pname = "hsluv";
          root = ./.;

          meta = {
            description = "Human-friendly HSL (revision 4)";
            license = [ lib.licenses.mit ];
            platforms = lib.platforms.all;
            maintainers = [ lib.maintainers.bb010g ];
          };
        }) { };
      };
      pkgs = import nixpkgs { inherit system; overlays = [
        (final: prev: { cargo-nono = final.callPackage ./nix/cargo-nono.nix { }; })
        (final: prev: { naersk = final.callPackage naersk { }; })
        (final: prev: import (nixpkgs-mozilla + "/rust-overlay.nix") final prev)
        (final: prev: {
          rustChannelStable = final.rustChannelOf {
            channel = "1.54.0";
            sha256 = "sha256-2NfCJiH3wk7sR1XlRf8+IZfY3S9sYKdL8TpMqk82Bq0=";
          };
          rustChannelNightly = final.rustChannelOf {
            channel = "nightly";
            date = "2021-08-18";
            sha256 = "sha256-thDzh6oLG/7RsPOb71yYHsdcyCs0VUL54q25CP6udK8=";
          };
          rustChannel = final.rustChannelStable;
          # rustChannel = final.rustChannelNightly;

          cargo = final.rustChannel.rust;
          rustc = final.rustChannel.rust;
        })
        overlay
      ]; };
    in rec {
      inherit overlay;
      packages.rust-hsluv = pkgs.rust-hsluv;
      defaultPackage = packages.rust-hsluv;

      devShell = pkgs.mkShell {
        inputsFrom = [ pkgs.rust-hsluv ];
        nativeBuildInputs = [
          pkgs.cargo-bloat
          pkgs.cargo-edit
          pkgs.cargo-expand
          pkgs.cargo-nono
          pkgs.rls
        ] ++ pkgs.lib.optional (pkgs.cargo != pkgs.rustc) [
          pkgs.clippy
          pkgs.rustc
          pkgs.rustfmt
        ];
      };
    });
}
