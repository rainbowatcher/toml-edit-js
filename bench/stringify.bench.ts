import fs from "node:fs/promises"
import { beforeAll } from "vitest";
import { bench, describe } from "vitest";
import v2init, { stringify as v2stringify } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { stringify as v3stringify } from "@rainbowatcher/toml-edit-js@v0.3"
import curr4init, { stringify } from "../packages/toml-edit-js/shims"
import { options } from "./benchOptions";


let toml = {
    "name": "@rainbowatcher/toml-edit-js",
    "type": "module",
    "version": "0.3.0",
    "description": "Edit TOML files in JavaScript",
    "author": "rainbowatcher <rainobw-w@qq.com>",
    "license": "MIT",
    "homepage": "https://github.com/rainbowatcher/toml-edit-js#readme",
    "repository": "https://github.com/rainbowatcher/toml-edit-js",
    "bugs": "https://github.com/rainbowatcher/toml-edit-js/issues",
    "keywords": [
        "wasm",
        "toml",
        "edit",
        "javascript"
    ],
    "exports": {
        ".": "./shims.js",
        "./index": "./index.js"
    },
    "types": "shims.d.ts",
    "files": [
        "index.js",
        "index_bg.wasm",
        "index_bg.wasm.d.ts",
        "shims.d.ts",
        "shims.js"
    ]
}

beforeAll(async () => {
    await v2init()
    await v3init()
    await curr4init()
})

describe.skip("small", () => {
    bench("v0.2", () => {
        v2stringify(toml)
    }, options)

    bench("v0.3", () => {
        v3stringify(toml)
    }, options)

    bench("current", () => {
        stringify(toml)
    }, options)
})


describe("middle", () => {
    bench("v0.2", () => {
        v2stringify(toml)
    }, options)

    bench("v0.3", () => {
        v3stringify(toml)
    }, options)

    bench("current", () => {
        stringify(toml)
    }, options)
})