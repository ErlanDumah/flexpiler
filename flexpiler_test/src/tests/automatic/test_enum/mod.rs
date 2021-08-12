mod subclass;


#[derive(flexpiler::Deserialize)]
pub enum TestEnum {
    NoData,
    DataEmpty(),
    DataSimple(std::string::String),
    DataComplex{
        a_usize: usize,
        a_string: std::string::String,
    },
}


#[test]
fn no_data_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::NoData"
    );

    let test_enum = match TestEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::NoData => {
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::NoData.");
        },
    }
}


#[test]
fn no_data_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::NoData \t\n"
    );

    let test_enum = match TestEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::NoData => {
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::NoData.");
        },
    }
}


#[test]
fn data_empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::DataEmpty()"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test_enum_basic_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataEmpty() => {
        },
        _ => {
            assert!(false, "result was not a TestEnum::DataEmpty.");
        },
    }
}


#[test]
fn data_empty_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::DataEmpty( \t\n) \t\n"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test_enum_basic_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataEmpty() => {
        },
        _ => {
            assert!(false, "result was not a TestEnum::DataEmpty.");
        },
    }
}


#[test]
fn data_simple_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::DataSimple(\"baka\")"
    );

    let test_enum = match TestEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataSimple(argument_0) => {
            assert_eq!(argument_0.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataSimple.");
        },
    }
}


#[test]
fn data_simple_max_formatting_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::DataSimple( \t\n\"baka\" \t\n) \t\n"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "data_simple_max_formatting_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataSimple(argument_0) => {
            assert_eq!(argument_0.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataSimple.");
        },
    }
}


#[test]
fn data_complex_min_formatting_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::DataComplex{a_usize:5,a_string:\"baka\",}"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "data_simple_max_formatting_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataComplex{a_usize, a_string} => {
            assert_eq!(a_usize, 5);
            assert_eq!(a_string.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataComplex.");
        },
    }
}





#[test]
fn data_complex_max_formatting_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::DataComplex{ \t\na_usize: \t\n5 \t\n, \t\na_string: \t\n\"baka\" \t\n, \t\n} \t\n"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "data_simple_max_formatting_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataComplex{a_usize, a_string} => {
            assert_eq!(a_usize, 5);
            assert_eq!(a_string.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataComplex.");
        },
    }
}

