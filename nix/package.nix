{
  lib,
  pkg-config,
  rustPlatform,
  pkgs,
  ...
}:
rustPlatform.buildRustPackage {
  pname = "batlert";
  version = "1.0.0";

  src = ../.;

  cargoHash = "sha256-W8vx7ovHaxjo9/Gn6w8fOCMAxbuXXdHQ872phIrZ4Y0=";

  nativeBuildInputs = [
    pkg-config
    rustPlatform.bindgenHook
  ];

  buildInputs = with pkgs; [
    glib.dev
    gtk4.dev
    gtk4-layer-shell.dev
    pango.dev
    gobject-introspection.dev
    cairo.dev
    gdk-pixbuf
    alsa-lib.dev
  ];

  meta = with lib; {
    description = "A GTK popup for linux, to indicate critical battery level";
    homepage = "https://github.com/AtleSkaanes/batlert";
    license = licenses.mit;
  };
}
