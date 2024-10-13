These are bin targets runnable as web workers.

Note that you need to add new workers to index.html like how the cargo bin target `worker` has been added below
```
<head>
  <link data-trunk rel="rust" data-wasm-opt="z" data-type="worker" data-bin="worker" data-weak-refs data-loader-shim />
  <link data-trunk rel="rust" data-wasm-opt="z" data-bin="ui" />

</head>

```
