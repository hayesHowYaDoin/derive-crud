use crud_test::database_test;

#[database_test]
fn create() {
    let t = trybuild::TestCases::new();
    t.pass("tests/create/pass/test.rs");
    t.compile_fail("tests/create/attribute_id_none/test.rs");
    t.compile_fail("tests/create/attribute_id_multiple/test.rs");
    t.compile_fail("tests/create/attribute_table_none/test.rs");
    t.compile_fail("tests/create/attribute_table_multiple/test.rs");
    t.compile_fail("tests/create/attribute_table_nonexistant/test.rs");
    t.compile_fail("tests/create/attribute_table_no_name/test.rs");
}
