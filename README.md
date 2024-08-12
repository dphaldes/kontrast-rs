# kontrast-rs

This is a proof-of-concept Rust version of KDE's [Kontrast](https://invent.kde.org/accessibility/kontrast). This is
using the [cxx-kde-frameworks](https://github.com/mystchonky/cxx-kde-frameworks) and [cxx-qt](https://github.com/KDAB/cx-qt) to show how a Rust-enabled Kirigami application could look like.

## Building

Make sure to clone cxx-kde-frameworks in the parent directory, and have the development libraries for Kirigami installed. Then it's as simple as `cargo run`.