use flexpiler::common::reader::transcribe::Trait;


#[test]
fn empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[]"
    );
    let mut transcribe_reader = reader::Transcribe::from(reader);

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut transcribe_reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization at {},{}:\n{}",
                    transcribe_reader.line_count(),
                    transcribe_reader.char_count(),
                    error);
            return;
        }
    };

    assert!(test_vec_i32.is_empty(), "Result was expected as empty Vec<i32>, was not empty.");
}


#[test]
fn empty_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\n[ \t\n] \t\n"
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
fn single_element_positive_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[5]"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_vec_i32.len(), 1, "Result was expected to be a Vec of len() 1, was instead len() {}", test_vec_i32.len());
    match test_vec_i32.get(0) {
        Some(&5) => {},
        Some(number) => {
            assert!(false, "Result was expected to have Some(5) at index 0, was Some({}) instead", number);
        },
        None => {
            assert!(false, "Result was expected to have Some(5) at index 0, was None instead");
        }
    }
}


#[test]
fn single_element_positive_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\n[ \t\n5 \t\n] \t\n"
    );

    let test_vec_i32: std::vec::Vec<i32> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_i32.len(), 1, "Result was expected to be a Vec of len() 1, was instead len() {}", test_vec_i32.len());
    match test_vec_i32.get(0) {
        Some(&5) => {},
        Some(number) => {
            assert!(false, "Result was expected to have Some(5) at index 0, was Some({}) instead", number);
        },
        None => {
            assert!(false, "Result was expected to have Some(5) at index 0, was None instead");
        }
    }
}
