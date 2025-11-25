

//use helper_library::evaluate_filename_as_mac_residue;

use Mac_Residues_Cleaner::helper_library::evaluate_filename_as_mac_residue;

#[test]
fn test_add() {
    let ok = evaluate_filename_as_mac_residue(".DS_Store");
    assert_eq!(ok, true);
}



//use helper_library;