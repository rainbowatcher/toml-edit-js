import { stringify as iarnaTomlStringify } from "@iarna/toml"
import { stringify as ltdJTomlStringify } from "@ltd/j-toml"
import v3init, { stringify as v3stringify } from "@rainbowatcher/toml-edit-js@v0.3"
import v4init, { stringify as v4stringify } from "@rainbowatcher/toml-edit-js@v0.4"
import v5init, { stringify as v5stringify } from "@rainbowatcher/toml-edit-js@v0.5"
import { stringify as smolStringify } from "smol-toml"
import { bench } from "vitest"
import init, { stringify } from "../packages/toml-edit-js/shims"

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


await v3init()
await v4init()
await v5init()
await init()


bench("v0.3", () => {
    v3stringify(toml)
})

bench("v0.4", () => {
    v4stringify(toml)
})

bench("v0.5", () => {
    v5stringify(toml)
})

bench("smol-toml", () => {
    smolStringify(toml)
})

bench("@iarna/toml", () => {
    iarnaTomlStringify(toml)
})

bench("@ltd/j-toml", () => {
    ltdJTomlStringify(toml)
})

bench("current", () => {
    stringify(toml)
})
