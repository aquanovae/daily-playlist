{
  description = ''
    Generate daily spotify playlist
  '';


  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    src = {
      url = "github:aquanovae/daily-playlist";
      flake = false;
    };
  };


  outputs = { nixpkgs, ... }@inputs: let

    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

    buildInputs = with pkgs; [
        openssl
    ];
    nativeBuildInputs = with pkgs; [
        cargo
        rustc
        pkg-config
    ];

  in {

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {

      pname = "daily-playlist";
      version = "1.1.2";

      src = inputs.src;

      inherit buildInputs nativeBuildInputs;

      useFetchCargoVendor = true;
      cargoHash = "sha256-LHlxWEjkciedpTa2fSYwR5Z6DSfu/fSUFUov2h4QH2c=";
    };


    devShells.${system}.default = pkgs.stdenv.mkDerivation {

      name = "rust";

      inherit buildInputs nativeBuildInputs;
    };
  };
}
