{ config, lib, pkgs, ... }:

{
  config = {
    packages = with pkgs; [
      bacon
      sqlite
      openssl
    ];

    dotenv.enable = true;

    languages.rust = {
      enable = true;
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
      channel = "stable";
    };

    git-hooks.hooks = {
      nixpkgs-fmt.enable = true;
      clippy.enable = true;
      rustfmt.enable = true;
    };
  };
}
