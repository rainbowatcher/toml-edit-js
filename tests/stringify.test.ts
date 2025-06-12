import {
    beforeAll, describe, expect, it,
} from "vitest"
import init, { stringify } from "../packages/toml-edit-js/shims.js"


describe("stringify", () => {
    beforeAll(async () => {
        await init()
    })

    describe("simple value", () => {
        it("stringify number", () => {
            const toml = 2
            const result = stringify(toml)
            expect(result).toBe("2")
        })

        it("stringify bool", () => {
            const toml = false
            const result = stringify(toml)
            expect(result).toBe("false")
        })

        it("stringify date", () => {
            const toml = new Date(2023, 1, 1, 0, 0, 0)
            const result = stringify(toml)
            expect(result).toBe("2023-01-31T16:00:00Z")
        })

        it("stringify string", () => {
            const toml = "foo"
            const result = stringify(toml)
            expect(result).toBe("foo")
        })

        it("stringify array", () => {
            const toml = [1, 2, 3]
            const result = stringify(toml)
            expect(result).toBe("[1, 2, 3]")
        })
    })

    it("stringify toml", () => {
        const toml = {
            "": 1,
            "🀄": Infinity,
            $: Number.NaN,
            "0-1": -18,
            "a.b": +99,
            aB: 1,
            b: [
                1,
                2,
                3,
            ],
            c: "hello",
            "cargo-feature": "1",
            d: {
                a: 1,
                b: 2,
            },
            da: "1979-05-27T00:32:00.999999-07:00",
            date: new Date(2023, 1, 1),
            e: {
                d: {
                    h: "2023-01-01T00:00:01-07:00",
                    i: "2023-01-01T00:00:01Z",
                    j: "2023-01-01T00:00:01",
                    k: "2023-01-01",
                },
                f: 1,
            },
        }

        const result = stringify(toml)
        expect(result).toMatchInlineSnapshot(`
            """ = 1
            "🀄" = inf
            "$" = nan
            0-1 = -18
            "a.b" = 99
            aB = 1
            b = [1, 2, 3]
            c = "hello"
            cargo-feature = "1"
            da = "1979-05-27T00:32:00.999999-07:00"
            date = 2023-01-31T16:00:00Z

            [d]
            a = 1
            b = 2

            [e]
            f = 1

            [e.d]
            h = "2023-01-01T00:00:01-07:00"
            i = "2023-01-01T00:00:01Z"
            j = "2023-01-01T00:00:01"
            k = "2023-01-01"
            "
        `)
    })
})
