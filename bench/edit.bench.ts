import fs from "node:fs/promises"
import v2init, { edit as v2edit } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { edit as v3edit } from "@rainbowatcher/toml-edit-js@v0.3"
import v4init, { edit as v4edit } from "@rainbowatcher/toml-edit-js@v0.4"
import v5init, { edit as v5edit } from "@rainbowatcher/toml-edit-js@v0.5"
import { bench } from "vitest"
import init, { edit } from "../packages/toml-edit-js/shims"


await v2init()
await v3init()
await v4init()
await v5init()
await init()
const toml = await fs.readFile(`${process.cwd()}/bench/fixture/pyproject.toml`, "utf8")


bench("v0.2", () => {
    v2edit(toml, "flake8.ignore", "W500")
})

bench("v0.3", () => {
    v3edit(toml, "flake8.ignore", "W500")
})

bench("v0.4", () => {
    v4edit(toml, "flake8.ignore", "W500")
})

bench("v0.5", () => {
    v5edit(toml, "flake8.ignore", "W500")
})

bench("current", () => {
    edit(toml, "flake8.ignore", "W500")
})
