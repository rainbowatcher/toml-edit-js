import dedent from "dedent"
import init, { parse } from "packages/toml-edit-js/index.js"
import {
    beforeAll, describe, expect, it,
} from "vitest"

describe("parse", () => {
    beforeAll(async () => {
        await init()
    })

    describe("key", () => {
        it("normal keys", () => {
            const toml = `key = "value"
            bare_key = "value"
            bare-key = "value"
            1234 = "value"
            `
            expect(parse(toml)).toStrictEqual({
                1234: "value",
                bare_key: "value",
                "bare-key": "value",
                key: "value",
            })
        })

        it("quoted keys", () => {
            const toml = `"127.0.0.1" = "value"
            "character encoding" = "value"
            "ʎǝʞ" = "value"
            'key2' = "value"
            'quoted "value"' = "value"`
            expect(parse(toml)).toStrictEqual({
                "127.0.0.1": "value",
                "character encoding": "value",
                key2: "value",
                ʎǝʞ: "value",
                "quoted \"value\"": "value",
            })
        })

        it("dot keys", () => {
            const toml = `physical.color = "orange"
            physical.shape = "round"
            site."google.com" = true
            fruit.name = "banana"
            fruit. color = "yellow"
            fruit . flavor = "banana"`
            expect(parse(toml)).toStrictEqual({
                fruit: {
                    color: "yellow",
                    flavor: "banana",
                    name: "banana",
                },
                physical: {
                    color: "orange",
                    shape: "round",
                },
                site: {
                    "google.com": true,
                },
            })
        })
    })


    describe("string", () => {
        it("normal", () => {
            const toml = `str = "I'm a string."
            cargo-feature = "async"
            `
            expect(parse(toml)).toStrictEqual({
                "cargo-feature": "async",
                str: "I'm a string.",
            })
        })

        it("multiline", () => {
            const toml = dedent`
                str = """
                Roses are red
                Violets are blue"""
    
                str2 = """
                The quick brown \
    
    
                fox jumps over \
                the lazy dog."""
            `
            expect(parse(toml)).toStrictEqual({
                str: "Roses are red\nViolets are blue",
                str2: "The quick brown \n\nfox jumps over the lazy dog.",
            })
        })

        it("single quote", () => {
            const toml = dedent.withOptions({ escapeSpecialCharacters: false })`
                # What you see is what you get.
                winpath  = 'C:\Users\nodejs\templates'
                winpath2 = '\\ServerX\admin$\system32\'
                quoted   = 'Tom "Dubs" Preston-Werner'
                regex    = '<\i\c*\s*>'
            `
            expect(parse(toml)).toStrictEqual({
                quoted: "Tom \"Dubs\" Preston-Werner",
                regex: String.raw`<\i\c*\s*>`,
                winpath: String.raw`C:\Users\nodejs\templates`,
                winpath2: "\\\\ServerX\\admin$\\system32\\",
            })
        })

        it("comments", () => {
            const toml = "# this is a comment"
            expect(parse(toml)).toMatchInlineSnapshot("{}")

            const toml2 = `str = "I'm a string." # this is a comment`
            expect(parse(toml2)).toStrictEqual({
                str: "I'm a string.",
            })
        })

        it("with escape chars", () => {
            const toml = `str = "I'm a string. \\"You can quote me\\". Name\\tJos\\u00E9\\nLocation\\tSF."`
            expect(parse(toml)).toStrictEqual({
                str: "I'm a string. \"You can quote me\". Name	José\nLocation	SF.",
            })
        })

        it("error with wrong escape chars", () => {
            const errToml = `str = "I'm a string. "You can quote me". Name\tJos\u00E9\nLocation\tSF."`
            expect(() => { parse(errToml) }).toThrowErrorMatchingInlineSnapshot(`
            [Error: TOML parse error at line 1, column 23
              |
            1 | str = "I'm a string. "You can quote me". Name	José
              |                       ^
            expected newline, \`#\`
            ]
          `)
        })
    })


    describe("interger", () => {
        it("normal", () => {
            const toml = dedent`
                a = 1
                b=+99
                c=0
                d =-17
            `
            expect(parse(toml)).toStrictEqual({
                a: 1,
                b: 99,
                c: 0,
                d: -17,
            })
        })

        it("with underscore", () => {
            const toml = dedent`
                int5 = 1_000
                int6 = 5_349_221
                int7 = 53_49_221
                int8 = 1_2_3_4_5
            `
            expect(parse(toml)).toStrictEqual({
                int5: 1000,
                int6: 5_349_221,
                int7: 5_349_221,
                int8: 12_345,
            })
        })

        it("hexadecimal, octal, or binary", () => {
            const toml = dedent`
                # hexadecimal with prefix '0x'
                hex1 = 0xDEADBEEF
                hex2 = 0xdeadbeef
                hex3 = 0xdead_beef
    
                # octal with prefix '0o'
                oct1 = 0o01234567
                oct2 = 0o755 # useful for Unix file permissions
    
                # binary with prefix '0b'
                bin1 = 0b11010110
            `
            expect(parse(toml)).toStrictEqual({
                bin1: 214,
                hex1: 3_735_928_559,
                hex2: 3_735_928_559,
                hex3: 3_735_928_559,
                oct1: 342_391,
                oct2: 493,
            })
        })
    })

    describe("float", () => {
        it("normal", () => {
            const toml = dedent`
                # fractional
                flt1 = +1.0
                flt2 = 3.1415
                flt3 = -0.01
    
                # exponent
                flt4 = 5e+22
                flt5 = 1e06
                flt6 = -2E-2
    
                # both
                flt7 = 6.626e-34
            `
            expect(parse(toml)).toStrictEqual({
                flt1: 1,
                flt2: 3.1415,
                flt3: -0.01,
                flt4: 5e+22,
                flt5: 1_000_000,
                flt6: -0.02,
                flt7: 6.626e-34,
            })
        })

        it("with underscore", () => {
            const toml = "flt8 = 224_617.445_991_228"
            expect(parse(toml)).toStrictEqual({
                flt8: 224_617.445_991_228,
            })
        })

        it("infinity", () => {
            const toml = dedent`
                # infinity
                sf1 = inf
                sf2 = +inf
                sf3 = -inf
    
                # not a number
                sf4 = nan
                sf5 = +nan
                sf6 = -nan
            `
            expect(parse(toml)).toStrictEqual({
                sf1: Infinity,
                sf2: Infinity,
                sf3: -Infinity,
                sf4: Number.NaN,
                sf5: Number.NaN,
                sf6: Number.NaN,
            })
        })
    })

    describe("boolean", () => {
        it("normal", () => {
            const toml = `t = true
            f = false`
            expect(parse(toml)).toStrictEqual({
                f: false,
                t: true,
            })
        })
    })

    describe("datetime", () => {
        // we convert toml date time to string
        it("normal", () => {
            const toml = dedent`
                odt1 = 1979-05-27T07:32:00Z
                odt2 = 1979-05-27T00:32:00-07:00
                odt3 = 1979-05-27T00:32:00.999999-07:00
                odt4 = 1979-05-27 07:32:00Z
    
                ldt1 = 1979-05-27T07:32:00
                ldt2 = 1979-05-27T00:32:00.999999
    
                ld1 = 1979-05-27
                lt1 = 07:32:00
                lt2 = 00:32:00.999999
            `
            expect(parse(toml)).toStrictEqual({
                ld1: "1979-05-27",
                ldt1: "1979-05-27T07:32:00",
                ldt2: "1979-05-27T00:32:00.999999",
                lt1: "07:32:00",
                lt2: "00:32:00.999999",
                odt1: "1979-05-27T07:32:00Z",
                odt2: "1979-05-27T00:32:00-07:00",
                odt3: "1979-05-27T00:32:00.999999-07:00",
                odt4: "1979-05-27T07:32:00Z",
            })
        })
    })


    describe("array", () => {
        it("normal", () => {
            const toml = dedent`
                integers = [ 1, 2, 3 ]
                floats = [
                    1.1,
                    2.2,
                    3.3
                ]
                colors = [ "red", "yellow", "green" ]
                nested_arrays_of_ints = [ [ 1, 2 ], [3, 4, 5] ]
                nested_mixed_array = [ [ 1, 2 ], ["a", "b", "c"] ]
                string_array = [ "all", 'strings', """are the same""", '''type''' ]
    
                # Mixed-type arrays are allowed
                numbers = [ 0.1, 0.2, 0.5, 1, 2, 5 ]
                contributors = [
                "Foo Bar <foo@example.com>",
                { name = "Baz Qux", email = "bazqux@example.com", url = "https://example.com/bazqux" }
                ]
            `
            expect(parse(toml)).toStrictEqual({
                colors: [
                    "red",
                    "yellow",
                    "green",
                ],
                contributors: [
                    "Foo Bar <foo@example.com>",
                    {
                        email: "bazqux@example.com",
                        name: "Baz Qux",
                        url: "https://example.com/bazqux",
                    },
                ],
                floats: [
                    1.1,
                    2.2,
                    3.3,
                ],
                integers: [
                    1,
                    2,
                    3,
                ],
                nested_arrays_of_ints: [
                    [
                        1,
                        2,
                    ],
                    [
                        3,
                        4,
                        5,
                    ],
                ],
                nested_mixed_array: [
                    [
                        1,
                        2,
                    ],
                    [
                        "a",
                        "b",
                        "c",
                    ],
                ],
                numbers: [
                    0.1,
                    0.2,
                    0.5,
                    1,
                    2,
                    5,
                ],
                string_array: [
                    "all",
                    "strings",
                    "are the same",
                    "type",
                ],
            })
        })

    })


    describe("table", () => {
        it("normal", () => {
            const toml = `a = { b = "c", d = "e" }`
            expect(parse(toml)).toStrictEqual({
                a: {
                    b: "c",
                    d: "e",
                },
            })
        })

        it("name", () => {
            const toml = `[dog."tater.man"]
            type.name = "pug"`
            expect(parse(toml)).toStrictEqual({
                dog: {
                    "tater.man": {
                        type: {
                            name: "pug",
                        },
                    },
                },
            })
        })

        it("array of tables", () => {
            const toml = dedent`
                [[products]]
                name = "Hammer"
                sku = 738594937
    
                [[products]]
    
                [[products]]
                name = "Nail"
                sku = 284758393
    
                color = "gray"
            `
            expect(parse(toml)).toStrictEqual({
                products: [
                    {
                        name: "Hammer",
                        sku: 738_594_937,
                    },
                    {},
                    {
                        color: "gray",
                        name: "Nail",
                        sku: 284_758_393,
                    },
                ],
            })
        })

        it("array of tables2", () => {
            const toml = dedent`
                [[fruits]]
                name = "apple"
    
                [fruits.physical]  # subtable
                color = "red"
                shape = "round"
    
                [[fruits.varieties]]  # nested array of tables
                name = "red delicious"
    
                [[fruits.varieties]]
                name = "granny smith"
    
    
                [[fruits]]
                name = "banana"
    
                [[fruits.varieties]]
                name = "plantain"
            `
            expect(parse(toml)).toStrictEqual({
                fruits: [
                    {
                        name: "apple",
                        physical: {
                            color: "red",
                            shape: "round",
                        },
                        varieties: [
                            {
                                name: "red delicious",
                            },
                            {
                                name: "granny smith",
                            },
                        ],
                    },
                    {
                        name: "banana",
                        varieties: [
                            {
                                name: "plantain",
                            },
                        ],
                    },
                ],
            })
        })

        it("extra", () => {
            const toml = dedent`
                points = [ { x = 1, y = 2, z = 3 },
                           { x = 7, y = 8, z = 9 },
                           { x = 2, y = 4, z = 8 } ]
            `
            expect(parse(toml)).toStrictEqual({
                points: [
                    {
                        x: 1,
                        y: 2,
                        z: 3,
                    },
                    {
                        x: 7,
                        y: 8,
                        z: 9,
                    },
                    {
                        x: 2,
                        y: 4,
                        z: 8,
                    },
                ],
            })
        })
    })
})
