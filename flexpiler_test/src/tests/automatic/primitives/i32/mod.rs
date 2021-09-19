mod option;
mod vec;


#[test]
fn positive_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "5"
    );

    let test_i32 = match i32::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_i32, 5, "result was expected to be 5 but was {}", test_i32);
}


#[test]
fn positive_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\n5 \t\n"
    );

    let test_i32 = match i32::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_i32, 5, "result was expected to be 5 but was {}", test_i32);
}