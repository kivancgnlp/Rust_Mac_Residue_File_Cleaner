

//mod helper_library;

#[test]
fn test_add() {
    let ok = crate::helper_library::evaluate_filename_as_mac_residue(".DS_Store");
    assert_eq!(ok, true);
}



//use helper_library;