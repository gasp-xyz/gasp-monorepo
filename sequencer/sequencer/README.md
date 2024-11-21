### Simple demo app to showcase GASP rust bindings capabilities

Fetching metatadata
```bash
subxt metadata -f bytes --allow-insecure --url ws://127.0.0.1:9944
```

Generating binding
```bash
subxt codegen --file metadata.scale | rustfmt --edition=2018 --emit=stdout > src/gasp/gasp_bindings.rs
```

