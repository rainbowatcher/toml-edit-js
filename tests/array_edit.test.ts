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

        it("array of tables", () => {
            const aot = dedent`
                [foo]
                bar = [
                    { name = "tom",age = 12 }
                ]
            `
            expect(edit(aot, "foo.bar.[0].age", 20, opt)).toBe(dedent`
                [foo]
                bar = [
                    { name = "tom",age = 20 }
                ]
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
            expect(() => edit(array, "foo.bar.[12]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: index out of boundary '12' for 'foo.bar'"`)
            expect(() => edit(array, "foo.bar.[4]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: index out of boundary '4' for 'foo.bar'"`)
        })

        it("empty index", () => {
            expect(() => edit(array, "foo.bar.[]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: invalid key '[]'"`)
        })

        it("negative index", () => {
            expect(() => edit(array, "foo.bar.[-1]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: invalid key '[-1]'"`)
        })

        it("range index", () => {
            expect(() => edit(array, "foo.bar.[1:3]", 4, opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: invalid key '[1:3]'"`)
        })
    })

    describe("array boundary tests", () => {
        it("empty array boundary", () => {
            const emptyArray = dedent`
                [foo]
                bar = [
                  { baz = 1 }
                ]
            `
            // Delete non-existent element should fail
            expect(() => edit(emptyArray, "foo.bar.[1].baz", 4, opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: index out of boundary '1' for 'foo.bar.[1]'"`)
        })

        it("array of tables boundary", () => {
            const aot = dedent`
                [[foo.bar]]
                name = "test1"
                [[foo.bar]]
                name = "test2"
            `
            expect(edit(aot, "foo.bar.[0]", { name: "test3" }, opt)).toStrictEqual(dedent`
                [[foo.bar]]
                name = "test3"
                [[foo.bar]]
                name = "test2"
            `)
            expect(edit(aot, "foo.bar.[1]", { name: "test3" }, opt)).toStrictEqual(dedent`
                [[foo.bar]]
                name = "test1"

                [[foo.bar]]
                name = "test3"
            `)
            expect(edit(aot, "foo.bar.[2]", { name: "test3" }, opt)).toStrictEqual(dedent`
                [[foo.bar]]
                name = "test1"
                [[foo.bar]]
                name = "test2"

                [[foo.bar]]
                name = "test3"
            `)
        })

        it("large index out of boundary", () => {
            expect(() => edit(array, "foo.bar.[999]", 1, opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: index out of boundary '999' for 'foo.bar'"`)
        })

        it("boundary in nested array", () => {
            const nested = dedent`
                [foo]
                bar = [[1,2], [3,4]]
            `
            expect(() => edit(nested, "foo.bar.[3]", [5, 6], opt)).toThrowErrorMatchingInlineSnapshot(`"Key Error: index out of boundary '3' for 'foo.bar'"`)
        })
    })
})
