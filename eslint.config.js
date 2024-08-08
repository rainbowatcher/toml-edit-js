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
    files: ["**/*.md/**"],
    rules: {
        "style-js/lines-around-comment": "off",
        "style-ts/lines-around-comment": "off",
    },
})
