import dedent from "dedent"
import {
    beforeAll, describe, expect, it,
} from "vitest"
import { edit, initSync } from "../packages/toml-edit-js/shims.js"

const array = dedent`
    [foo]
    bar = [1,2,3]
`
const arrayWithIndent = dedent`
    [foo]
    bar = [1, 2, 3]
`
const opt = { finalNewline: false }

describe("array edit", () => {
    beforeAll(() => {
        initSync()
    })

    describe("set", () => {
        it("set first", () => {
            expect(edit(array, "foo.bar.[0]", 3, opt)).toBe(dedent`
                [foo]
                bar = [3,2,3]
            `)
        })

        it("set at last", () => {
            expect(edit(array, "foo.bar.[3]", 4, opt)).toBe(dedent`
                [foo]
                bar = [1,2,3,4]
            `)
        })

        it("set array at last", () => {
            expect(edit(array, "foo.bar.[3]", [4], opt)).toBe(dedent`
                [foo]
                bar = [1,2,3,[4]]
            `)
        })

        it("set string at last", () => {
            expect(edit(array, "foo.bar.[3]", "baz", opt)).toBe(dedent`
                [foo]
                bar = [1,2,3,"baz"]
            `)
        })

        it("set boolean at last", () => {
            expect(edit(arrayWithIndent, "foo.bar.[3]", false, opt)).toBe(dedent`
                [foo]
                bar = [1, 2, 3, false]
            `)
        })

        it("set date at last", () => {
            expect(edit(arrayWithIndent, "foo.bar.[3]", new Date(0), opt)).toBe(dedent`
                [foo]
                bar = [1, 2, 3, 1970-01-01T00:00:00Z]
            `)
        })
    })

    it("delete", () => {
        expect(edit(array, "foo.bar.[2]", null, opt)).toBe(dedent`
            [foo]
            bar = [1,2]
        `)
    })


    describe("invalid case", () => {
        it("set out of boundary", () => {
            expect(() => edit(array, "foo.bar.[12]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Index out of boundary: '12']`)
        })

        it("empty index", () => {
            expect(() => edit(array, "foo.bar.[]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Invalid array index: '']`)
        })

        it("negative index", () => {
            expect(() => edit(array, "foo.bar.[-1]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Invalid array index: '-1']`)
        })

        it("range index", () => {
            expect(() => edit(array, "foo.bar.[1:3]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Invalid array index: '1:3']`)
        })
    })
})
