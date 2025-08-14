import dedent from "dedent"
import {
    beforeAll, describe, expect, it,
} from "vitest"
import init, { stringify } from "../packages/toml-edit-js/shims.js"

const opts = { finalNewline: false }

describe("stringify", () => {
    beforeAll(async () => {
        await init()
    })

    describe("simple value", () => {
        it("stringify number", () => {
            const toml = 2
            expect(stringify(toml)).toBe("2")
        })

        it("stringify bool", () => {
            const toml = false
            expect(stringify(toml)).toBe("false")
        })

        // TODO: Find out the reason why toml automatically converts time zones
        it("stringify date", () => {
            const toml = new Date(0)
            expect(stringify(toml)).toBe("1970-01-01T00:00:00Z")
        })

        it("stringify string", () => {
            const toml = "foo"
            expect(stringify(toml)).toBe('"foo"')
        })

        it("stringify array", () => {
            const toml = [1, 2, 3]
            expect(stringify(toml)).toBe("[1, 2, 3]")
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
            date: new Date(0),
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

        expect(stringify(toml, opts)).toStrictEqual(dedent`
            "" = 1
            "🀄" = inf
            "$" = nan
            0-1 = -18
            "a.b" = 99
            aB = 1
            b = [1, 2, 3]
            c = "hello"
            cargo-feature = "1"
            da = "1979-05-27T00:32:00.999999-07:00"
            date = 1970-01-01T00:00:00Z

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
        `)
    })

    // https://github.com/rainbowatcher/toml-edit-js/issues/6
    it("issue#6", () => {
        const toml = {
            project: {
                dependencies: [
                    {
                        git: "https://github.com/gilead-biostats/gsm.core",
                        name: "gsm.core",
                        tag: "v1.1.0",
                    },
                    {
                        branch: "main",
                        git: "https://github.com/gilead-rbqm/grail.ado",
                        name: "grail.ado",
                    },
                    "pkgpub",
                    "tomledit",
                ],
                name: "prep-pkgs",
                r_version: "4.4",
                repositories: [
                    {
                        alias: "prism",
                        url: "https://prism.dev.a2-ai.cloud/rpkgs/stratus/2025-04-26/",
                    },
                    {
                        alias: "CRAN",
                        url: "https://packagemanager.posit.co/cran/latest",
                    },
                ],
            },
        }
        expect(stringify(toml, opts)).toStrictEqual(dedent`
            [project]
            dependencies = [{ git = "https://github.com/gilead-biostats/gsm.core", name = "gsm.core", tag = "v1.1.0" }, { branch = "main", git = "https://github.com/gilead-rbqm/grail.ado", name = "grail.ado" }, "pkgpub", "tomledit"]
            name = "prep-pkgs"
            r_version = "4.4"
            repositories = [{ alias = "prism", url = "https://prism.dev.a2-ai.cloud/rpkgs/stratus/2025-04-26/" }, { alias = "CRAN", url = "https://packagemanager.posit.co/cran/latest" }]
        `)
    })
})
