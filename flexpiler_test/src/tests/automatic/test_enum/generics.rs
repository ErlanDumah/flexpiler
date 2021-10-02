

#[derive(flexpiler::Deserialize)]
pub enum Enum<ContentType> {
    NoContent,
    Argument(ContentType),
    Complex{ content: ContentType },
}


#[test]
fn i32_no_content_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Enum::NoContent"
    );

    let result = Enum::<i32>::deserialize(&mut reader);
    let test_enum = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        Enum::NoContent => {
        },
        _ => {
            assert!(false, "test_enum was expected to be an Enum::Argument(..)");
        }
    }
}


#[test]
fn i32_no_content_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nEnum::NoContent \t\n"
    );

    let result = Enum::<i32>::deserialize(&mut reader);
    let test_enum = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        Enum::NoContent => {
        },
        _ => {
            assert!(false, "test_enum was expected to be an Enum::Argument(..)");
        }
    }
}


#[test]
fn i32_argument_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Enum::Argument(60)"
    );

    let result = Enum::<i32>::deserialize(&mut reader);
    let test_enum = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        Enum::Argument(content) => {
            assert_eq!(content, 60,
                       "content was expected to be 60, was {} instead.",
                       content);
        },
        _ => {
            assert!(false, "test_enum was expected to be an Enum::Argument(..)");
        }
    }
}


#[test]
fn i32_argument_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nEnum::Argument \t\n( \t\n60 \t\n) \t\n"
    );

    let result = Enum::<i32>::deserialize(&mut reader);
    let test_enum = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        Enum::Argument(content) => {
            assert_eq!(content, 60,
                       "content was expected to be 60, was {} instead.",
                       content);
        },
        _ => {
            assert!(false, "test_enum was expected to be an Enum::Argument(..)");
        }
    }
}


#[test]
fn i32_complex_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Enum::Complex{content:60}"
    );

    let result = Enum::<i32>::deserialize(&mut reader);
    let test_enum = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        Enum::Complex{content} => {
            assert_eq!(content, 60,
                       "content was expected to be 60, was {} instead.",
                       content);
        },
        _ => {
            assert!(false, "test_enum was expected to be an Enum::Argument(..)");
        }
    }
}


#[test]
fn i32_complex_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nEnum::Complex \t\n{ \t\ncontent: \t\n60 \t\n} \t\n"
    );

    let result = Enum::<i32>::deserialize(&mut reader);
    let test_enum = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        Enum::Complex{content} => {
            assert_eq!(content, 60,
                       "content was expected to be 60, was {} instead.",
                       content);
        },
        _ => {
            assert!(false, "test_enum was expected to be an Enum::Complex{..}");
        }
    }
}
