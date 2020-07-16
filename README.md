<br />
<p align="center">
  <a href="https://github.com/divy-work/deno_sass">
    <img src="./assets/deno_sass.png" alt="deno_sass logo" width="310">
  </a>
  <h3 align="center">deno_sass</h3>

  <p align="center">
    High quality bindings to the sass_rs crate for Deno.
 </p>
 <p align="center">

  [![stars](https://img.shields.io/github/stars/divy-work/deno_sass)](https://github.com/divy-work/deno_sass/stargazers)
  [![issues](https://img.shields.io/github/issues/divy-work/deno_sass)](https://github.com/divy-work/deno_sass/issues)
  ![deno version](https://img.shields.io/badge/deno-1.0.5-success)
  [![vr scripts](https://badges.velociraptor.run/flat.svg)](https://velociraptor.run)
 
 </p>
</p>

> 🌀 The library is not yet available on Windows. Expect breaking changes.


## Example

```typescript
import { compile } from "https://x.nest.land/sass@0.2.0/mod.ts";

compile("a { color: #000; }", {
  output_style: "nested",
  precision: 5,
  indented_syntax: false,
  include_paths: []
}).result;
```

#### Join Discord

[![](https://discordapp.com/api/guilds/715564894904123424/widget.png?style=banner2)](https://discord.gg/uqywa4W)

## Building from source

### Prerequisites

- [deno](https://deno.land/)
- [rust](https://www.rust-lang.org/)

## Building
```bash
$ cargo build
```

## Example

```bash
$ deno run --unstable -A examples/compile.ts
```

### Contribution

Pull request, issues and feedback are very welcome. Code style is formatted with `deno fmt` and commit messages are done following [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) spec.

## Copyright

deno_sass is licensed under the MIT license. Please see the [LICENSE](LICENSE) file.
