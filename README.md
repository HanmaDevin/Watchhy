# Watchhy

This program should keep track of everything you are watching while it is
running. For this it utilizes `playerctl`.
It keeps everything in sync via git.

## Dependencies

### Fedora

```bash
sudo dnf install gtk4-devel libadwaita-devel meson desktop-file-utils gcc glib-compile-resources gtk4-update-icon-cache update-desktop-database
```

### Debian

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev meson desktop-file-utils gcc gtk-update-icon-cache
```

### Arch

```bash
sudo pacman -S gtk4 libadwaita meson desktop-file-utils gcc
```
