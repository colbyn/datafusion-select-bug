```shell
$ cargo run
   Compiling datafusion-select-bug v0.1.0 (/Users/colbyn/Developer/Temporary/datafusion-select-bug)
    Finished dev [unoptimized + debuginfo] target(s) in 2.03s
     Running `target/debug/datafusion-select-bug`
thread 'main' panicked at src/main.rs:21:10:
called `Result::unwrap()` on an `Err` value: SchemaError(FieldNotFound { field: Column { relation: None, name: "value" }, valid_fields: [Column { relation: Some(Bare { table: "my_table" }), name: "id" }, Column { relation: Some(Bare { table: "my_table" }), name: " value" }] })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```