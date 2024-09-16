# Experimental

A fully experimental and starter template (blueprint layout) for gtk development with rust.

## Meson Build (optimized release build)
```sh
$ meson --prefix=$(pwd) --datadir=build/src --buildtype=release build && meson compile -C build
$ ./buld/src/experimental
```
This will generate a **build** folder where all release binaries should be available.

## Rust debug build (after meson build)
```sh
$ cargo run
```
This will generate **target** folder where all debug binaries (unoptimized) should be available.

## Flatpak build and install (user level)
> Remember to commit (git) your changes before making a build like this.
```sh
$ flatpak-builder flatpak-build-dir com.firstApp.experimental.json --force-clean --user --install
```
This will generate **.flatpak-builder** and **flatpak-build-dir** folder where all binaries related to flatpak should be available.

>You should be finding your App in your application launcher by now or you could just simply run this command and launch the App.
```sh
$ flatpak run com.firstApp.experimental
```
## Flatpak build a sharable single file bundle
```sh
$ flatpak build-bundle ~/.local/share/flatpak/repo experimental.flatpak com.firstApp.experimental --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo
```

### Use [blueprint-compiler](https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/setup.html) to convert any ui to blueprint files

In your project’s directory, run blueprint-compiler port (or <path to blueprint-compiler.py> port) to start the porting process. It will walk you through the steps outlined below. It should work for most projects, but if something goes wrong you may need to follow the manual steps instead.

> Clone [blueprint-compiler](https://gitlab.gnome.org/jwestman/blueprint-compiler) from source.

```sh
$ ~/Downloads/blueprint-compiler/blueprint-compiler.py port
```

### Usefull links and resources
- https://discourse.gnome.org/t/build-and-run-project-outside-of-builder/12229/4
- https://developer.gnome.org/documentation/tutorials/beginners/getting_started.html
- https://mesonbuild.com/Project-templates.html
- https://mesonbuild.com/Builtin-options.html
- https://www.reddit.com/r/flatpak/comments/1bje34z/trying_to_buildbundle_but_cant_find_valid/
