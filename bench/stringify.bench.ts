import v2init, { stringify as v2stringify } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { stringify as v3stringify } from "@rainbowatcher/toml-edit-js@v0.3"
import { stringify as smolStringify } from "smol-toml"
import { bench } from "vitest"
import curr4init, { stringify } from "../packages/toml-edit-js/shims"


const toml = {
    author: "rainbowatcher <rainobw-w@qq.com>",
    bugs: "https://github.com/rainbowatcher/toml-edit-js/issues",
    description: "Edit TOML files in JavaScript",
    exports: {
        ".": "./shims.js",
        "./index": "./index.js",
    },
    files: [
        "index.js",
        "index_bg.wasm",
        "index_bg.wasm.d.ts",
        "shims.d.ts",
        "shims.js",
    ],
    homepage: "https://github.com/rainbowatcher/toml-edit-js#readme",
    keywords: [
        "wasm",
        "toml",
        "edit",
        "javascript",
    ],
    license: "MIT",
    name: "@rainbowatcher/toml-edit-js",
    repository: "https://github.com/rainbowatcher/toml-edit-js",
    type: "module",
    types: "shims.d.ts",
    version: "0.3.0",
}


await v2init()
await v3init()
await curr4init()


bench("v0.2", () => {
    v2stringify(toml)
})

bench("v0.3", () => {
    v3stringify(toml)
})

bench("smol-toml", () => {
    smolStringify(toml)
})

bench("current", () => {
    stringify(toml)
})
