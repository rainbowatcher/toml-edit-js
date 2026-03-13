import dedent from "dedent"
import { beforeAll, describe, expect, it } from "vitest"
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
    expect(edit(input, "foo.bar", "qux")).toBe(dedent`
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
      expect(edit(input, "foo.bar", 9_223_372_036_854_775_807, opt)).toStrictEqual(dedent`
                [foo]
                bar = 9223372036854775807
            `)
      expect(edit(input, "foo.bar", 9_223_372_036_854_775_808, opt)).toStrictEqual(dedent`
                [foo]
                bar = 9223372036854775807
            `)
      // eslint-disable-next-line no-loss-of-precision
      expect(edit(input, "foo.bar", -9_223_372_036_854_775_809, opt)).toStrictEqual(dedent`
                [foo]
                bar = -9223372036854775808
            `)
    })

    it("bigint", () => {
      expect(() =>
        edit(input, "foo.bar", 15_033_211_231_241_234_523_452_345_345_787n, opt),
      ).toThrowErrorMatchingInlineSnapshot(`[Error: Bigint is not supported]`)
      expect(() => edit(input, "foo.bar", 1n, opt)).toThrowErrorMatchingInlineSnapshot(
        `[Error: Bigint is not supported]`,
      )
      expect(() =>
        edit(input, "foo.bar", 9_007_199_254_740_992n, opt),
      ).toThrowErrorMatchingInlineSnapshot(`[Error: Bigint is not supported]`)
      expect(() =>
        edit(input, "foo.bar", -9_007_199_254_740_992n, opt),
      ).toThrowErrorMatchingInlineSnapshot(`[Error: Bigint is not supported]`)
    })
  })

  it("unset", () => {
    expect(edit(input, "foo.bar", null, opt)).toBe("[foo]")
    expect(edit(input, "foo.bar", undefined, opt)).toBe("[foo]")
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

    const input1 = dedent`
            [foo]
            # comment
            bar = 1
        `
    expect(edit(input1, "foo.bar", 2, opt)).toBe(dedent`
            [foo]
            # comment
            bar = 2
        `)
    expect(edit(input1, "foo.bar", { baz: 3 }, opt)).toBe(dedent`
            [foo]
            # comment
            bar = { baz = 3 }
        `)
    // TODO: This is a known bug: https://github.com/toml-rs/toml/issues/691
    // expect(edit(input1, "foo.bar", { baz: 3 }, {...opt, inline: false})).toMatchInlineSnapshot(`
    //   "[foo]
    //   # comment
    //   [foo.bar]
    //   baz = 3"
    // `)
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
      expect(() =>
        edit(input, "foo.bar", 1, { unknown: "true" }),
      ).toThrowErrorMatchingInlineSnapshot(`"Type error: unknown property 'unknown'"`)
    })

    it("with invalid type", () => {
      // @ts-expect-error type error
      expect(() =>
        edit(input, "foo.bar", 1, { finalNewline: "true" }),
      ).toThrowErrorMatchingInlineSnapshot(`"Type error: expect finalNewline to be boolean"`)
    })

    it("with array option", () => {
      // @ts-expect-error type error
      expect(() => edit(input, "foo.bar", 1, ["true"])).toThrowErrorMatchingInlineSnapshot(
        `"Type error: IEditOptions can not be array"`,
      )
    })

    it("with string option", () => {
      // @ts-expect-error type error
      expect(() => edit(input, "foo.bar", 1, "true")).toThrowErrorMatchingInlineSnapshot(
        `"Type error: IEditOptions should be an object"`,
      )
    })

    it("last path is invalid", () => {
      edit(input, "foo.bar", 1, opt)
      expect(() =>
        edit(input, "foo.bar.baz", { a: 1, b: 2 }, opt),
      ).toThrowErrorMatchingInlineSnapshot(`"Key Error: invalid key 'baz'"`)
    })

    it("invalid array access", () => {
      expect(() =>
        edit(input, "foo.[0].baz", { a: 1, b: 2 }, opt),
      ).toThrowErrorMatchingInlineSnapshot(`"Type error: item 'foo' is not an array"`)
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

describe("edit with comment", () => {
  beforeAll(async () => {
    await init()
  })

  const inputTableMixedComments = dedent`
        # Table comment
        [foo]
        # Before bar
        bar = 1 # inline comment
        # After bar
        # Second line of comment
        baz = 2
    `
  const inputArrayComments = dedent`
        # Array comment
        items = [
            # comment before first element
            1, # inline comment
            # comment before second element
            2
        ]
    `
  const inputInlineTableComments = dedent`
        # Table comment
        foo = {
            # comment before first element
            a = 1, # inline comment
            # comment before second element
            b = 2
        }
    `

  describe("edit table", () => {
    it("set existing item", () => {
      expect(edit(inputTableMixedComments, "foo.bar", "qux", opt)).toBe(dedent`
                # Table comment
                [foo]
                # Before bar
                bar = "qux" # inline comment
                # After bar
                # Second line of comment
                baz = 2
            `)
      expect(edit(inputTableMixedComments, "foo.baz", "qux", opt)).toBe(dedent`
                # Table comment
                [foo]
                # Before bar
                bar = 1 # inline comment
                # After bar
                # Second line of comment
                baz = "qux"
            `)
    })

    it("add new value", () => {
      expect(edit(inputTableMixedComments, "foo.qux", "qux", opt)).toBe(dedent`
                # Table comment
                [foo]
                # Before bar
                bar = 1 # inline comment
                # After bar
                # Second line of comment
                baz = 2
                qux = "qux"
            `)
    })

    it("remove value", () => {
      expect(edit(inputTableMixedComments, "foo.bar", undefined, opt)).toBe(dedent`
                # Table comment
                [foo]
                # After bar
                # Second line of comment
                baz = 2
            `)
    })
  })

  describe("edit array", () => {
    it("set existing item", () => {
      expect(edit(inputArrayComments, "items.[0]", 3, opt)).toBe(dedent`
                # Array comment
                items = [
                    # comment before first element
                    3, # inline comment
                    # comment before second element
                    2
                ]
            `)
      expect(edit(inputArrayComments, "items.[1]", 4, opt)).toBe(dedent`
                # Array comment
                items = [
                    # comment before first element
                    1, # inline comment
                    # comment before second element
                    4
                ]
            `)
    })

    it("add new item", () => {
      expect(edit(inputArrayComments, "items.[2]", 3, opt)).toBe(dedent`
                # Array comment
                items = [
                    # comment before first element
                    1, # inline comment
                    # comment before second element
                    2,
                    3
                ]
            `)
    })

    it("remove item", () => {
      expect(edit(inputArrayComments, "items.[1]", undefined, opt)).toBe(dedent`
                # Array comment
                items = [
                    # comment before first element
                    1
                ]
            `)
    })
  })

  describe("edit inlinetable", () => {
    it("set existing key", () => {
      expect(edit(inputInlineTableComments, "foo.a", 3, opt)).toBe(dedent`
                # Table comment
                foo = {
                    # comment before first element
                    a = 3, # inline comment
                    # comment before second element
                    b = 2
                }
            `)
      expect(edit(inputInlineTableComments, "foo.b", 4, opt)).toBe(dedent`
                # Table comment
                foo = {
                    # comment before first element
                    a = 1, # inline comment
                    # comment before second element
                    b = 4
                }
            `)
    })

    it("add new key", () => {
      expect(edit(inputInlineTableComments, "foo.c", 3, opt)).toBe(dedent`
                # Table comment
                foo = {
                    # comment before first element
                    a = 1, # inline comment
                    # comment before second element
                    b = 2,
                    c = 3
                }
            `)
    })

    it("remove key", () => {
      expect(edit(inputInlineTableComments, "foo.a", undefined, opt)).toBe(dedent`
                # Table comment
                foo = {
                    # comment before second element
                    b = 2
                }
            `)
    })
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

  it("issue#8", () => {
    const toml = dedent`
            [package]
            # comment
            rand = "1"
        `
    expect(edit(toml, "package.rand", "2", opt)).toBe(dedent`
            [package]
            # comment
            rand = "2"
        `)
  })

  it("issue#11", () => {
    const inputWithComment = dedent`
            [foo]
            one = 1 # a comment
        `
    expect(edit(inputWithComment, "foo.bar", "qux", opt)).toBe(dedent`
            [foo]
            one = 1 # a comment
            bar = "qux"
        `)
  })
})
