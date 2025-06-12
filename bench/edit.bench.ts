import fs from "node:fs/promises"
import { beforeAll } from "vitest";
import { bench, describe } from "vitest";
import v2init, { edit as v2edit } from "@rainbowatcher/toml-edit-js@v0.2"
import v3init, { edit as v3edit } from "@rainbowatcher/toml-edit-js@v0.3"
import init, { edit } from "@rainbowatcher/toml-edit-js@v0.3"
import { options } from "./benchOptions";

let toml = ""

beforeAll(async () => {
    await v2init()
    await v3init()
    await init()
    toml = await fs.readFile(process.cwd() + "/bench/fixture/pyproject.toml", "utf8")
})

describe.skip("small", () => {
    const toml = `
        [foo]
        bar = 1
    `
    
    bench("v0.2", () => {
        v2edit(toml, "foo.bar", 2)
    }, options)

    bench("v0.3", () => {
        v3edit(toml, "foo.bar", 2)
    }, options)

    bench("current", () => {
        edit(toml, "foo.bar", 2)
    }, options)
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