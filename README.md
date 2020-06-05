# deno-sass

[![stars](https://img.shields.io/github/stars/divy-work/deno-ciphers)](https://github.com/divy-work/deno-ciphers/stargazers)
[![issues](https://img.shields.io/github/issues/divy-work/deno-ciphers)](https://github.com/divy-work/deno-ciphers/issues)
![deno version](https://img.shields.io/badge/deno-1.0.5-success)
[![vr scripts](https://badges.velociraptor.run/flat.svg)](https://velociraptor.run)

Fast WASM SaSS compiler for Deno.

## Example

```typescript
import { compile } from "mod.ts";
compile("a { color: #000; }");
```

## Building from source

### Prerequisites

- [deno](https://deno.land/)
- [rust](https://www.rust-lang.org/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

## Building
```bash
$ deno run -A scripts/build.ts
```

## Testing

Requires `wasm.js` to be built first.

```bash
$ deno test
```

### Contribution

Pull request, issues and feedback are very welcome. Code style is formatted with `deno fmt` and commit messages are done following [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) spec.

### Licence

Copyright 2020, Divy Srivastava. All rights reserved. MIT license.
