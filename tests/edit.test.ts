import {
    beforeAll, describe, expect, it,
} from "vitest"
import init, { edit } from "../packages/toml-edit-js/index.js"

beforeAll(async () => {
    await init()
})

describe("edit", () => {
    it("set string", () => {
        const input = "[foo]\nbar = 1"
        const result = edit(input, "foo.bar", "qux")
        expect(result).toMatchInlineSnapshot(`
            "[foo]
            bar = "qux"
            "
        `)
    })

    it("set number", () => {
        const input = "[foo]\nbar = 1"
        expect(edit(input, "foo.bar", 2)).toMatchInlineSnapshot(`
            "[foo]
            bar = 2
            "
        `)

        expect(edit(input, "foo.bar", 2.2)).toMatchInlineSnapshot(`
            "[foo]
            bar = 2.2
            "
        `)

        expect(edit(input, "foo.bar", -2.2)).toMatchInlineSnapshot(`
            "[foo]
            bar = -2.2
            "
        `)

        expect(edit(input, "foo.bar", Number.NaN)).toMatchInlineSnapshot(`
            "[foo]
            bar = nan
            "
        `)

        expect(edit(input, "foo.bar", Infinity)).toMatchInlineSnapshot(`
            "[foo]
            bar = inf
            "
        `)

        expect(edit(input, "foo.bar", 10e3)).toMatchInlineSnapshot(`
            "[foo]
            bar = 10000
            "
        `)

        expect(edit(input, "foo.bar", -0.1e5)).toMatchInlineSnapshot(`
            "[foo]
            bar = -10000
            "
        `)
    })

    it("bitint", () => {
        const input = "[foo]\nbar = 1"
        expect(() => edit(input, "foo.bar", 15_033_211_231_241_234_523_452_345_345_787n))
            .toThrowErrorMatchingInlineSnapshot("[Error: bigint is not supported]")
    })

    it("null or undefined", () => {
        const input = "[foo]\nbar = 1"
        expect(() => edit(input, "foo.bar", null)).toThrowErrorMatchingInlineSnapshot("[Error: null and undefined is not supported]")
    })

    it("boolean", () => {
        const input = "[foo]\nbar = true"
        expect(edit(input, "foo.bar", false)).toMatchInlineSnapshot(`
            "[foo]
            bar = false
            "
        `)
    })

    it("array", () => {
        const input = "[foo]\nbar = 1"
        expect(edit(input, "foo.bar", [1, 2, 3])).toMatchInlineSnapshot(`
            "[foo]
            bar = [1, 2, 3]
            "
        `)
    })

    it("object", () => {
        const input = "[foo]\nbar = 1"
        expect(edit(input, "foo.bar", { a: 1, b: 2 })).toMatchInlineSnapshot(`
            "[foo]
            bar = { a = 1, b = 2 }
            "
        `)
    })

    it("datetime", () => {
        const input = "[foo]\nbar = 1"
        expect(edit(input, "foo.bar", new Date(0))).toMatchInlineSnapshot(`
            "[foo]
            bar = {}
            "
        `)
    })
})
