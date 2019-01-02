use ::azure_ost_core::callbacks::*;
use ::indicatif::{ProgressBar, ProgressStyle};
use std::cell::RefCell;

struct CLIOutput {
    pub progress_bar: Option<ProgressBar>
}

struct CLICallbacks {
    output: RefCell<CLIOutput>,
}

impl CLICallbacks {
    pub fn new() -> CLICallbacks {
        CLICallbacks{output: RefCell::new(CLIOutput{progress_bar: None})}
    }
}

fn get_pb_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
}

impl AzureCallbacks for CLICallbacks {
    fn pre_phase(&self, phase: AzureProcessPhase) {
        match phase {
            AzureProcessPhase::Hashing => println!("Calculating file hashes for comparison or saving..."),
            AzureProcessPhase::Exporting => println!("Exporting selected files..."),
            _ => (),
        };
    }
    fn post_phase(&self, phase: AzureProcessPhase) {
        match phase {
            AzureProcessPhase::Begin => println!("Beginning process."),
            AzureProcessPhase::ReadingBGMSheet => println!("Finished reading BGM datasheet."),
            AzureProcessPhase::Hashing => println!("Finished calculating file hashes."),
            AzureProcessPhase::Collecting => println!("Determined which files to operate on."),
            AzureProcessPhase::SavingManifest => println!("Saved file manifest"),
            AzureProcessPhase::Exporting => println!("Completed export of all files."),
        };
    }

    fn process_begin(&self, info: AzureProcessBegin) {
        if self.output.borrow().progress_bar.is_some() {
            self.output.borrow().progress_bar.as_ref().unwrap().set_length(info.total_operations_count as u64);
            self.output.borrow().progress_bar.as_ref().unwrap().set_position(0);
        } else {
            self.output.borrow_mut().progress_bar = Some(ProgressBar::new(info.total_operations_count as u64));
            self.output.borrow_mut().progress_bar.as_ref().unwrap().set_style(get_pb_style());
        }
    }
    fn process_progress(&self, info: AzureProcessProgress) {
        if self.output.borrow().progress_bar.is_some() {
            self.output.borrow().progress_bar.as_ref().unwrap().set_position(info.operations_progress as u64);
        }
    }
    fn process_nonfatal_error(&self, info: AzureProcessNonfatalError) {
        if self.output.borrow().progress_bar.is_some() {
            self.output.borrow().progress_bar.as_ref().unwrap().println(format!("An error occurred: {}", info.reason));
        }
    }
    fn process_complete(&self, _info: AzureProcessComplete) {
        if self.output.borrow().progress_bar.is_some() {
            self.output.borrow().progress_bar.as_ref().unwrap().finish();
            self.output.borrow_mut().progress_bar = None;
        }
    }
}

pub fn create() -> Box<AzureCallbacks> {
    Box::new(CLICallbacks::new())
}