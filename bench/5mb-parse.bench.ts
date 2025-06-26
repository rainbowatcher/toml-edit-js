import fs from "node:fs/promises"
import v2init, { parse as v2parse } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { parse as v3parse } from "@rainbowatcher/toml-edit-js@v0.3"
import { parse as smolParse } from "smol-toml"
import { bench } from "vitest"
import curr4init, { parse } from "../packages/toml-edit-js/shims"


await v2init()
await v3init()
await curr4init()
const toml = await fs.readFile(`${process.cwd()}/bench/fixture/5mb-mixed.toml`, "utf8")

bench("v0.2", () => {
    v2parse(toml)
})

bench("v0.3", () => {
    v3parse(toml)
})

bench("smol-toml", () => {
    smolParse(toml)
})

bench("current", () => {
    parse(toml)
})
