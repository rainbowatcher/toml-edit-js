import { defineConfig } from "wasmup"

export default defineConfig({
    clean: true,
    entries: ["."],
    output: "packages/toml-edit-js",
    release: true,
    scope: "rainbowatcher",
})
