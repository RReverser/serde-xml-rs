extern crate docmatic;

#[test]
#[ignore]
fn test_readme() {
    docmatic::assert_file("README.md");
}
