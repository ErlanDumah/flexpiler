use super::TestEnum;


#[test]
fn empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[]"
    );
    let test_vec_automatic_test_struct: std::vec::Vec<TestEnum> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert!(test_vec_automatic_test_struct.is_empty(), "Result was expected to be empty.");
}


#[test]
fn single_element_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[TestEnum::NoData]"
    );
    let test_vec_automatic_test_struct: std::vec::Vec<TestEnum> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_automatic_test_struct.len(), 1, "Result was expected as Vec<AutomaticTestStruct> with len() 1, instead had length {}", test_vec_automatic_test_struct.len());
    match test_vec_automatic_test_struct.get(0) {
        Some(&TestEnum::NoData) => {
        },
        Some(_) => {
            assert!(false, "Result was expected to have Some(TestEnum::NoData) at index 0, was Some(...) instead");
        }
        None => {
            assert!(false, "Result was expected to have Some(...) at index 0, was None instead");
        }
    }
}


#[test]
fn second_element_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[TestEnum::NoData, TestEnum::DataEmpty(),]"
    );
    let test_vec_automatic_test_struct: std::vec::Vec<TestEnum> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_automatic_test_struct.len(), 2, "Result was expected as Vec<AutomaticTestStruct> with len() 2, instead had length {}", test_vec_automatic_test_struct.len());
    match test_vec_automatic_test_struct.get(0) {
        Some(&TestEnum::NoData) => {
        },
        Some(_) => {
            assert!(false, "Result was expected to have Some(TestEnum::NoData) at index 0, was Some(...) instead");
        }
        None => {
            assert!(false, "Result was expected to have Some(...) at index 0, was None instead");
        }
    }
    match test_vec_automatic_test_struct.get(1) {
        Some(&TestEnum::DataEmpty()) => {
        },
        Some(_) => {
            assert!(false, "Result was expected to have Some(TestEnum::DataEmpty()) at index 1, was Some(...) instead");
        }
        None => {
            assert!(false, "Result was expected to have Some(...) at index 1, was None instead");
        }
    }
}
