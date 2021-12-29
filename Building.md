# Building and running Ellie

Please note ellie is in active **alpha** stage and is not ready for use. Planed runtime postponed due to [#55](https://github.com/behemehal/Ellie-Language/issues/55), but after (12.04.2021) runtime work is started. You can follow progress in [blog](https://ellie.behemehal.net/blog.html) or you can join our community on [Discord](https://discord.gg/EqVh4T959N)

## Building ellie
Building by `cargo build` command will output executable binary [bin\ellie](./src/bin/ellie.rs) in `/target` directory. You can remove `default-features` feature to build without `std` usage.


## Parsing ellie code

```
elliec ./file.ei --set-version 1.0.0 --output json --output-path ./output_file.json
```

This command will save parsed code as `output_file.json`.