

use std::fs::{read_dir, remove_file};
use Mac_Residues_Cleaner::helper_library::evaluate_filename_as_mac_residue;
//use crate::helper_library::evaluate_filename_as_mac_residue;



fn list_files(path_str:&str, depth:u32, file_list: &mut Vec<String>, verbose: bool) -> Result<(), Box<dyn std::error::Error>>{

    println!("Listing files in : {path_str} , depth: {depth}");
    let dir_iterator = read_dir(path_str)?;


    for entry in dir_iterator {
        let entry = entry?;
        if verbose{
            println!("{:?}",entry.path());
        }


        if entry.path().is_file() {
            if let Some(filename) = entry.path().file_name() {
                if let Some(filename_str) = filename.to_str() {
                    if verbose{
                        println!("{filename_str}");
                    }

                    let eval_positive = evaluate_filename_as_mac_residue(filename_str);
                    if eval_positive{
                        file_list.push(filename_str.to_string());
                    }
                }
            }

        }
        if entry.path().is_dir() {
            list_files(&entry.path().display().to_string(), depth + 1, file_list, verbose)?;
        }



    }
    Ok(())

}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mac file cleaner v1.0");

    let verbose = std::env::args().any(|arg| arg == "--verbose");

    if verbose{
        println!("Verbose mode on");
    }

    let mut candidate_file_list:Vec<String> = Vec::new();

    list_files(".", 0, &mut candidate_file_list, false)?;

    println!("Candidate files:");
    candidate_file_list.iter().for_each(|filename| println!("{filename}"));

    println!("Do you want to delete them? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.trim().eq("y"){
        candidate_file_list.iter().for_each(|filename| {
            println!("deleting {filename}");
            let del_result = remove_file(filename);
            match del_result {
                Ok(_) => println!("{filename} deleted"),
                Err(e) => println!("{filename} failed to delete: {e}")
            }
        });

    }

    Ok(())
}
