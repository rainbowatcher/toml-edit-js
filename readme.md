[![NPM Version](https://img.shields.io/npm/v/@rainbowatcher/toml-edit-js)](https://www.npmjs.com/package/@rainbowatcher/toml-edit-js)
![GitHub License](https://img.shields.io/github/license/rainbowatcher/toml-edit-js)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/rainbowatcher/toml-edit-js/ci.yml)
![NPM Downloads](https://img.shields.io/npm/dm/%40rainbowatcher%2Ftoml-edit-js)

# Toml edit for JavaScript

Bring [`toml-edit`](https://github.com/toml-rs/toml) to the JavaScript world via WebAssembly.

## Usage

```sh
npm install @rainbowatcher/toml-edit-js
```

```js
import init, {
    edit, initSync, parse, stringify,
} from "@rainbowatcher/toml-edit-js"

const toml = `
[package]
rand = "1"

[profile.release]
strip = "symbols"
lto = true
codegen-units = 1
`

await init()
// or initSync()
const parsed = parse(toml)
/*
the parsed will be a js object as follow
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
the edited will be a string as follow

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

more example see [tests](https://github.com/rainbowatcher/toml-edit-js/tree/main/tests)

## API

```ts
function parse(input: string): any
function stringify(input: any): string
function edit(input: string, path: string, value: any, option?: IEditOptions): string
```

### Options

edit method can receive a options

```ts
type IEditOptions = {
    /**
     * whether add the final newline
     * @default true
     */
    finalNewline?: boolean

    /**
     * write data in InlineTable format when the value to be written is a object type and inline is set to true
     * @default true
     */
    inline?: boolean
}
```

stringify method can receive a options

```ts
type IStringifyOptions = {
    /**
     * whether add the final newline
     */
    finalNewline?: boolean

    /**
     * prefer inline table style
     * @default false
     */
    inline?: boolean
}
```

# License

[MIT](https://github.com/rainbowatcher/toml-edit-js/blob/main/LICENSE).
