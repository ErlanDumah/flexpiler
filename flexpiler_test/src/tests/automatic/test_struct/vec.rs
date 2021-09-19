use super::AutomaticTestStruct;


#[test]
fn empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "[]"
    );
    let test_vec_automatic_test_struct: std::vec::Vec<AutomaticTestStruct> = match std::vec::Vec::deserialize(&mut reader) {
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
        "[AutomaticTestStruct{a_string:\"Hello\",a_i32:-34,a_usize:50,a_sub:AutomaticSubTestStruct{a_usize:67}}]"
    );
    let test_vec_automatic_test_struct: std::vec::Vec<AutomaticTestStruct> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_automatic_test_struct.len(), 1, "Result was expected as Vec<AutomaticTestStruct> with len() 1, instead had length {}", test_vec_automatic_test_struct.len());
    match test_vec_automatic_test_struct.get(0) {
        Some(automatic_test_struct_ref) => {
            assert_eq!(automatic_test_struct_ref.a_usize, 50, "Result was expected to have field a_usize 5, had {} instead", automatic_test_struct_ref.a_usize);
            assert_eq!(automatic_test_struct_ref.a_string.as_str(), "Hello", "Result was expected to have field a_string \"Hello\", had {} instead", automatic_test_struct_ref.a_string);
            assert_eq!(automatic_test_struct_ref.a_i32, -34, "Result was expected to have field a_i32 -34, had {} instead", automatic_test_struct_ref.a_i32);
            assert_eq!(automatic_test_struct_ref.a_sub.a_usize, 67, "Result was expected to have field a_sub.a_usize 67, had {} instead", automatic_test_struct_ref.a_sub.a_usize);
        },
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
        "[AutomaticTestStruct{a_string:\"Hello\",a_i32:-34,a_usize:50,a_sub:AutomaticSubTestStruct{a_usize:67}},AutomaticTestStruct{a_string:\"Yallow\",a_i32:-24,a_usize:60,a_sub:AutomaticSubTestStruct{a_usize:19}},]"
    );
    let test_vec_automatic_test_struct: std::vec::Vec<AutomaticTestStruct> = match std::vec::Vec::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}",
                    error);
            return;
        }
    };

    assert_eq!(test_vec_automatic_test_struct.len(), 2, "Result was expected as Vec<AutomaticTestStruct> with len() 2, instead had length {}", test_vec_automatic_test_struct.len());
    match test_vec_automatic_test_struct.get(0) {
        Some(automatic_test_struct_ref) => {
            assert_eq!(automatic_test_struct_ref.a_usize, 50, "Result was expected to have field a_usize 5, had {} instead", automatic_test_struct_ref.a_usize);
            assert_eq!(automatic_test_struct_ref.a_string.as_str(), "Hello", "Result was expected to have field a_string \"Hello\", had {} instead", automatic_test_struct_ref.a_string);
            assert_eq!(automatic_test_struct_ref.a_i32, -34, "Result was expected to have field a_i32 -34, had {} instead", automatic_test_struct_ref.a_i32);
            assert_eq!(automatic_test_struct_ref.a_sub.a_usize, 67, "Result was expected to have field a_sub.a_usize 67, had {} instead", automatic_test_struct_ref.a_sub.a_usize);
        },
        None => {
            assert!(false, "Result was expected to have Some(...) at index 0, was None instead");
        }
    }
    match test_vec_automatic_test_struct.get(1) {
        Some(automatic_test_struct_ref) => {
            assert_eq!(automatic_test_struct_ref.a_usize, 60, "Result was expected to have field a_usize 60, had {} instead", automatic_test_struct_ref.a_usize);
            assert_eq!(automatic_test_struct_ref.a_string.as_str(), "Yallow", "Result was expected to have field a_string \"Yallow\", had {} instead", automatic_test_struct_ref.a_string);
            assert_eq!(automatic_test_struct_ref.a_i32, -24, "Result was expected to have field a_i32 -24, had {} instead", automatic_test_struct_ref.a_i32);
            assert_eq!(automatic_test_struct_ref.a_sub.a_usize, 19, "Result was expected to have field a_sub.a_usize 19, had {} instead", automatic_test_struct_ref.a_sub.a_usize);
        },
        None => {
            assert!(false, "Result was expected to have Some(...) at index 0, was None instead");
        }
    }
}
