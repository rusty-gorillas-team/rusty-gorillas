# Rusty Gorillas

_Educational project_ - modernised Gorillas game written in Rust.

The main target of the project is to get the participants to know Rust better, while working together on fun project.

## How to build

Please have a closer look at [SDL2 setup](https://github.com/Rust-SDL2/rust-sdl2) in order to setup SDL2 dependency on your platform. This is just one time discomfort to continue with awesome project ;)

The project is designed to run with the latest **stable Rust**.

## Contribution guidelines

In order to keep the code style uniform, we use `rustfmt` and `clippy`. You can install them like:
- `rustup component add rustfmt-preview`
- `rustup component add clippy-preview`

The work on the project is organised via [GitHub Issues](https://github.com/rusty-gorillas-team/rusty-gorillas/issues).

Please create a separate branch for each issue.

Once ready:
- please run `cargo fmt` and `cargo clippy` and resolve any formal issues it might discover
- create Pull request and ask other team member for code review
