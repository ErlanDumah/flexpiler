mod test_enum;
mod test_struct;


#[test]
fn simple_test_struct_sub_basic_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;
    use test_struct::sub_struct::TestStructSub;

    let mut reader = reader::String::from(
        "TestStructSub{ a_usize: 60 }"
    );

    let parse_result = TestStructSub::deserialize(&mut reader);

    let test_struct = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.a_usize, 60,
               "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful(): a_usize deserialised value had unexpected usize value {}, expected {}",
               test_struct.a_usize,
               60);
}


#[test]
fn simple_test_struct_basic_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;
    use test_struct::TestStruct;

    let mut reader = reader::String::from(
        "TestStruct{ a_sub: TestStructSub{ a_usize: 60 } a_string: \"Hello\" a_i32: -34 a_usize: 50 }"
    );

    let parse_result = TestStruct::deserialize(&mut reader);

    let test_struct = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_test_struct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.a_string, "Hello",
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_string,
               "Hello");

    assert_eq!(test_struct.a_usize, 50,
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_usize,
               50);

    assert_eq!(test_struct.a_i32, -34,
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               -34);

    assert_eq!(test_struct.a_sub.a_usize, 60,
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_sub.a_usize,
               60);
}
