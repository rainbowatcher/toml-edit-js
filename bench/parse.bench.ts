import fs from "node:fs/promises"
import v2init, { parse as v2parse } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { parse as v3parse } from "@rainbowatcher/toml-edit-js@v0.3"
import { parse as smolParse } from "smol-toml"
import { beforeAll, bench, describe } from "vitest"
import { options } from "./benchOptions"
import curr4init, { parse } from "../packages/toml-edit-js/shims"


let toml = ""
beforeAll(async () => {
    await v2init()
    await v3init()
    await curr4init()
    toml = await fs.readFile(`${process.cwd()}/bench/fixture/pyproject.toml`, "utf8")
})


describe("middle", () => {
    bench("v0.2", () => {
        v2parse(toml)
    }, options)

    bench("v0.3", () => {
        v3parse(toml)
    }, options)

    bench("smol-toml", () => {
        smolParse(toml)
    })

    bench("current", () => {
        parse(toml)
    }, options)

})
