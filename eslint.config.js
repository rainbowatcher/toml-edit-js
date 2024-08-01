import process from "node:process"
import { defineConfig } from "@rainbowatcher/eslint-config"

export default defineConfig({
    gitignore: true,
    json: true,
    markdown: true,
    style: true,
    toml: true,
    typescript: true,
}, {
    ignores: ["packages/toml-edit-js"],
}, {
    rules: {
        "style-js/linebreak-style": ["error", process.platform === "win32" ? "windows" : "unix"],
    },
})
