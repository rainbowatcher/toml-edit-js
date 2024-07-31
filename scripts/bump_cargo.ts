import fs from "node:fs/promises"
import path from "node:path"
import process from "node:process"
import c from "picocolors"
import { version } from "../package.json"
import init, { edit } from "../packages/toml-edit-js"

await init()
const CARGO_FILE_PATH = "Cargo.toml"
const ABS_CARGO_PATH = path.resolve(process.cwd(), CARGO_FILE_PATH)
const cargoFileStr = await fs.readFile(CARGO_FILE_PATH, "utf8")
const newCargoFileStr = edit(cargoFileStr, "package.version", version)
await fs.writeFile(ABS_CARGO_PATH, newCargoFileStr)
console.log(`${c.green("✔")} upgrade ${ABS_CARGO_PATH}`)
