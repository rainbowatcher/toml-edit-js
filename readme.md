[![NPM Version](https://img.shields.io/npm/v/@rainbowatcher/toml-edit-js)](https://www.npmjs.com/package/@rainbowatcher/toml-edit-js)
![GitHub License](https://img.shields.io/github/license/rainbowatcher/toml-edit-js)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/rainbowatcher/toml-edit-js/ci.yml)
![NPM Downloads](https://img.shields.io/npm/dm/%40rainbowatcher%2Ftoml-edit-js)

# TOML Edit for JavaScript

This library brings the power of Rust's `toml_edit` to the JavaScript ecosystem through WebAssembly. It allows you to edit TOML files while preserving all comments, spacing, and formatting. It also provides `parse` and `stringify` functions for standard TOML handling.

## Features

- **Format-Preserving Editing**: Modify specific values in your TOML data while keeping your TOML file's formatting, comments, and whitespace intact.
- **Standard TOML Parsing**: Parses TOML text into a JavaScript object.
- **Standard TOML Stringifying**: Converts a JavaScript object back into a TOML string.
- **Synchronous & Asynchronous API**: Supports both sync and async initialization for flexible integration.
- **High Performance**: Built with Rust and WebAssembly for great performance. Check out the [benchmarks](https://github.com/rainbowatcher/toml-edit-js/tree/main/bench).

## Installation

```sh
npm install @rainbowatcher/toml-edit-js
```

## Usage

First, you need to initialize the WebAssembly module. You can do this either asynchronously or synchronously.

```javascript
import init, { edit, initSync, parse, stringify } from "@rainbowatcher/toml-edit-js"

// Asynchronous initialization
await init()

// Or synchronous initialization (e.g., in a CommonJS environment)
// initSync();
```

### Parse a TOML String

```javascript
const tomlString = `
# Main package configuration
[package]
name = "my-app"
version = "0.1.0" # Initial version

# Release profile settings
[profile.release]
strip = "symbols"
lto = true
codegen-units = 1
`

const parsed = parse(tomlString)

console.log(parsed)
/*
Output:
{
    "package": {
        "name": "my-app",
        "version": "0.1.0"
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
```

### Edit a TOML String

The `edit` function allows you to change a value at a specific path. The original formatting and comments are preserved.

```javascript
const originalToml = `
# Main package configuration
[package]
name = "my-app"
version = "0.1.0" # Initial version

# Release profile settings
[profile.release]
strip = "symbols"
lto = true
codegen-units = 1
`

const updatedToml = edit(originalToml, "package.rand", { version: "1.0" })

console.log(updatedToml)
/*
Output:
# Main package configuration
[package]
name = "my-app"
version = "1.0" # Initial version

# Release profile settings
[profile.release]
strip = "symbols"
lto = true
codegen-units = 1
*/
```

### Stringify a JavaScript Object

You can convert a JavaScript object back into a TOML string.

```javascript
const data = {
  database: {
    ip: "192.168.1.1",
    ports: [8001, 8002],
  },
}

const tomlStr = stringify(data)

console.log(tomlStr)
/*
Output:
[database]
ip = "192.168.1.1"
ports = [8001, 8002]
*/
```

For more examples, please see the [tests](https://github.com/rainbowatcher/toml-edit-js/tree/main/tests).

## API

### `init(): Promise<void>`

Asynchronously initializes the WebAssembly module.

### `initSync(): void`

Synchronously initializes the WebAssembly module.

### `parse(input: string): any`

Parses a TOML string into a JavaScript object.

### `stringify(input: any, opts?: IStringifyOptions | null): string`

Stringifies a JavaScript object into a TOML string.

### `edit(input: string, path: string, value: any, opts?: IEditOptions | null): string`

Edits a TOML string at a given path with a new value, preserving formatting.

## Options

### `IEditOptions`

Options for the `edit` function.

```ts
type IEditOptions = {
  /**
   * Whether to add a final newline to the output.
   * @default true
   */
  finalNewline?: boolean

  /**
   * When the value to be written is an object, write it as an inline table.
   * @default true
   */
  inline?: boolean
}
```

### `IStringifyOptions`

Options for the `stringify` function.

```ts
type IStringifyOptions = {
  /**
   * Whether to add a final newline to the output.
   * @default true
   */
  finalNewline?: boolean

  /**
   * Prefer using inline tables for all tables.
   * @default false
   */
  inline?: boolean
}
```

## License

This project is licensed under the [MIT License](https://github.com/rainbowatcher/toml-edit-js/blob/main/LICENSE).
