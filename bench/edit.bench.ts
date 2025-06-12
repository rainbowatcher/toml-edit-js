import fs from "node:fs/promises"
import v2init, { edit as v2edit } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { edit as v3edit } from "@rainbowatcher/toml-edit-js@v0.3"
import { beforeAll, bench, describe } from "vitest"
import { options } from "./benchOptions"
import init, { edit } from "../packages/toml-edit-js/shims"

let toml = ""

beforeAll(async () => {
    await v2init()
    await v3init()
    await init()
    toml = await fs.readFile(`${process.cwd()}/bench/fixture/pyproject.toml`, "utf8")
})


describe("middle", () => {

    bench("v0.2", () => {
        v2edit(toml, "flake8.ignore", "W500")
    }, options)

    bench("v0.3", () => {
        v3edit(toml, "flake8.ignore", "W500")
    }, options)

    bench("current", () => {
        edit(toml, "flake8.ignore", "W500")
    }, options)
})
