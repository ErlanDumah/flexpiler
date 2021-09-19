

#[test]
fn none_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "None"
    );

    let test_i32 = match std::option::Option::<i32>::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_i32 {
        Some(value) => {
            assert!(false, "result was expected to be None but was Some({})", value);
        },
        None => {
        }
    }
}


#[test]
fn none_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nNone \t\n"
    );

    let test_i32 = match std::option::Option::<i32>::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_i32 {
        Some(value) => {
            assert!(false, "result was expected to be None but was Some({})", value);
        },
        None => {
        }
    }
}


#[test]
fn some_positive_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Some(5)"
    );

    let test_i32 = match std::option::Option::<i32>::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_i32 {
        Some(value) => {
            assert_eq!(value, 5, "result was expected to be Some(5) but was Some({})", value);
        },
        None => {
            assert!(false, "result was expected to be Some(5) but was None");
        }
    }
}


#[test]
fn some_positive_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nSome \t\n( \t\n5 \t\n) \t\n"
    );

    let test_i32 = match std::option::Option::<i32>::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_i32 {
        Some(value) => {
            assert_eq!(value, 5, "result was expected to be Some(5) but was Some({})", value);
        },
        None => {
            assert!(false, "result was expected to be Some(5) but was None");
        }
    }
}

