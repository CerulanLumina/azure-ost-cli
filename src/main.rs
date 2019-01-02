extern crate clap;
#[macro_use]
extern crate human_panic;
extern crate azure_ost_core;
extern crate indicatif;

mod clapargs;
mod cli_app;

use azure_ost_core::{AzureOptions, BGMOptions, ExportMode};

use std::path::PathBuf;

fn main() {
    setup_panic!();
    let matches = clapargs::get_clap_app().get_matches();
    let sqpack = matches.value_of("sqpack")
        .unwrap_or_else(|| argument_fail("sqpack"));
    let threads = matches.value_of("threads")
        .map(|thread_str| {
            use std::str::FromStr;
            usize::from_str(thread_str)
        })
        .unwrap_or(Ok(num_cpus::get()))
        .unwrap_or_else(|_| argument_fail("threads"));
    AzureOptions::new(PathBuf::from(sqpack), threads)
        .and_then(|azure_options| {
            matches.subcommand_matches("all")
                .map(|subc_matches| main_all(subc_matches, azure_options));
            Ok(())
        })
        .unwrap_or_else(|_| {
            eprintln!("Failed to create AzureOST parameters. This may have happened because \
            the FFXIV path is invalid.");
            std::process::exit(2);
        })

//    matches.subcommand_matches("")
}

fn main_all(subc_matches: &clap::ArgMatches, azure_options: AzureOptions) {

    let bgm_options = get_bgm_options(subc_matches);
    println!("Beginning to process all files...");
    azure_ost_core::process_all(azure_options, bgm_options, cli_app::create().as_ref())
        .map(|_| {
            ()
        })
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

fn argument_fail(arg: &str) -> ! {
    eprintln!("An argument was either not provided or failed to validate: {}", arg);
    std::process::exit(1);
}