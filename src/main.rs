use std::path::PathBuf;

use jsplusplus::JSPlusPlus;

fn main() {
    let path = PathBuf::from("F:/projects/js++/test.js");

    let jsplusplus = JSPlusPlus::new();

    if let Err(e) = jsplusplus.run_file(path) {
        println!("Error: {}", e);
    }
}
