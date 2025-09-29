{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    glib.dev
    gtk4.dev
    gtk4-layer-shell.dev
    pango.dev
    gobject-introspection.dev
    cairo.dev
    gdk-pixbuf
    alsa-lib.dev
  ];
}
