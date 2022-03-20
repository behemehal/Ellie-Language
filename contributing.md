# Contributing Ellie

For enhancement's you should write your idea to our message channels for approval we don't want you to waste your time if your enhancement does not fit with the project.

For security fixes please inform us about vulnerability(s) before you create public review. You can use `info@behemehal.net` mail.

Thank you

## Development Tips

- If you're getting strange syntax error add `--show-debug-lines` to your `compiler` or `tokenizer` command.
- If internal error occurs, use `--allow-panics` option. You can get rust panics this way.
- For disabling syntax warnings use `--disable-warnings` option.
- For viewing binary module information you can use `viewModule` command. Here is a example output:
```
ModuleName        = ellie
ModuleDescription = A ellie module
ModuleVersion     = 1.0.0
EllieVersion      = 3.0.0
```
- If you're working on issue please create a new issue in [EllieNonWorks Repo](https://github.com/behemehal/EllieNonWorks). Submiting issue with id will help us to identify the issue.

For more info about Ellie please visit [Ellie Developer Handbook](DeveloperHandbook.md).