import dedent from "dedent"
import {
    beforeAll, describe, expect, it,
} from "vitest"
import init, { edit, initSync } from "../packages/toml-edit-js/shims.js"

const input = dedent`
    [foo]
    bar = 1
`
const opt = { finalNewline: false }

describe("edit", () => {
    beforeAll(async () => {
        await init()
    })

    it("set string", () => {
        expect(edit(input, "foo.bar", "qux", opt)).toBe(dedent`
            [foo]
            bar = "qux"
        `)
    })

    it("set string with default finalNewline", () => {
        expect(edit(input, "foo.bar", "qux"))
            .toBe(dedent`
                [foo]
                bar = "qux"\n
            `)
    })

    describe("set number", () => {
        it("integer", () => {
            expect(edit(input, "foo.bar", 2, opt)).toBe(dedent`
                [foo]
                bar = 2
            `)
        })

        it("float", () => {
            expect(edit(input, "foo.bar", 2.2, opt)).toBe(dedent`
                [foo]
                bar = 2.2
            `)
        })

        it("negative float", () => {
            expect(edit(input, "foo.bar", -2.2, opt)).toBe(dedent`
                [foo]
                bar = -2.2
            `)
        })

        it("nan", () => {
            expect(edit(input, "foo.bar", Number.NaN, opt)).toBe(dedent`
                [foo]
                bar = nan
            `)
        })

        it("infinity", () => {
            expect(edit(input, "foo.bar", Infinity, opt)).toBe(dedent`
                [foo]
                bar = inf
            `)
        })

        it("scientific notation", () => {
            expect(edit(input, "foo.bar", 10e3, opt)).toBe(dedent`
                [foo]
                bar = 10000
            `)
            expect(edit(input, "foo.bar", -0.1e5, opt)).toBe(dedent`
                [foo]
                bar = -10000
            `)
        })

        it("with safe number boundary", () => {
            expect(edit(input, "foo.bar", 9_007_199_254_740_991, opt)).toBe(dedent`
                [foo]
                bar = 9007199254740991
            `)
            expect(edit(input, "foo.bar", -9_007_199_254_740_991, opt)).toBe(dedent`
                [foo]
                bar = -9007199254740991
            `)
        })

        it("with i64 boundary", () => {
            // eslint-disable-next-line no-loss-of-precision
            expect(edit(input, "foo.bar", 9_223_372_036_854_775_807, opt)).toBe(dedent`
                [foo]
                bar = 9223372036854775807
            `)
            expect(edit(input, "foo.bar", 9_223_372_036_854_775_808, opt)).toMatchInlineSnapshot(`
                "[foo]
                bar = 9223372036854775807"
            `)
            // eslint-disable-next-line no-loss-of-precision
            expect(edit(input, "foo.bar", -9_223_372_036_854_775_809, opt)).toMatchInlineSnapshot(`
                "[foo]
                bar = -9223372036854775808"
            `)
        })

        it("bigint", () => {
            expect(() => edit(input, "foo.bar", 15_033_211_231_241_234_523_452_345_345_787n, opt))
                .toThrowErrorMatchingInlineSnapshot(`[Error: Bigint is not supported]`)
            expect(() => edit(input, "foo.bar", 1n, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Bigint is not supported]`)
            expect(() => edit(input, "foo.bar", 9_007_199_254_740_992n, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Bigint is not supported]`)
            expect(() => edit(input, "foo.bar", -9_007_199_254_740_992n, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Bigint is not supported]`)
        })
    })


    it("unset", () => {
        expect(edit(input, "foo.bar", null, opt)).toMatchInlineSnapshot(`"[foo]"`)
        expect(edit(input, "foo.bar", undefined, opt)).toMatchInlineSnapshot(`"[foo]"`)
    })

    it("set boolean", () => {
        expect(edit(input, "foo.bar", false, opt)).toBe(dedent`
            [foo]
            bar = false
        `)
    })

    it("set array", () => {
        expect(edit(input, "foo.bar", [1, 2, 3], opt)).toBe(dedent`
            [foo]
            bar = [1, 2, 3]
        `)
        expect(edit(input, "foo.bar", [4, 5, 6], opt)).toBe(dedent`
            [foo]
            bar = [4, 5, 6]
        `)
    })

    it("set object", () => {
        expect(edit(input, "foo.bar", { a: 1, b: 2 }, opt)).toBe(dedent`
            [foo]
            bar = { a = 1, b = 2 }
        `)

        expect(edit(input, "foo.bar", { a: 1, b: 2 }, { ...opt, inline: false })).toBe(dedent`
            [foo]

            [foo.bar]
            a = 1
            b = 2
        `)
    })

    it("set datetime", () => {
        expect(edit(input, "foo.bar", new Date(0), opt)).toBe(dedent`
            [foo]
            bar = 1970-01-01T00:00:00Z
        `)
    })

    it("nest key", () => {
        const input1 = dedent`
            [foo.bar]
            baz = 0
        `
        expect(edit(input1, "foo.bar.baz", 1, opt)).toBe(dedent`
            [foo.bar]
            baz = 1
        `)
    })

    describe("key with space", () => {
        it("value key with space", () => {
            const input1 = dedent`
                [foo.bar]
                baz = 0
            `
            expect(edit(input1, "foo.bar. baz", 1, opt)).toBe(dedent`
                [foo.bar]
                baz = 0
                " baz" = 1
            `)
        })

        it("path key with space", () => {
            const input1 = dedent`
                [foo.bar]
                baz = 0
            `
            expect(edit(input1, `foo." bar".baz`, 1, opt)).toBe(dedent`
                [foo.bar]
                baz = 0

                [foo." bar"]
                baz = 1
            `)
        })
    })

    describe("key with dot", () => {
        it("value key with dot", () => {
            const input1 = dedent`
                [foo]
                bar = 0
            `
            expect(edit(input1, `foo."bar.baz"`, 1, opt)).toBe(dedent`
                [foo]
                bar = 0
                "bar.baz" = 1
            `)
        })
    })

    describe("error", () => {
        it("with unknown field", () => {
            // @ts-expect-error type error
            expect(() => edit(input, "foo.bar", 1, { unknown: "true" })).toThrowErrorMatchingInlineSnapshot(`[Error: Unknown property 'unknown']`)
        })

        it("with invalid type", () => {
            // @ts-expect-error type error
            expect(() => edit(input, "foo.bar", 1, { finalNewline: "true" })).toThrowErrorMatchingInlineSnapshot(`[Error: Type Missmatch, expect finalNewline to be boolean]`)
        })

        it("with array option", () => {
            // @ts-expect-error type error
            expect(() => edit(input, "foo.bar", 1, ["true"])).toThrowErrorMatchingInlineSnapshot(`[Error: Type Missmatch, IEditOptions can not be array]`)
        })

        it("with string option", () => {
            // @ts-expect-error type error
            expect(() => edit(input, "foo.bar", 1, "true")).toThrowErrorMatchingInlineSnapshot(`[Error: IEditOptions should be an object]`)
        })

        it("last path is invalid", () => {
            edit(input, "foo.bar", 1, opt)
            expect(() => edit(input, "foo.bar.baz", { a: 1, b: 2 }, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: Invalid key: 'baz']`)
        })

        it("invalid array access", () => {
            expect(() => edit(input, "foo.[0].baz", { a: 1, b: 2 }, opt)).toThrowErrorMatchingInlineSnapshot(`[Error: 'foo' is not a array]`)
        })
    })
})

