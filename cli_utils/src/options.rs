use clap::{Arg, Command, ValueHint};

pub fn generate_elliefmt_options() -> Command<'static> {
    Command::new("EllieFMT")
        .about("Ellie Formatter")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("format")
                .about("Format a file")
                .arg(
                    Arg::new("allowPanics")
                        .help("Allow panics")
                        .short('a')
                        .long("--allow-panics"),
                )
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                ), //FEATURE
                   //.arg(
                   //    Arg::new("formatShortType")
                   //        .help("Convert short types to regular types")
                   //        .short('l')
                   //        .long("--log-level")
                   //),
        )
        .subcommand(
            Command::new("analyze")
                .about("Analyze file")
                .arg(
                    Arg::new("allowPanics")
                        .help("Allow panics")
                        .short('a')
                        .long("--allow-panics"),
                )
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                ),
        )
        .subcommand(
            Command::new("version")
                .about("Get version")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(Arg::new("detailed").short('d').long("--detailed-version")),
        )
}

pub fn generate_elliec_options() -> Command<'static> {
    Command::new("EllieCompiler")
        .about("Ellie Compiler")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("tokenize")
                .about("Tokenize file")
                .arg(
                    Arg::new("allowPanics")
                        .help("Allow panics")
                        .short('a')
                        .long("--allow-panics"),
                )
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(
                    Arg::new("showDebugLines")
                        .help("Show debugging lines")
                        .short('s')
                        .long("--show-debug-lines"),
                )
                .arg(
                    Arg::new("target")
                        .help("Target file to compile")
                        .takes_value(true)
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            Command::new("compile")
                .about("Compile file")
                .arg(
                    Arg::new("targetArchitecture")
                        .help("Targeted architecture for bytecode")
                        .short('c')
                        .long("--arch")
                        .default_values(&["64", "32", "16"])
                        .default_value("64"),
                )
                .arg(
                    Arg::new("performanceInfo")
                        .help("Output performance info")
                        .short('q')
                        .long("-performance-info"),
                )
                .arg(
                    Arg::new("allowPanics")
                        .help("Allow panics")
                        .short('a')
                        .long("--allow-panics"),
                )
                .arg(
                    Arg::new("experimentalFeatures")
                        .help("Allow experimental features")
                        .short('x')
                        .long("--experimental-features"),
                )
                .arg(
                    Arg::new("showDebugLines")
                        .help("Show debugging lines")
                        .short('s')
                        .long("--show-debug-lines"),
                )
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(
                    Arg::new("isLib")
                        .help("Compile as lib")
                        .short('l')
                        .long("-compile-lib"),
                )
                .arg(
                    Arg::new("disableWarnings")
                        .help("Disable warnings")
                        .short('w')
                        .long("-disable-warnings"),
                )
                .arg(
                    Arg::new("excludeStd")
                        .help("Don't import standard library")
                        .short('e')
                        .long("-exclude-std"),
                )
                .arg(
                    Arg::new("insertModule")
                        .help("Insert a module from binary")
                        .short('i')
                        .long("--insert-module")
                        .takes_value(true)
                        .multiple_values(true)
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("binaryVersion")
                        .help("Binary version")
                        .short('b')
                        .long("--binary-version")
                        .default_value("1.0.0")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("description")
                        .help("Description of module")
                        .short('d')
                        .long("--module-description")
                        .default_value("A ellie module")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("moduleName")
                        .help("Name of module")
                        .short('m')
                        .long("--module-name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("outputPath")
                        .help("Output path to write")
                        .short('p')
                        .long("--output-path")
                        .takes_value(true)
                        .value_hint(ValueHint::DirPath),
                )
                .arg(
                    Arg::new("outputType")
                        .help("Output type")
                        .short('o')
                        .long("--output-type")
                        .takes_value(true)
                        .default_value("byteCode"),
                )
                .arg(
                    Arg::new("target")
                        .help("Target file to compile")
                        .takes_value(true)
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            Command::new("viewModule")
                .about("Analyze given module information")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(
                    Arg::new("target")
                        .help("Target module to analyze")
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            Command::new("version")
                .about("Get version")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(Arg::new("detailed").short('d').long("--detailed-version")),
        )
}

pub fn generate_ellievm_options() -> Command<'static> {
    Command::new("EllieVM")
        .about("Ellie Virtual Machine")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Run compiled program")
                .arg(
                    Arg::new("targetArchitecture")
                        .help("Targeted architecture for bytecode")
                        .short('c')
                        .long("--arch")
                        .default_values(&["64", "32", "16"])
                        .default_value("64"),
                )
                .arg(
                    Arg::new("debugInfo")
                        .help("Supply debug info file for more information")
                        .short('d')
                        .long("--debug-info")
                        .takes_value(true)
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("heapDump")
                        .help("Dump heap to file")
                        .short('u')
                        .long("--heap-dump"),
                )
                .arg(
                    Arg::new("vmDebug")
                        .help("Run vm slower and print more information")
                        .short('v')
                        .long("-vm-debug"),
                )
                .arg(
                    Arg::new("allowPanics")
                        .help("Allow panics")
                        .short('a')
                        .long("--allow-panics"),
                )
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(
                    Arg::new("target")
                        .help("Target assembly to compile")
                        .takes_value(true)
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            Command::new("version")
                .about("Get version")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(Arg::new("detailed").short('d').long("--detailed-version")),
        )
}
