use clap::Parser;

#[derive(Parser)]
#[command(
    name = "spring-tui",
    version = env!("CARGO_PKG_VERSION"),
    author = "hyunwo.oh@turntabl.io",
    about = "tui for generating Spring Boot projects using Spring Initializr",
    long_about = "A fast and interactive terminal-based tool for generating Spring Boot projects. \n\
                  provides both a rich tui interface and cli options for automated workflows. \n\
                  export configurations to yaml/json/toml and generate projects from config files."
)]
pub struct Args {
    /// Path to configuration file (YAML, JSON, or TOML) to generate project from
    #[arg(
        short,
        long,
        value_name = "path-to-config",
        help = "Generate project from config file",
        long_help = "Specify a configuration file in yaml, json, or toml format to automatically \
                     generate a Spring Boot project with predefined settings"
    )]
    pub file: Option<String>,

    // /// Directory path for downloading generated projects
    // #[arg(
    //     short,
    //     long,
    //     value_name = "DIR",
    //     help = "Set default download directory",
    //     long_help = "Override the default download directory for generated Spring Boot projects. \
    //                  The generated ZIP file will be saved to this location"
    // )]
    // dir: Option<String>,

    /// Automatically extract the generated project ZIP file
    #[arg(
        short,
        long,
        help = "Extract project after generation",
        long_help = "Automatically extract the downloaded ZIP file into the target directory. \
                     only applies when generating from a config file (--file option)"
    )]
    pub extract: bool,
}


#[derive(Default)]
pub struct CliResult {
    pub file: Option<String>,
    pub extract: Option<bool>,
}

pub fn parse() -> Option<CliResult> {
    let args = Args::parse();

    if args.file.is_none() {
        return None;
    }

    let extract = args.file.as_ref().map(|_| args.extract);

    Some(CliResult {
        file: args.file,
        extract,
    })
}