describe("edit with sync init", () => {
    beforeAll(() => {
        initSync()
    })

    it("set string", () => {
        expect(edit(input, "foo.bar", "qux", opt)).toBe(dedent`
            [foo]
            bar = "qux"
        `)
    })
})


describe("issue", () => {
    beforeAll(() => {
        initSync()
    })

    // https://github.com/rainbowatcher/toml-edit-js/issues/6
    it("issue#6", () => {
        const toml = dedent`
            [project]
            name = "prep-pkgs"
            r_version = "4.4"

            # any CRAN-type repository, order matters. Additional ability to force source package installation
            # Example: {alias = "CRAN", url = "https://cran.r-project.org", force_source = true}
            repositories = [
                { alias = "prism", url = "https://prism.dev.a2-ai.cloud/rpkgs/stratus/2025-04-26/" },
                { alias = "CRAN", url = "https://packagemanager.posit.co/cran/latest" },
            ]

            dependencies = [
                # a comment before the first dep
                { name = "gsm.core", git = "https://github.com/gilead-biostats/gsm.core", tag = "v1.1.0" },
                # a comment after the first dep
                { name = "grail.ado", git = "https://github.com/gilead-rbqm/grail.ado", branch = "main" },
                "pkgpub",
                "tomledit"
            ]
        `
        expect(edit(toml, "project.dependencies.[0].tag", "v1.2.0")).toMatchInlineSnapshot(`
            "[project]
            name = "prep-pkgs"
            r_version = "4.4"

            # any CRAN-type repository, order matters. Additional ability to force source package installation
            # Example: {alias = "CRAN", url = "https://cran.r-project.org", force_source = true}
            repositories = [
                { alias = "prism", url = "https://prism.dev.a2-ai.cloud/rpkgs/stratus/2025-04-26/" },
                { alias = "CRAN", url = "https://packagemanager.posit.co/cran/latest" },
            ]

            dependencies = [
                # a comment before the first dep
                { name = "gsm.core", git = "https://github.com/gilead-biostats/gsm.core", tag = "v1.2.0" },
                # a comment after the first dep
                { name = "grail.ado", git = "https://github.com/gilead-rbqm/grail.ado", branch = "main" },
                "pkgpub",
                "tomledit"
            ]
            "
        `)
    })
})
