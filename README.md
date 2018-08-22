# node-rs
This crate exposes a `stdweb`-friendly interface to the `node.js` API. This is
very much a work-in-progress.

## API Design
There are some guiding principles we're following in the design of the API.

First, when translating names, Rust will use Rust-centric casing for types and
functions. That means types will be `PascalType`, which in general matches
the style for types in JavaScript. Function names will be `snake_case`, which
is different from typical JavaScript naming convention.

For example, `setTimeout` becomes `set_timeout`.

Second, namespacing will follow these typical patterns:
- `global` names will fall directly under the `node_rs` crate top-level module
  name
- "module" names will fall under a Rust module of the same name. e.g.
  `require('cluster').fork()` in JavaScript becomes `cluster::fork()` in Rust

Third, built-in modules will not need to be "imported" like they are in
JavaScript. `node-rs` will assume that built-in modules can be imported, and so
all module routines can just be called directly without first importing the
module. In order to support runtime feature detection (e.g. for modules
introduced in newer node.js releases), each module will have an `initialize()`
function that can be used to check to see if the module is available.

For example:

```rust
use node_rs::cluster;

if (let Some(_) = cluster::initialize()) {
    // 'cluster' module is available
    cluster::fork();
}
```

Finally, all global values will be accessible via function calls. For example,
`global.process` is available as `node_rs::process`.