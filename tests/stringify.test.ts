import {
    beforeAll, describe, expect, it,
} from "vitest"
import init, { stringify } from "../packages/toml-edit-js/index.js"

beforeAll(async () => {
    await init()
})

describe("stringify", () => {
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
            date: new Date(),
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
            "$" = nan
            0-1 = -18
            "a.b" = 99
            aB = 1
            b = [1, 2, 3]
            c = "hello"
            cargo-feature = "1"
            da = "1979-05-27T00:32:00.999999-07:00"
            "🀄" = inf

            [d]
            a = 1
            b = 2

            [date]

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

    it("stringify array error", () => {
        const toml = [1, 2, 3]
        expect(() => stringify(toml)).toThrowErrorMatchingInlineSnapshot("[Error: Array is not supported]")
    })

})
