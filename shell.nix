if builtins ? getFlake then
  { system ? builtins.currentSystem }:
  (let result = builtins.getFlake (toString ./.); in rec {
    defaultNix = result // (if result ? defaultPackage.${system} then { default = result.defaultPackage.${system}; } else { });
    shellNix = result // (if result ? devShell.${system} then { default = result.devShell.${system}; } else { });
  }).shellNix
else
  (import (let
    lock = builtins.fromJSON (builtins.readFile ./flake.lock);
  in builtins.fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/${lock.nodes.flake-compat.locked.rev}.tar.gz";
    sha256 = lock.nodes.flake-compat.locked.narHash;
  }) { src =  ./.; }).shellNix
