

#[test]
fn some_positive_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nSome \t\n( \t\n[ \t\n5 \t\n] \t\n) \t\n"
    );

    let test_option_vec_i32: std::option::Option<std::vec::Vec<i32>> = match std::option::Option::<i32>::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    let test_vec_i32 = match test_option_vec_i32 {
        Some(test_vec_i32) => test_vec_i32,
        None => {
            assert!(false, "Result was expected to be Some([5]), was None instead");
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

