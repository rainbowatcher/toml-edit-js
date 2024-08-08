# Toml edit for JavaScript

This repo is built on top of the `toml-edit` crate. It brings `toml-edit` to the JavaScript world through WebAssembly.

## Usage

```sh
npm install @rainbowatcher/toml-edit-js
```

```js
import init, { edit, parse, stringify } from "@rainbowatcher/toml-edit-js"

const toml = `
[package]
rand = "1"

[profile.release]
strip = "symbols"
lto = true
codegen-units = 1
`

await init()
const parsed = parse(toml)
/*
the const parsed will be as follow
{
    "package": {
        "rand": "1"
    },
    "profile": {
        "release": {
            "strip": "symbols",
            "lto": true,
            "codegen-units": 1
        }
    }
}
*/

const edited = edit(toml, "package.rand", { version: "1.0" })
/*
the const edited will be as follow

[package]
rand = { version = "1.0" }

[profile.release]
strip = "symbols"
lto = true
codegen-units = 1
*/

const str = stringify(parsed)
/* same as const toml */
```

## Options

edit method can receive a options

```ts
type IEditOptions = {
    finalNewline: boolean
}
```

# License

[MIT](https://github.com/rainbowatcher/toml-edit-js/blob/main/LICENSE).
