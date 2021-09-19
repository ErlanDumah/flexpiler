mod vec;


#[test]
fn some_string_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Some(\"baka\")"
    );

    let test_option: std::option::Option<std::string::String> = match std::option::Option::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_option {
        Some(string) => {
            assert_eq!(string.as_str(), "baka");
        }
        _ => assert!(false, "Result was not a \"Some\" entry."),
    }
}


#[test]
fn some_string_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nSome \t\n( \t\n\"baka\" \t\n) \t\n"
    );

    let test_option: std::option::Option<std::string::String> = match std::option::Option::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_option {
        Some(string) => {
            assert_eq!(string.as_str(), "baka");
        }
        _ => assert!(false, "Result was not a \"Some\" entry."),
    }
}


#[test]
fn none_string_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "None"
    );

    let test_option: std::option::Option<std::string::String> = match std::option::Option::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_option {
        None => {
        }
        _ => assert!(false, "Result was not a \"None\" entry."),
    }
}


#[test]
fn none_string_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nNone \t\n"
    );

    let test_option: std::option::Option<std::string::String> = match std::option::Option::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_option {
        None => {
        }
        _ => assert!(false, "Result was not a \"None\" entry."),
    }
}


#[test]
fn some_i32_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Some(5)"
    );

    let test_option: std::option::Option<i32> = match std::option::Option::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_option {
        Some(value) => {
            assert_eq!(value, 5);
        }
        _ => assert!(false, "Result was not a \"Some\" entry."),
    }
}


#[test]
fn some_i32_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nSome \t\n( \t\n5 \t\n) \t\n"
    );

    let test_option: std::option::Option<i32> = match std::option::Option::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_option {
        Some(value) => {
            assert_eq!(value, 5);
        }
        _ => assert!(false, "Result was not a \"Some\" entry."),
    }
}
