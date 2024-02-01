# File Hash Example - ArrayBuffer

A simple example for calculating a SHA256 File Hash using component `use_drop_zone` (using leptos_use crate) and reading the selected file as buffer array (using `web-sys` crate).

If you don't have it installed already, install [Trunk](https://trunkrs.dev/) and [Tailwind](https://tailwindcss.com/docs/installation)
as well as the nightly toolchain for Rust and the wasm32-unknown-unknown target:

```bash
cargo install trunk
npm install -D tailwindcss @tailwindcss/forms
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

Then, open two terminals. In the first one, run:

```
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

In the second one, run:

```bash
trunk serve --open
```
