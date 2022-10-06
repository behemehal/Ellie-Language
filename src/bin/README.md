# Ellie Binaries

## Elliec
Elliec is compiler for Ellie files, it contains options for compiling and linking.

### Examples

- ### Compile
    `elliec compile ./code.ei`

    Compile code to ellie binary. Outputs to `./code.eib`

    - ### Options
        - `-b | --binary-version` : Binary version

            EllieC compiles your directory as a package, so you can use this option to give package a version. Default is `1.0.0`.

        - `-c | --module-description` : Module description

            This option is used to give description to module. Default is "A ellie module"

        - `-d | --disable-warnings`: Disable warnings

            This option is used to disable warnings.
        
        - `-i | --insert-module`: Insert module

            This option is used to insert module to current module, example usage is;
            `--insert-module ./ellieStd.eib`. For referencing code in other modules, you can give directory path to this option. For example;
            `--insert-module ./ellieStd.eib=./ellieStdFolder`.

        - `-j | --json-log`: JSON log

            This option is used to enable JSON log.
        - `-m | --module-name`: Module name

            This option is used to give module name. Default is fileName.
        - `-o | --output-type`: Output type

            This option is used to give output type. Default is `bin`, other options are `json` and `depA` for dependency analysis.
        - `-p | --output-path`: Output path

            This option is used to give output path. Default is main file's path.


- ### Module Viewer
    `elliec viewModule ./code.eib`
    
    Parses ellie information from binary. Example output;

    - ### Options
        - `-j | --json-log`: JSON log


    ```
    ModuleName        = test
    ModuleDescription = A ellie package
    ModuleVersion     = 1.0.0
    EllieVersion      = 2.0.0
    InnerModules      =
            ModuleName    = ellie
            ModuleVersion = 1.0.0
    ```
    If binary compiled with old version of ellie, it will show following info;
    ```
    Info: This module is legacy, used ellie_version: 2.0.0 current ellie_version: 3.0.0
    ```

# EllieVM
EllieVM is virtual machine for Ellie binaries, it contains options for running Ellie binaries.

### Examples

- ### Run
    `ellievm run [file] -j -c 64`

    Run code from ellie binary. Outputs to `./code.eib`

    - ### Options
        - `-j | --json-log`: JSON log

            This option is used to enable JSON log.
        - `-a | --arch`: Targeted architecture

            This option is used to give output path. Default is main file's path.

# EllieFMT
EllieFMT is formatter for Ellie files, it contains options for formatting and analyzing Ellie files.

### Examples

- ### Format
    `elliefmt format [file] -j -c 64`

    Run code from ellie binary. Outputs to `./code.eib`

    - ### Options
        - `-j | --json-log`: JSON log

            This option is used to enable JSON log.
        - `-a | --arch`: Targeted architecture

            This option is used to give output path. Default is main file's path.