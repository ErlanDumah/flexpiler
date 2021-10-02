

#[derive(flexpiler::Deserialize)]
pub struct Struct<ContentType> {
    pub content: ContentType,
}



#[test]
fn i32_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "Struct{content:60}"
    );

    let result = Struct::<i32>::deserialize(&mut reader);

    let test_struct = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.content, 60,
               "test_struct.content was expected to be 60, was {} instead.",
               test_struct.content);
}


#[test]
fn i32_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nStruct \t\n{ \t\ncontent: \t\n60 \t\n} \t\n"
    );

    let result = Struct::<i32>::deserialize(&mut reader);

    let test_struct = match result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.content, 60,
               "test_struct.content was expected to be 60, was {} instead.",
               test_struct.content);
}
