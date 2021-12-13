# Building and running Ellie

Please note ellie is in active **alpha** stage and is not ready for use. Planed runtime postponed due to [#55](https://github.com/behemehal/Ellie-Language/issues/55), but after (12.04.2021) runtime work is started. You can follow progress in [blog](https://ellie.behemehal.net/blog.html) or you can join our community on [Discord](https://discord.gg/EqVh4T959N)

## Building ellie
Building by `cargo build` command will output executable binary [bin\ellie](./src/bin/ellie.rs) in `/target` directory. You can remove `default-features` feature to build without `std` usage.


## Parsing ellie code

```
ellie ./file.ei --render-tokenized
```

This command will output tokenized elements in `file.ei` as json in `file_tokenized.json`.

```
ellie ./file.ei --render-parsed
```
This command will output parsed elements in `file.ei` as json in `file_parsed.json`.