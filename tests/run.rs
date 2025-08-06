use crud_test::database_test;

#[database_test]
fn create() {
    let t = trybuild::TestCases::new();
    t.pass("tests/create/expected/test.rs");
    t.compile_fail("tests/create/attribute_id_multiple/test.rs");
    t.compile_fail("tests/create/attribute_id_none/test.rs");
    t.compile_fail("tests/create/attribute_table_multiple/test.rs");
    t.compile_fail("tests/create/attribute_table_no_name/test.rs");
    t.compile_fail("tests/create/attribute_table_none/test.rs");
    t.compile_fail("tests/create/attribute_table_nonexistant/test.rs");
    t.compile_fail("tests/create/bad_schema/test.rs");
}

#[database_test]
fn read() {
    let t = trybuild::TestCases::new();
    t.pass("tests/read/expected/test.rs");
    t.compile_fail("tests/read/attribute_id_multiple/test.rs");
    t.compile_fail("tests/read/attribute_id_none/test.rs");
    t.compile_fail("tests/read/attribute_table_multiple/test.rs");
    t.compile_fail("tests/read/attribute_table_no_name/test.rs");
    t.compile_fail("tests/read/attribute_table_none/test.rs");
    t.compile_fail("tests/read/attribute_table_nonexistant/test.rs");
    t.compile_fail("tests/read/bad_schema/test.rs");
}

#[database_test]
fn read_all() {
    let t = trybuild::TestCases::new();
    t.pass("tests/read_all/expected/test.rs");
    t.pass("tests/read_all/attribute_id_multiple/test.rs");
    t.pass("tests/read_all/attribute_id_one/test.rs");
    t.compile_fail("tests/read_all/attribute_table_multiple/test.rs");
    t.compile_fail("tests/read_all/attribute_table_no_name/test.rs");
    t.compile_fail("tests/read_all/attribute_table_none/test.rs");
    t.compile_fail("tests/read_all/attribute_table_nonexistant/test.rs");
    t.compile_fail("tests/read_all/bad_schema/test.rs");
}
