#[test]
fn macro_empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "vec![]"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert!(test_vec_i32.is_empty(), "Result was expected as empty Vec<i32>, was not empty.");
}


#[test]
fn macro_empty_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nvec![ \t\n] \t\n"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert!(test_vec_i32.is_empty(), "Result was expected as empty Vec<i32>, was not empty.");
}


#[test]
fn new_min_declaration_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Vec::new()"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert!(test_vec_i32.is_empty(), "Result was expected as empty Vec<i32>, was not empty.");
}


#[test]
fn new_min_declaration_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nVec::new( \t\n) \t\n"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert!(test_vec_i32.is_empty(), "Result was expected as empty Vec<i32>, was not empty.");
}


#[test]
fn new_max_declaration_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "std::vec::Vec::new()"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert!(test_vec_i32.is_empty(), "Result was expected as empty Vec<i32>, was not empty.");
}


#[test]
fn new_max_declaration_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nstd::vec::Vec::new \t\n( \t\n) \t\n"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert!(test_vec_i32.is_empty(), "Result was expected as empty Vec<i32>, was not empty.");
}
