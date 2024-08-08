# Toml edit for JavaScript

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

# License

[MIT](https://github.com/rainbowatcher/cross-release/blob/main/LICENSE).
