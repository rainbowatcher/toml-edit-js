import fs from "node:fs/promises"
import v2init, { parse as v2parse } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { parse as v3parse } from "@rainbowatcher/toml-edit-js@v0.3"
import v4init, { parse as v4parse } from "@rainbowatcher/toml-edit-js@v0.4"
import v5init, { parse as v5parse } from "@rainbowatcher/toml-edit-js@v0.5"
import { parse as smolParse } from "smol-toml"
import { bench } from "vitest"
import init, { parse } from "../packages/toml-edit-js/shims"

await v2init()
await v3init()
await v4init()
await v5init()
await init()
const toml = await fs.readFile(`${process.cwd()}/bench/fixture/pyproject.toml`, "utf8")


bench("v0.2", () => {
    v2parse(toml)
})

bench("v0.3", () => {
    v3parse(toml)
})

bench("v0.4", () => {
    v4parse(toml)
})

bench("v0.5", () => {
    v5parse(toml)
})

bench("smol-toml", () => {
    smolParse(toml)
})

bench("current", () => {
    parse(toml)
})
