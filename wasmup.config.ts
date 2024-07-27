import { defineConfig } from "wasmup"

export default defineConfig({
    clean: true,
    entry: ".",
    output: "packages/toml-edit-js",
    release: true,
})
