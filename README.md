# Experimental

A fully experimental and starter template (blueprint layout) for gtk development with rust.

## Meson Build
```sh
$ meson --prefix=$(pwd) --datadir=build/src build && meson compile -C build
$ ./buld/src/experimental
```

## Rust debug build (after meson build)
```sh
$ cargo run
```

## Flatpak build
```sh
$ flatpak-builder flatpak-build-dir com.firstApp.experimental.json --force-clean --user --install
```
## Flatpak build sharable sigle-file bundle
```sh
$ flatpak build-bundle ~/.local/share/flatpak/repo experimental.flatpak com.firstApp.experimental --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo
```