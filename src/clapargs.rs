use ::clap::{Arg, App, SubCommand, ArgGroup};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

pub fn get_clap_app<'a, 'b>() -> App<'a, 'b> {

    App::new(NAME)

        .version(VERSION)
        .about(DESCRIPTION)
        .author(AUTHORS)
        .subcommand(SubCommand::with_name("one")
            .about("Compares/exports a single file")
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Sets the internal SCD file to export by its name \
                 - e.g. music/ffxiv/BGM_Con_Bahamut_Bigboss1.scd")
                .takes_value(true))
            .arg(Arg::with_name("index")
                .short("i")
                .long("index")
                .value_name("INDEX")
                .help("Sets the internal SCD file to export \
                  - e.g. for music/ffxiv/BGM_Con_Bahamut_Bigboss1.scd, 236. \
                  This value can be found in the BGM.csv sheet that is \
                  exported using azure-ost util bgm-csv")
                .takes_value(true))
            .group(ArgGroup::with_name("method")
                .args(&["name", "index"])
                .required(true))
            .arg(Arg::with_name("save")
                .long("save")
                .short("s")
                .value_name("JSON_FILE")
                .takes_value(true)
                .help("Sets the JSON file to save to that can be later \
                 used by --compare. Saves indices, names, and SHA-1 hashes."))
            .arg(Arg::with_name("compare")
                .long("compare")
                .short("c")
                .value_name("JSON_FILE")
                .takes_value(true)
                .help("Sets the JSON file to read from when determining \
                if the file is new or not. Checks internal index, name, and \
                SHA-1 hash."))
            .arg(Arg::with_name("export-ogg")
                .long("export-ogg")
                .short("o")
                .value_name("OUTPUT_FOLDER")
                .takes_value(true)
                .help("Exports the selected file in OGG/Vorbis format to the \
                 specified output folder. If used with --compare, will only export \
                 if the file is selected as new/changed."))
            .arg(Arg::with_name("export-mp3")
                .long("export-mp3")
                .short("m")
                .value_name("OUTPUT_FOLDER")
                .takes_value(true)
                .help("Exports the selected file in MP3 format to the \
                 specified output folder. If used with --compare, will only export \
                  if the file is selected as new/changed."))
            .group(ArgGroup::with_name("export")
                .args(&["export-ogg", "export-mp3"])
            )
            .after_help("Compares, saves hashes, or exports a single file")
        )
        .subcommand(SubCommand::with_name("all")
            .about("Compares/exports all files")
            .arg(Arg::with_name("save")
                .long("save")
                .short("s")
                .value_name("JSON_FILE")
                .takes_value(true)
                .help("Sets the JSON file to save to that can be later \
                 used by --compare. Saves indices, names, and SHA-1 hashes."))
            .arg(Arg::with_name("compare")
                .long("compare")
                .short("c")
                .value_name("JSON_FILE")
                .takes_value(true)
                .help("Sets the JSON file to read from when determining \
                if the files are new or not. Checks internal indices, names, and \
                 SHA-1 hashes."))
            .arg(Arg::with_name("export-ogg")
                .long("export-ogg")
                .short("o")
                .value_name("OUTPUT_FOLDER")
                .takes_value(true)
                .help("Exports the selected file in OGG/Vorbis format to the \
                 specified output folder. If used with --compare, will only export \
                  if the file is selected as new/changed."))

            .arg(Arg::with_name("export-mp3")
                .long("export-mp3")
                .short("m")
                .value_name("OUTPUT_FOLDER")
                .takes_value(true)
                .help("Exports the selected file in MP3 format to the \
                 specified output folder. If used with --compare, will only export \
                  if the file is selected as new/changed."))
            .group(ArgGroup::with_name("export")
                .args(&["export-ogg", "export-mp3"])
            )
            .after_help("Initially selects all files, which can be \
             refined by --compare and exported with --export-ogg or --export-mp3")

        )
        .subcommand(SubCommand::with_name("util")
            .about("Utility commands for working with BGM")
            .subcommand(SubCommand::with_name("bgm-csv")
                .about("Exports the BGM internal sheet as CSV format")
                .arg(Arg::with_name("csv-file")
                    .short("c")
                    .long("csv-file")
                    .required(true)
                    .help("Sets the output CSV file")
                    .takes_value(true)
                    .value_name("CSV_FILE")
                )
            )
        )
        .arg(Arg::with_name("sqpack")
            .short("q")
            .long("sqpack")
            .required(true)
            .value_name("FFXIV_SQPACK_PATH")
            .takes_value(true)
            .help("Sets the FFXIV sqpack folder. This argument may be skipped if the FFXIV_SQPACK_PATH environment variable is set.")
            .env("FFXIV_SQPACK_PATH")
        )
        .arg(Arg::with_name("threads")
            .long("threads")
            .required(false)
            .value_name("THREAD_COUNT")
            .help("Sets the thread count when doing expensive processes. If omitted, uses the number of logical cores on the system.")
        )
}