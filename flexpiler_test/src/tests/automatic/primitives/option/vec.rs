

#[test]
fn single_element_none_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[None]"
    );

    let test_vec_option_i32: std::vec::Vec<std::option::Option<i32>> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_option_i32.len(), 1, "Result was expected to be a Vec of len() 1, was instead len() {}", test_vec_option_i32.len());
    match test_vec_option_i32.get(0) {
        Some(&Some(number)) => {
            assert!(false, "Result was expected to have Some(Some(5)) at index 0, was Some(Some({})) instead", number);
        },
        Some(&None) => {
       },
        None => {
            assert!(false, "Result was expected to have Some(5) at index 0, was None instead");
        }
    }
}


#[test]
fn single_element_some_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[Some(5)]"
    );

    let test_vec_option_i32: std::vec::Vec<std::option::Option<i32>> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_option_i32.len(), 1, "Result was expected to be a Vec of len() 1, was instead len() {}", test_vec_option_i32.len());
    match test_vec_option_i32.get(0) {
        Some(&Some(5)) => {},
        Some(&Some(number)) => {
            assert!(false, "Result was expected to have Some(Some(5)) at index 0, was Some(Some({})) instead", number);
        },
        Some(&None) => {
            assert!(false, "Result was expected to have Some(Some(5)) at index 0, was Some(None) instead");
        },
        None => {
            assert!(false, "Result was expected to have Some(5) at index 0, was None instead");
        }
    }
}


#[test]
fn single_element_none_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\n[ \t\nSome \t\n( \t\n5 \t\n) \t\n] \t\n"
    );

    let test_vec_option_i32: std::vec::Vec<std::option::Option<i32>> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_option_i32.len(), 1, "Result was expected to be a Vec of len() 1, was instead len() {}", test_vec_option_i32.len());
    match test_vec_option_i32.get(0) {
        Some(&Some(5)) => {},
        Some(&Some(number)) => {
            assert!(false, "Result was expected to have Some(Some(5)) at index 0, was Some(Some({})) instead", number);
        },
        Some(&None) => {
            assert!(false, "Result was expected to have Some(Some(5)) at index 0, was Some(None) instead");
        },
        None => {
            assert!(false, "Result was expected to have Some(5) at index 0, was None instead");
        }
    }
}

