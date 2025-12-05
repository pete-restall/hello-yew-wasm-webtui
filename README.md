# Hello, Yew WASM WebTUI !

## What is this ?
Just some messing around exploring the state of the Rust / WASM ecosystem.  Incorporates some further messing around
with some awesome styles, which turned out to be harder than expected and a complete time sink because of the way
Yew's `html!` macro parses its JSX-like syntax.

## Links
- MDN WebAssembly specifications and documentation <[https://developer.mozilla.org/en-US/docs/WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)>
- Trunk, a WASM build tool, bundler and development server for Rust <(https://trunkrs.dev/)[https://trunkrs.dev/)>
- Yew, a React-like framework for Rust WASM <[https://yew.rs/](https://yew.rs/)>
- WebTUI, an awesome set of CSS <[https://webtui.ironclad.sh/](https://webtui.ironclad.sh/)>

## Notes to Future Self

### Installing Tools / Prerequisites
```bash
$ rustup target add wasm32-unknown-unknown
$ cargo install -f wasm-bindgen-cli
$ cargo install --locked trunk
```
Then `trunk serve` ought to just work in a regular binary crate with a correctly configured [Cargo.toml](./Cargo.toml)
and bootstrapping HTML file.  The HTML is used for asset discovery and minification as well as a target `<div />` for
embedding the WASM into; [app.html](./app.html) serves the purpose for this project.

### Pains in the Yew-know-whats
It turns out that HTML identifiers in Yew's `html!` macro are expected to be of the form `ident(-ident)*`, where
`ident` is a Rust identifier (HTML compatibility issues right there), but further to that, WebTUI uses attributes
of the form `ident-`; note the trailing dash.  Other frameworks also use identifiers such as `nsident:ident` for
HTML attributes, which will also cause a problem with Yew.  See
[https://github.com/yewstack/yew/discussions/3477](https://github.com/yewstack/yew/discussions/3477) for the
2023 issue on this :(

### Size Comparison
```bash
$ total=0; for f in `ls dist/*`; do total=$((total + $(cat "$f" | wc -c))); done && echo "Total raw : $total"
$ total=0; for f in `ls dist/*`; do total=$((total + $(cat "$f" | gzip | wc -c))); done && echo "Total gz'd: $total"
```
Gives:
```bash
Total raw : 262768
Total gz'd: 89402
```
For comparison, a gzipped React 16 + DOM bundle [apparently](https://gist.github.com/Restuta/cda69e50a853aa64912d)
weighs in around 32KiB.  The awesome [SvelteKit](https://svelte.dev/tutorial/kit/introducing-sveltekit) will be even
less :-D

### Thoughts and Impressions
- WASM seems to kill a little bit of the 'openness' of the web by introducing code as a binary blob, which is more friction for inspection
- Seems promising as it's a W3C standard, but at the same time driven by the usual big US players and their interests, so enshittification is a concern
- WASM still needs JavaScript (and a websocket ?!?) to bootstrap it, although that will change in future
- Even when WASM can be loaded without JavaScript, it won't be a universal technology like HTML that is supported in every browser (think along the lines of the awesome [Lynx](https://lynx.invisible-island.net/) and accessibility in general)
- WASM seems pretty beefy compared to minified JavaScript, probably because a bunch of the Rust runtime also needs to be compiled / downloaded, versus the density of higher-level instructions and the browser-bundled runtime that comes 'for free' with JavaScript
- Using externally minified assets are going to be smaller than the DOM manipulation in Yew, but they're not going to be reactive, unless another framework or JavaScript is used; obviously this defeats the purpose of using Yew
- Care needs to be taken when embedding assets in Rust because four spaces takes three more bytes than one tab... :-/  There are only so many spaces in the world and using them for indentation is profligacy
- Care needs to be taken with Rust code size, eg. by paying attention to generics and inlining, much like when using an embedded target
- It's more friction writing front-end Rust than hacking JavaScript due to the stateful nature of UIs and the DOM
- If using multiple WASM components - perhaps to ease a big application's perceived load-time by providing some initial lightweight functionality - you will end up downloading multiple copies of the Rust (or whatever source language's) runtime as well; the alternative for a big application would be one big download and a slow initial page load for the user, which is also not a good experience
- WASM feels like something more suited to an organisation-internal application as opposed to something you'd write an average website with; SvelteKit with JavaScript still feels like my go-to for SPAs and other JavaScript-enabled (and public) sites, possibly with WASM for select use-cases where it makes sense
- Certainly better than the (often platform-dependent) Java Applets, Flash, Silverlight, ActiveX, etc. attempts that require(d) browser plugins
- Excellent way to share DTOs, validation logic, etc. full-stack
- WebTUI is awesome, except the `-` attribute suffix is janky and jarring, especially next to an `=`; but I can live with that
