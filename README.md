## Test for Spin's new component deps feature

1. Clone this and https://github.com/itowlson/vscode-wasm-component-model-sample into sibling directories.  (Important for relative paths to work.)
2. Go into `vscode-wasm-component-model-sample/calculator-impl` and run `cargo component build --release --target wasm32-unknown-unknown`
3. Go into this repo and run `spin up --build`.  When I do this, I get:

```
Error: Failed to instantiate component 'spin-comp-deps-test'

Caused by:
   0: composing component "spin-comp-deps-test"
   1: dependency 'vscode:example/types' doesn't export 'vscode:example/types' to satisfy import 'vscode:example/types'
```

4. Uncomment the `spin.toml` line `# dependencies_inherit_configuration = true` and rerun `spin up`. When I do this, it works.

Tested using https://github.com/fermyon/spin/pull/2673 at commit `9cba0fc6b9bd190615042fc4a748bd45e1a87fef`
