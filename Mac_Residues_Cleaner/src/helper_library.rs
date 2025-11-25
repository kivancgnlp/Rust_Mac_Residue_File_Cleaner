

pub(crate) fn evaluate_filename_as_mac_residue(filename:&str) -> bool{

    if ".DS_Store".eq(filename){
        return true;
    }

    if filename.starts_with("."){
        println!("{filename} is possible a mac residue");
    }

    

    false
}