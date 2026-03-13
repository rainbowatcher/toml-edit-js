## <small>0.6.5 (2026-03-13)</small>

* fix: preserve array style when deleting down to one element ([04dcbb1](https://github.com/rainbowatcher/toml-edit-js/commit/04dcbb1))
* fix: preserve TOML decoration/comments in set operations ([9d91373](https://github.com/rainbowatcher/toml-edit-js/commit/9d91373))
* fix(set): preserve TOML decoration on array and table mutations ([0d2b2e6](https://github.com/rainbowatcher/toml-edit-js/commit/0d2b2e6))
* fix(test): move ts directive to right place ([9b7feeb](https://github.com/rainbowatcher/toml-edit-js/commit/9b7feeb))
* style(code): apply oxc formatting across project files ([6ffb4f2](https://github.com/rainbowatcher/toml-edit-js/commit/6ffb4f2))
* style(config): reformat project configuration files ([836b047](https://github.com/rainbowatcher/toml-edit-js/commit/836b047))
* chore: update deps ([4bd7dc5](https://github.com/rainbowatcher/toml-edit-js/commit/4bd7dc5))
* chore(deps): bump deps ([db07a63](https://github.com/rainbowatcher/toml-edit-js/commit/db07a63))
* chore(deps): bump deps ([86f23c4](https://github.com/rainbowatcher/toml-edit-js/commit/86f23c4))
* chore(deps): update conventional-changelog dependency to version 7.2.0 ([c07898a](https://github.com/rainbowatcher/toml-edit-js/commit/c07898a))
* chore(eslint): add .alma-snapshots to ignored paths ([5a4fd87](https://github.com/rainbowatcher/toml-edit-js/commit/5a4fd87))
* chore(tooling): migrate lint and format tools to oxc ([5ac9a91](https://github.com/rainbowatcher/toml-edit-js/commit/5ac9a91))
* docs: add rustdoc and clarify insert behavior ([394bf0d](https://github.com/rainbowatcher/toml-edit-js/commit/394bf0d))
* refactor: refactor path mutations and add robust array/table edits ([f5bd7e2](https://github.com/rainbowatcher/toml-edit-js/commit/f5bd7e2))
* refactor(core): pass edit path keys by reference ([e20c587](https://github.com/rainbowatcher/toml-edit-js/commit/e20c587))

## <small>0.6.4 (2025-11-19)</small>

- chore: disable commit body max line length in commitlint ([1958900](https://github.com/rainbowatcher/toml-edit-js/commit/1958900))
- chore(deps): update Rust and Node.js dependencies to latest versions ([b575a17](https://github.com/rainbowatcher/toml-edit-js/commit/b575a17))
- refactor: enhance array boundary checks and update related tests ([0f5ce50](https://github.com/rainbowatcher/toml-edit-js/commit/0f5ce50))

## <small>0.6.3 (2025-09-28)</small>

- chore: bump deps ([a8184cd](https://github.com/rainbowatcher/toml-edit-js/commit/a8184cd))

## <small>0.6.2 (2025-09-15)</small>

- refactor: refactor and optimize path parsing and quote handling logic ([ade0d35](https://github.com/rainbowatcher/toml-edit-js/commit/ade0d35))
- refactor: refactor function parameters to use references for consistency ([72c155c](https://github.com/rainbowatcher/toml-edit-js/commit/72c155c))
- chore: bump deps ([738d52b](https://github.com/rainbowatcher/toml-edit-js/commit/738d52b))
- chore: update dependencies ([fac853c](https://github.com/rainbowatcher/toml-edit-js/commit/fac853c))
- docs: correct output ([8ea67e2](https://github.com/rainbowatcher/toml-edit-js/commit/8ea67e2))

## <small>0.6.1 (2025-08-20)</small>

- fix: fix code formatting and syntax errors in example code ([6a0ae5c](https://github.com/rainbowatcher/toml-edit-js/commit/6a0ae5c))
- fix: fix error message string to match expected output ([1b015a6](https://github.com/rainbowatcher/toml-edit-js/commit/1b015a6))
- refactor: improve error handling and validation in core module ([b38776c](https://github.com/rainbowatcher/toml-edit-js/commit/b38776c))
- chore: remove support for legacy toml-edit-js versions ([e09e01b](https://github.com/rainbowatcher/toml-edit-js/commit/e09e01b))
- chore: update lock file ([bbdfc43](https://github.com/rainbowatcher/toml-edit-js/commit/bbdfc43))

## 0.6.0 (2025-08-14)

- style: apply lint style rule ([8117984](https://github.com/rainbowatcher/toml-edit-js/commit/8117984))
- docs: improve documentation, examples, and clarify API usage ([1f14f9c](https://github.com/rainbowatcher/toml-edit-js/commit/1f14f9c))
- feat: implement inline table support in serialization logic ([2e42512](https://github.com/rainbowatcher/toml-edit-js/commit/2e42512))
- feat: refactor stringify to improve newline control ([eb7a210](https://github.com/rainbowatcher/toml-edit-js/commit/eb7a210))
- ci: add GitHub Actions workflow for automated release process ([1bce830](https://github.com/rainbowatcher/toml-edit-js/commit/1bce830))
- perf: add multiple TOML parser and serializer benchmarks ([e1ecb05](https://github.com/rainbowatcher/toml-edit-js/commit/e1ecb05))
- perf: optimize functions with inline attributes for performance gains ([b513b64](https://github.com/rainbowatcher/toml-edit-js/commit/b513b64))
- refactor: fix parse_edit_path signature and improve float-to-integer conversion ([4326fc3](https://github.com/rainbowatcher/toml-edit-js/commit/4326fc3))
- refactor: functions to convert between Js Value and TOML Item to avoid unnecessary object copying ([0054900](https://github.com/rainbowatcher/toml-edit-js/commit/0054900))
- refactor: improve parse_edit_path robustness and performance ([dffae90](https://github.com/rainbowatcher/toml-edit-js/commit/dffae90))
- refactor: move `from_str` to `parse` for document parsing ([6483cd5](https://github.com/rainbowatcher/toml-edit-js/commit/6483cd5))
- refactor: refactor and clarify array concatenation and stringification logic ([b902ec6](https://github.com/rainbowatcher/toml-edit-js/commit/b902ec6))
- refactor: refactor and optimize data conversion and serialization logic ([4c6e8de](https://github.com/rainbowatcher/toml-edit-js/commit/4c6e8de))
- refactor: refactor decoration functions for clarity and performance ([f4f2a03](https://github.com/rainbowatcher/toml-edit-js/commit/f4f2a03))
- refactor: refactor EditOptions by removing deprecated action field ([b8aa5d9](https://github.com/rainbowatcher/toml-edit-js/commit/b8aa5d9))
- refactor: refactor parameter naming and clean up import statements ([753105e](https://github.com/rainbowatcher/toml-edit-js/commit/753105e))
- refactor: refactor table and array conversions for conciseness ([389bd3d](https://github.com/rainbowatcher/toml-edit-js/commit/389bd3d))
- refactor: refactor type casting and object iteration for safety and clarity ([ccb2e69](https://github.com/rainbowatcher/toml-edit-js/commit/ccb2e69))
- refactor: refactor wrappers to use references for improved memory management ([14b80c2](https://github.com/rainbowatcher/toml-edit-js/commit/14b80c2))
- refactor: refine JavaScript value conversions and object handling ([5079dbd](https://github.com/rainbowatcher/toml-edit-js/commit/5079dbd))
- build: configure WASM build profiles and testing fixtures ([f4b0b3f](https://github.com/rainbowatcher/toml-edit-js/commit/f4b0b3f))
- chore: change visibility of modules ([e0fd83a](https://github.com/rainbowatcher/toml-edit-js/commit/e0fd83a))
- chore: update benchmarks and dependencies for toml-edit-js v0.4 and v0.5 ([360e04b](https://github.com/rainbowatcher/toml-edit-js/commit/360e04b))
- fix: fix shims can't run in browser by update wasmup dependency to version 0.10.1 ([9e2e7e9](https://github.com/rainbowatcher/toml-edit-js/commit/9e2e7e9))
- fix: improve test assertions for strict equality and correctness ([02da875](https://github.com/rainbowatcher/toml-edit-js/commit/02da875))

## <small>0.5.2 (2025-08-10)</small>

- docs: update ([94c6a02](https://github.com/rainbowatcher/toml-edit-js/commit/94c6a02))
- chore: bump deps ([cfdbd1a](https://github.com/rainbowatcher/toml-edit-js/commit/cfdbd1a))
- chore: bump deps ([7ea324a](https://github.com/rainbowatcher/toml-edit-js/commit/7ea324a))
- chore: move ImDocument to Document ([78dcf52](https://github.com/rainbowatcher/toml-edit-js/commit/78dcf52))
- style: adjust indent ([98a383c](https://github.com/rainbowatcher/toml-edit-js/commit/98a383c))
- fix: edit table value using value mut object instead of insert ([1bc1f5c](https://github.com/rainbowatcher/toml-edit-js/commit/1bc1f5c))

## <small>0.5.1 (2025-07-18)</small>

- test: error message update ([4dc2436](https://github.com/rainbowatcher/toml-edit-js/commit/4dc2436))
- test: refactor test cases to reference issue#6 for clarity ([cc7822d](https://github.com/rainbowatcher/toml-edit-js/commit/cc7822d)), closes [issue#6](https://github.com/issue/issues/6) [#6](https://github.com/rainbowatcher/toml-edit-js/issues/6) [issue#6](https://github.com/issue/issues/6)
- chore: add large toml parse bench and adjust code style ([b8cd773](https://github.com/rainbowatcher/toml-edit-js/commit/b8cd773))
- chore: add support for smol-toml parsing and benchmarking ([6b9b7d8](https://github.com/rainbowatcher/toml-edit-js/commit/6b9b7d8))
- chore: bump deps ([e96a53d](https://github.com/rainbowatcher/toml-edit-js/commit/e96a53d))
- chore: remove entry point and release flag configurations ([6cf5307](https://github.com/rainbowatcher/toml-edit-js/commit/6cf5307))
- chore: update toml_edit dependency to latest version ([1d7f254](https://github.com/rainbowatcher/toml-edit-js/commit/1d7f254))
- ci: update CI Node.js versions and matrix configurations ([4dab5de](https://github.com/rainbowatcher/toml-edit-js/commit/4dab5de))

## 0.5.0 (2025-06-22)

- style: apply eslint style rule ([f8e5f29](https://github.com/rainbowatcher/toml-edit-js/commit/f8e5f29))
- style: apply eslint style rule ([3692ded](https://github.com/rainbowatcher/toml-edit-js/commit/3692ded))
- feat: follow previous decoration when insert into table ([98618c7](https://github.com/rainbowatcher/toml-edit-js/commit/98618c7))
- fix: fix path key handling and indentation ([bbda6cf](https://github.com/rainbowatcher/toml-edit-js/commit/bbda6cf))
- fix: fix pattern matching to handle all Item variants explicitly during conversion from JsArray ([68ba3a5](https://github.com/rainbowatcher/toml-edit-js/commit/68ba3a5))
- docs: update edit options type definition ([8008be2](https://github.com/rainbowatcher/toml-edit-js/commit/8008be2))
- test: toml automatic convert timezone ([5009973](https://github.com/rainbowatcher/toml-edit-js/commit/5009973))

## 0.4.0 (2025-06-12)

- perf: refactor and enhance TOML parsing, editing, and benchmarking infrastructure ([d70fccf](https://github.com/rainbowatcher/toml-edit-js/commit/d70fccf))

## 0.3.0 (2025-05-21)

- docs: add README shields, update examples, and improve documentation ([c4d92a6](https://github.com/rainbowatcher/toml-edit-js/commit/c4d92a6))
- chore: bump dev deps ([5ab2246](https://github.com/rainbowatcher/toml-edit-js/commit/5ab2246))
- chore: refactor test setup and add new array editing tests ([01c2899](https://github.com/rainbowatcher/toml-edit-js/commit/01c2899))
- chore: remove useless options in eslint config ([e8338f2](https://github.com/rainbowatcher/toml-edit-js/commit/e8338f2))
- chore: update dependencies and build configurations for consistency ([cb52877](https://github.com/rainbowatcher/toml-edit-js/commit/cb52877))
- chore: update import paths and refine test initialization procedures ([d23eab6](https://github.com/rainbowatcher/toml-edit-js/commit/d23eab6))
- chore(deps): bump deps ([31a3f53](https://github.com/rainbowatcher/toml-edit-js/commit/31a3f53))
- chore(deps): bump deps ([b201689](https://github.com/rainbowatcher/toml-edit-js/commit/b201689))
- chore(deps): bump rust deps ([0130fca](https://github.com/rainbowatcher/toml-edit-js/commit/0130fca))
- style: remove trailing spaces ([9adb8ce](https://github.com/rainbowatcher/toml-edit-js/commit/9adb8ce))
- fix: add empty object to init function to avoid raise warning ([95f4eb0](https://github.com/rainbowatcher/toml-edit-js/commit/95f4eb0))
- fix: update lock file ([a3b4953](https://github.com/rainbowatcher/toml-edit-js/commit/a3b4953))
- fix: wasmup option entries is rename to entry ([c4cf574](https://github.com/rainbowatcher/toml-edit-js/commit/c4cf574))
- ci: add verify wasm-opt install step ([877eafd](https://github.com/rainbowatcher/toml-edit-js/commit/877eafd))
- ci: simplify CI workflow by removing redundant wasm-opt verification ([4c505c5](https://github.com/rainbowatcher/toml-edit-js/commit/4c505c5))
- build: optimize build size ([9c0ce48](https://github.com/rainbowatcher/toml-edit-js/commit/9c0ce48))

## <small>0.2.1 (2024-09-06)</small>

- fix: replace private to publish in cargo project file ([4bb7eb4](https://github.com/rainbowatcher/toml-edit-js/commit/4bb7eb4))
- chore: add license file ([f3d7fb8](https://github.com/rainbowatcher/toml-edit-js/commit/f3d7fb8))
- chore: add vscode config for eslint ([3834759](https://github.com/rainbowatcher/toml-edit-js/commit/3834759))
- chore: apply eslint rule ([0a9ad88](https://github.com/rainbowatcher/toml-edit-js/commit/0a9ad88))
- chore: bump node dev deps ([ff2e8fd](https://github.com/rainbowatcher/toml-edit-js/commit/ff2e8fd))
- chore: bump rust deps ([f10cf9a](https://github.com/rainbowatcher/toml-edit-js/commit/f10cf9a))
- chore: bump wasmup to 0.7.0 ([d461efa](https://github.com/rainbowatcher/toml-edit-js/commit/d461efa))
- chore: remove cargo bump script ([1abc878](https://github.com/rainbowatcher/toml-edit-js/commit/1abc878))
- chore: remove lint staged ([7e21418](https://github.com/rainbowatcher/toml-edit-js/commit/7e21418))
- chore: update eslint config ([8a342b0](https://github.com/rainbowatcher/toml-edit-js/commit/8a342b0))
- chore(deps): bump deps ([70c3531](https://github.com/rainbowatcher/toml-edit-js/commit/70c3531))
- docs: add description ([e18d15e](https://github.com/rainbowatcher/toml-edit-js/commit/e18d15e))
- docs: add description for options ([2085091](https://github.com/rainbowatcher/toml-edit-js/commit/2085091))
- test: add test case ([7ab9606](https://github.com/rainbowatcher/toml-edit-js/commit/7ab9606))

## 0.2.0 (2024-08-01)

- ci: lint only in linux ([19c0d4b](https://github.com/rainbowatcher/toml-edit-js/commit/19c0d4b))
- ci: refactor workflows ([a1f76db](https://github.com/rainbowatcher/toml-edit-js/commit/a1f76db))
- ci: run build before typecheck ([ed08e19](https://github.com/rainbowatcher/toml-edit-js/commit/ed08e19))
- ci: should build before lint ([32f8c5d](https://github.com/rainbowatcher/toml-edit-js/commit/32f8c5d))
- ci: try install pre-requisites ([e1d7c30](https://github.com/rainbowatcher/toml-edit-js/commit/e1d7c30))
- revert: attemp change linebreak-style rule in windows ([4f63ce1](https://github.com/rainbowatcher/toml-edit-js/commit/4f63ce1))
- chore: add changelog ([6d3ba19](https://github.com/rainbowatcher/toml-edit-js/commit/6d3ba19))
- chore: add typecheck script ([9fc1966](https://github.com/rainbowatcher/toml-edit-js/commit/9fc1966))
- chore: attemp change linebreak-style rule in windows ([ee2cfdc](https://github.com/rainbowatcher/toml-edit-js/commit/ee2cfdc))
- chore: bump deps ([102c8fe](https://github.com/rainbowatcher/toml-edit-js/commit/102c8fe))
- chore: bump deps, bump wasmup to 0.5.3 ([72c0490](https://github.com/rainbowatcher/toml-edit-js/commit/72c0490))
- chore: release v0.1.1 ([9963a01](https://github.com/rainbowatcher/toml-edit-js/commit/9963a01))
- chore: remove .npmignore and move commitlint config to package.json ([724cb49](https://github.com/rainbowatcher/toml-edit-js/commit/724cb49))
- chore: update wasmup version ([465779b](https://github.com/rainbowatcher/toml-edit-js/commit/465779b))
- chore: update wasmup version to 0.5.2 ([f802adf](https://github.com/rainbowatcher/toml-edit-js/commit/f802adf))
- test: use dedent for multiline string ([1f896a4](https://github.com/rainbowatcher/toml-edit-js/commit/1f896a4))
- test: use dedent for multiline string ([af3b841](https://github.com/rainbowatcher/toml-edit-js/commit/af3b841))
- feat: add options for edit function ([57f535a](https://github.com/rainbowatcher/toml-edit-js/commit/57f535a))
- refactor: remove build dist ([d3390d0](https://github.com/rainbowatcher/toml-edit-js/commit/d3390d0))

## <small>0.1.1 (2024-08-01)</small>

- chore: add changelog ([6d3ba19](https://github.com/rainbowatcher/toml-edit-js/commit/6d3ba19))
- refactor: remove build dist ([d3390d0](https://github.com/rainbowatcher/toml-edit-js/commit/d3390d0))

## 0.1.0 (2024-08-01)

- chore: add deps ([a49f167](https://github.com/rainbowatcher/toml-edit-js/commit/a49f167))
- chore: add license field ([49dc705](https://github.com/rainbowatcher/toml-edit-js/commit/49dc705))
- chore: add package meta info ([eda39eb](https://github.com/rainbowatcher/toml-edit-js/commit/eda39eb))
- chore: add ts files in packages ([085aae1](https://github.com/rainbowatcher/toml-edit-js/commit/085aae1))
- chore: add wasmup to spell check word ([a2030fb](https://github.com/rainbowatcher/toml-edit-js/commit/a2030fb))
- chore: bump deps ([a6f22eb](https://github.com/rainbowatcher/toml-edit-js/commit/a6f22eb))
- chore: eslint ignore wasm dist dir ([deb9e92](https://github.com/rainbowatcher/toml-edit-js/commit/deb9e92))
- chore: init ([f174cbb](https://github.com/rainbowatcher/toml-edit-js/commit/f174cbb))
- chore: release v0.1.0 ([c431200](https://github.com/rainbowatcher/toml-edit-js/commit/c431200))
- chore: remove build script in scripts ([ea84b5a](https://github.com/rainbowatcher/toml-edit-js/commit/ea84b5a))
- chore: update wasm dist ([850fc88](https://github.com/rainbowatcher/toml-edit-js/commit/850fc88))
- chore: update wasmup config ([2099fbd](https://github.com/rainbowatcher/toml-edit-js/commit/2099fbd))
- perf: adjust publish script ([8f700b0](https://github.com/rainbowatcher/toml-edit-js/commit/8f700b0))
- feat: add commitlint / lint-staged / husky / conventional-changelog and remove unused deps ([bc28999](https://github.com/rainbowatcher/toml-edit-js/commit/bc28999))
- feat: add script to bump version in cargo.toml ([0fe8a0e](https://github.com/rainbowatcher/toml-edit-js/commit/0fe8a0e))
- feat: update dist ([d6283e9](https://github.com/rainbowatcher/toml-edit-js/commit/d6283e9))
- style: apply eslint rules ([08b8132](https://github.com/rainbowatcher/toml-edit-js/commit/08b8132))
- style: update rustfmt config and apply it ([152da10](https://github.com/rainbowatcher/toml-edit-js/commit/152da10))
- docs: update title ([e78f364](https://github.com/rainbowatcher/toml-edit-js/commit/e78f364))
