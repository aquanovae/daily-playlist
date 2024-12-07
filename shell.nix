with import <nixpkgs> {};

stdenv.mkDerivation {

  name = "rust";

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [ 
    cargo
    rustc

    openssl
  ];
}
