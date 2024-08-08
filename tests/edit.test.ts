import dedent from "dedent"
import {
    beforeAll, describe, expect, it,
} from "vitest"
import init, { edit } from "../packages/toml-edit-js/index.js"

beforeAll(async () => {
    await init()
})

const input = dedent`
    [foo]
    bar = 1
`
const opt = { finalNewline: false }
describe("edit", () => {
    it("set string", () => {
        expect(edit(input, "foo.bar", "qux", opt)).toBe(dedent`
            [foo]
            bar = "qux"
        `)
    })

    it("set string with truthy option", () => {
        expect(edit(input, "foo.bar", "qux", { finalNewline: true })).toBe(dedent`
            [foo]
            bar = "qux"\n
        `)
    })

    it("set number", () => {
        expect(edit(input, "foo.bar", 2, opt)).toBe(dedent`
            [foo]
            bar = 2
        `)

        expect(edit(input, "foo.bar", 2.2, opt)).toBe(dedent`
            [foo]
            bar = 2.2
        `)

        expect(edit(input, "foo.bar", -2.2, opt)).toBe(dedent`
            [foo]
            bar = -2.2
        `)

        expect(edit(input, "foo.bar", Number.NaN, opt)).toBe(dedent`
            [foo]
            bar = nan
        `)

        expect(edit(input, "foo.bar", Infinity, opt)).toBe(dedent`
            [foo]
            bar = inf
        `)

        expect(edit(input, "foo.bar", 10e3, opt)).toBe(dedent`
            [foo]
            bar = 10000
        `)

        expect(edit(input, "foo.bar", -0.1e5, opt)).toBe(dedent`
            [foo]
            bar = -10000
        `)
    })

    it("bitint", () => {
        expect(() => edit(input, "foo.bar", 15_033_211_231_241_234_523_452_345_345_787n, opt))
            .toThrowErrorMatchingInlineSnapshot("[Error: bigint is not supported]")
    })

    it("null or undefined", () => {
        expect(() => edit(input, "foo.bar", null, opt)).toThrowErrorMatchingInlineSnapshot("[Error: null and undefined is not supported]")
    })

    it("boolean", () => {
        const input = "[foo]\nbar = true"
        expect(edit(input, "foo.bar", false, opt)).toBe(dedent`
            [foo]
            bar = false
        `)
    })

    it("array", () => {
        expect(edit(input, "foo.bar", [1, 2, 3], opt)).toBe(dedent`
            [foo]
            bar = [1, 2, 3]
        `)
    })

    it("object", () => {
        expect(edit(input, "foo.bar", { a: 1, b: 2 }, opt)).toBe(dedent`
            [foo]
            bar = { a = 1, b = 2 }
        `)
    })

    it("datetime", () => {
        expect(edit(input, "foo.bar", new Date(0), opt)).toBe(dedent`
            [foo]
            bar = {}
        `)
    })

    it("key with dot", () => {
        const input1 = dedent`
            [foo.bar]
            baz = 0
        `
        expect(edit(input1, "foo.bar.baz", 1, opt)).toBe(dedent`
            [foo.bar]
            baz = 1
        `)
    })
})

describe("error", () => {
    it("with unknown field", () => {
        // @ts-expect-error type error
        expect(() => edit(input, "foo.bar", 1, { unknown: "true" })).toThrowErrorMatchingInlineSnapshot("[Error: unknown property [unknown]]")
    })

    it("with invalid type", () => {
        // @ts-expect-error type error
        expect(() => edit(input, "foo.bar", 1, { finalNewline: "true" })).toThrowErrorMatchingInlineSnapshot("[Error: finalNewline should be a boolean]")
    })

    it("with array option", () => {
        // @ts-expect-error type error
        expect(() => edit(input, "foo.bar", 1, ["true"])).toThrowErrorMatchingInlineSnapshot("[Error: IEditOptions can not be an array]")
    })

    it("with string option", () => {
        // @ts-expect-error type error
        expect(() => edit(input, "foo.bar", 1, "true")).toThrowErrorMatchingInlineSnapshot("[Error: IEditOptions should be an object]")
    })
})
