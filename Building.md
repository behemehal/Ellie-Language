# Building and running Ellie

Please note ellie is in active **alpha** stage and is not ready for use. Planed runtime postponed due to [#55](https://github.com/behemehal/Ellie-Language/issues/55), but after (12.04.2021) runtime work is started. You can follow progress in [blog](https://ellie.behemehal.net/blog.html) or you can join our community on [Discord](https://discord.gg/EqVh4T959N)


## Building ellie compiler
Ellie compiler(`elliec`) is located at `./src/bin/elliec.rs`. You can build by `cargo build` command. This will output executable to `./target/debug/elliec[?.exe]`.

### Parsing code

By default `elliec` does not import std types to parser. You can find ellieStd at [Releases]() or you can compile by `elliec compile ./lib/ellie.ei -m ellieStd -p ./` command. This will output compiled binary at active directory. Then you can import ellieStd by `elliec compile ./target.ei -m MyPackage -i ./ellie.bin`