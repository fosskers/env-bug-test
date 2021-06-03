# Debugging the `env` Horror

> Uncaught TypeError: Error resolving module specifier “env”. Relative module
> specifiers must start with “./”, “../” or “/”.

This repo is a minimal example for reproducing the compilation bug.

1. Open `cargo make serve` and `cargo make watch`.
2. Without changing any code, open `localhost:8000` and you should see `"Nothing yet!"`.
3. Uncomment the line at the top of `init` and refresh the page.
4. Notice a blank page and the above error in the browser Console.
5. Uncomment the `parking_lot` line in `Cargo.toml` and revisit the page. Notice no change.
6. Close `cargo make watch` once and do `cargo make build_release`. This employs
   `lto` and other optimizations.
7. You'll see `"Nothing yet!"` on the page, but the WASM panics at runtime with:

> panicked at 'time not implemented on this platform', library/std/src/sys/wasm/../unsupported/time.rs:39:9

8. Uncomment the `chrono` line in `Cargo.toml`. The `env` error is back.

Changing optimization levels, etc., doesn't seem to have an effect.
