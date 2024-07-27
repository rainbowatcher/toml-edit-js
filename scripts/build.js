import { readFileSync } from "node:fs"
import { readFile, writeFile } from "node:fs/promises"
import process from "node:process"
import { $ } from "execa"
import { ensureDir, ensureFile } from "fs-extra"
import ora from "ora"
import c from "picocolors"
import { rimraf } from "rimraf"

const { version } = readFileSync(`${process.cwd()}/package.json`, "utf8")

const distPackageName = "toml-edit-js"
const sourceDir = "."
const distDir = `packages/${distPackageName}`
const packageName = "qi-js"
const indexFilePath = `${process.cwd()}/${distDir}/index.js`
const pkgJsonPath = `${process.cwd()}/${distDir}/package.json`
const wasmPath = `${process.cwd()}/${distDir}/index_bg.wasm`
const author = "rainbowatcher <rainbow-w@qq.com>"
const repo = "https://github.com/rainbowatcher/toml-edit-js/"
const license = "MIT"
const pkgJsonContent = JSON.stringify({
    author,
    bugs: {
        url: `${repo}/issues`,
    },
    description: "A tiny, fast, and powerful formatter for documentation, especially for content in CJK.",
    files: [
        "index_bg.wasm",
        "index.js",
        "index.d.ts",
    ],
    homepage: `${repo}#readme`,
    license,
    main: "index.js",
    module: "index.js",
    name: distPackageName,
    repository: {
        type: "git",
        url: `git+${repo}.git`,
    },
    type: "module",
    types: "index.d.ts",
    version,
}, null, 2)

const spinner = ora()
spinner.info("Start building")
spinner.start(`Cleaning ${distDir}`)
await ensureDir(`${process.cwd()}/${distDir}`)
await rimraf(`${process.cwd()}/${distDir}`)
spinner.succeed()

spinner.start("Building wasm")
await $`wasm-pack build ${sourceDir} --no-pack --no-opt --release -d ${process.cwd()}/${distDir} -t web --out-name index`
spinner.succeed()

spinner.start("Optimizing wasm")
await $`wasm-opt -O4 ${wasmPath} --enable-threads --enable-bulk-memory -o ${wasmPath}`
spinner.succeed()

spinner.start("Handle index.js")
const indexStr = await readFile(indexFilePath, "utf8")
const fetchExpr = "input = fetch(input);"
if (indexStr.includes(fetchExpr)) {
    const newIndexStr = `/* eslint-disable unicorn/no-abusive-eslint-disable */
/* eslint-disable */\n${indexStr.replace(fetchExpr, /* javascript */`if (globalThis.process?.release?.name === "node") {
        const fs = (await import('fs')).default;
        input = fs.readFileSync(input);
        } else {
            input = fetch(input);
    }`)}`
    await writeFile(indexFilePath, newIndexStr)
} else {
    spinner.stopAndPersist({ text: "generated js file may incorrect" })
    throw new Error("input is not defined in the generated js file")
}
spinner.succeed()

spinner.start("Handle package.json")
await ensureFile(pkgJsonPath)
await writeFile(pkgJsonPath, pkgJsonContent)
spinner.succeed()

// eslint command fail here, execute eslint twice then the error disappears
await rimraf(`${process.cwd()}/${distDir}/.gitignore`)

// try {
//     spinner.start("Linting generated code")
//     await $`pnpm exec eslint ${process.cwd()}/${targetDir} --fix`
//     spinner.succeed()
// } catch {
//     await $`pnpm exec eslint ${process.cwd()}/${targetDir} --fix`
//     spinner.succeed()
// }

spinner.succeed(`Build ${c.green("success")}`)
spinner.stop()
