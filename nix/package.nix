{
    fetchFromGitHub,
    pkg-config,
    alsa-lib,
    rustPlatform,
    stdenv,
    pkgs,
    ...
}: rustPlatform.buildRustPackage rec {
  pname = "batlert";
  version = "1.0.0";

  src = fetchFromGitHub {
      # url = "https://github.com/AtleSkaanes/batlert.git";
      # ref = "main";
      owner = "AtleSkaanes";
      repo = pname;
      rev = "v${version}";
      sha256 = "1iga3320mgi7m853la55xip514a3chqsdi1a1rwv25lr9b1p7vd3";
  };

  cargoHash = "";
  cargoLock.lockFile = ../Cargo.lock;

  # nativeBuildInputs =  [
  #   pkg-config
  #   rustPlatform.bindgenHook
  # ];
  # buildInputs =  [
  #   alsa-lib.dev
  # ];

  # meta = with stdenv.lib; {
  #   description = "";
  #   homepage = "https://github.com/AtleSkaanes/batlert";
  #   license = licenses.mit;
  #   maintainers = [ maintainers.tailhook ];
  # };
}
