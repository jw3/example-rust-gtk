gtk, dbus, rust
===

### gtk
- https://www.gtk.org/docs/installations/
- https://gtk-rs.org/#using
- https://blog.gtk.org/

### relm
- https://github.com/antoyo/relm

### glade
- https://gtk-rs.org/docs-src/tutorial/glade
- https://snapcraft.io/glade

### dbus
- https://www.freedesktop.org/software/systemd/man/org.freedesktop.systemd1.html
- https://developer.gnome.org/gio/stable/gdbus-convenience.html

### rpm
- https://github.com/rpm-software-management/librpm.rs

### dev dependencies
- libgtk-3-dev (3.22.x)
  - ubuntu 18.04 - `libgtk-3-dev` (3.22.30)
  - centos 7 - `gtk3-devel` (3.22.30)
- librpm (4.14)
- clang (6.0)

### runtime dependencies
- gtk
- librpm

### ref
- https://github.com/rust-lang/rust-bindgen/issues/242#issuecomment-275229404
- https://github.com/rpm-software-management/librpm.rs/pull/7#issuecomment-569369225
- https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories
- https://developer.gnome.org/dbus-glib/unstable/
- https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a
- https://medium.com/dwelo-r-d/wrapping-unsafe-c-libraries-in-rust-d75aeb283c65
