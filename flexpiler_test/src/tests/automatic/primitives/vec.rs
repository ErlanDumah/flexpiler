

#[test]
fn i32_empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[]"
    );

    let test_result: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert!(test_result.is_empty(), "result was not an empty vector.");
}


#[test]
fn string_empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[]"
    );

    let test_result: std::vec::Vec<std::string::String> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert!(test_result.is_empty(), "result was not an empty vector.");
}

