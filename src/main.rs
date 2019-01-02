extern crate clap;
#[macro_use]
extern crate human_panic;
extern crate azure_ost_core;
extern crate indicatif;

mod clapargs;
mod cli_app;

use azure_ost_core::{AzureOptions, BGMOptions, ExportMode};
use ::azure_ost_core::selector::Selector;

use std::path::PathBuf;

use std::str::FromStr;

fn main() {
    setup_panic!();
    let matches = clapargs::get_clap_app().get_matches();
    let sqpack = matches.value_of("sqpack")
        .unwrap_or_else(|| argument_fail("sqpack"));
    let threads = matches.value_of("threads")
        .map(|thread_str| {

            usize::from_str(thread_str)
        })
        .unwrap_or(Ok(num_cpus::get()))
        .unwrap_or_else(|_| argument_fail("threads"));
    let subcommand = matches.subcommand_name().unwrap_or_else(|| {
        println!("No subcommand specified.");
        std::process::exit(0);
    });
    AzureOptions::new(PathBuf::from(sqpack), threads)
        .and_then(|azure_options| {
            match subcommand {
                "all" => {
                    let subc_matches = matches.subcommand_matches("all")
                        .expect("all subcommand was specified but Option was None!");
                    main_all(subc_matches, azure_options);
                },
                "one" => {
                    let subc_matches = matches.subcommand_matches("one")
                        .expect("one subcommand was specified but Option was None!");
                    main_one(subc_matches, azure_options);
                },
                "util" => {
                    let subc_matches = matches.subcommand_matches("util")
                        .expect("one subcommand was specified but Option was None!");
                    main_util(subc_matches, azure_options);
                }
                other => {
                    eprintln!("Unknown subcommand: {}", other);
                    std::process::exit(1);
                }
            }
            Ok(())
        })
        .unwrap_or_else(|_| {
            eprintln!("Failed to create AzureOST parameters. This may have happened because \
            the FFXIV path is invalid.");
            std::process::exit(2);
        })

//    matches.subcommand_matches("")
}

fn main_util(subc_matches: &clap::ArgMatches, azure_options: AzureOptions) {
    let util_subcommand = subc_matches.subcommand_name()
        .unwrap_or_else(|| {
            println!("No subcommand specified.");
            std::process::exit(0);
        });
    match util_subcommand {
        "bgm-csv" => {
            let bgm_csv_matches = subc_matches.subcommand_matches("bgm-csv")
                .expect("bgm-csv subcommand was specified but Option was None!");
            let csv_out = bgm_csv_matches.value_of("csv-file")
                .unwrap_or_else(|| {
                    argument_fail("csv-out");
                });
            let csv_pathbuf = PathBuf::from(csv_out);
            azure_ost_core::bgm_csv(azure_options, csv_pathbuf)
                .unwrap_or_else(|err| {
                    eprintln!("An error occurred while creating the BGM CSV.");
                    use azure_ost_core::errors::AzureError;
                    match err {
                        AzureError::UnableToCreateSaveFile => {
                            eprintln!("The error occurred when trying to write or create the file.");
                        },
                        AzureError::FFXIVError(ff_err) => {
                            eprintln!("The error occurred when interfacing with the FFXIV data: {:?}", ff_err);
                        },
                        err => {
                            eprintln!("An unknown error occurred: {:?}", err);
                        }
                    }
                    std::process::exit(-1);
                });
            println!("Wrote BGM CSV to {}", csv_out);
            std::process::exit(0);
        },
        other => {
            eprintln!("Unknown subcommand: {}", other);
            std::process::exit(1);
        }
    }
}

fn main_one(subc_matches: &clap::ArgMatches, azure_options: AzureOptions) {
    let name = subc_matches.value_of("name");
    let index = subc_matches.value_of("index");
    let selected = option_name_index(name, index);
    let bgm_options = get_bgm_options(subc_matches);
    println!("Beginning to process selected file...");

    azure_ost_core::process_one(selected.as_ref(), azure_options, bgm_options, cli_app::create().as_ref())
        .unwrap_or_else(|err| {
            eprintln!("An error occurred while attempting to process. {:?}", err);
            std::process::exit(-1);
        });

}

fn main_all(subc_matches: &clap::ArgMatches, azure_options: AzureOptions) {

    let bgm_options = get_bgm_options(subc_matches);
    println!("Beginning to process all files...");
    azure_ost_core::process_all(azure_options, bgm_options, cli_app::create().as_ref())
        .unwrap_or_else(|err| {
            eprintln!("An error occurred while attempting to process. {:?}", err);
            std::process::exit(-1);
        });

}

fn get_bgm_options(subc_matches: &clap::ArgMatches) -> BGMOptions {
    let emp3 = subc_matches.value_of("export-mp3");
    let eogg = subc_matches.value_of("export-ogg");
    let emp3= emp3.map(|mp3| {
        if cfg!(feature="lamemp3") {
            mp3
        } else {
            eprintln!("MP3 encoding is not enabled.");
            argument_fail("export-mp3");
        }
    });
    let export_mode = option_exports_into_mode(emp3, eogg);
    let compare_file = subc_matches.value_of("compare").map(PathBuf::from);
    let save_file = subc_matches.value_of("save").map(PathBuf::from);
    BGMOptions::new(save_file, compare_file, export_mode)
        .unwrap_or_else(|err| {
            eprintln!("Failed to create BGM Options. {:?}", err);
            std::process::exit(2);
        })
}

#[cfg(feature = "lamemp3")]
fn get_mp3_mode(pb: PathBuf) -> ExportMode {
    ExportMode::MP3(pb)
}

#[cfg(not(feature = "lamemp3"))]
fn get_mp3_mode(_: PathBuf) -> ExportMode {
    panic!("Should not have tried to get an MP3 mode when it is not enabled")
}

fn option_exports_into_mode(emp3: Option<&str>, eogg: Option<&str>) -> Option<ExportMode> {
    if emp3.is_some() {

        let pb = PathBuf::from(emp3.expect("Checked if emp3 was some but was not some!"));
        if pb.exists() && !pb.is_dir() {
            eprintln!("The provided export directory already exists as a file! {:?}", pb.as_os_str());
            argument_fail("export-mp3");
        }
        Some(get_mp3_mode(pb))


    } else if eogg.is_some() {

        let pb = PathBuf::from(eogg.expect("Checked if eogg was some but was not some!"));
        if pb.exists() && !pb.is_dir() {
            eprintln!("The provided export directory already exists as a file! {:?}", pb.as_os_str());
            argument_fail("export-ogg");
        }
        Some(ExportMode::OGG(pb))

    } else {
        None
    }
}

fn option_name_index(name: Option<&str>, index: Option<&str>) -> Box<Selector> {
    let name = name.map(String::from);
    let index = index.map(usize::from_str);
    if name.is_some() {
        Box::new(name.expect("Checked if name was some but was not some!"))
    } else if index.is_some() {
        let index_res = index.expect("Checked if index was some but was not some!");
        let s_index = index_res.unwrap_or_else(|_| {
            argument_fail("index");
        });
        Box::new(s_index)
    } else {
        eprintln!("Neither name nor index was specified!");
        std::process::exit(1);
    }

}

fn argument_fail(arg: &str) -> ! {
    eprintln!("An argument was either not provided or failed to validate: {}", arg);
    std::process::exit(1);
}