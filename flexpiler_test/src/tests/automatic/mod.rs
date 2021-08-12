pub mod primitives;
pub mod test_enum;
pub mod test_struct;

use flexpiler::common::rustc::Format;
use flexpiler::format::Trait;

#[derive(flexpiler::Deserialize)]
pub struct AutomaticSubTestStruct {
    pub a_usize: usize,
}


#[derive(flexpiler::Deserialize)]
pub struct AutomaticTestStruct {
    pub a_i32: i32,
    pub a_usize: usize,
    pub a_string: std::string::String,
    pub a_sub: AutomaticSubTestStruct,
}


#[derive(flexpiler::Deserialize)]
pub enum AutomaticTestEnum {
    NoData,
    DataEmpty(),
    DataSimple(std::string::String),
    DataComplex{
        a_usize: usize,
        a_string: std::string::String,
    }
}


#[test]
fn simple_automatic_teststruct_automaticsubteststruct_basic_serialisation_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "AutomaticSubTestStruct{ a_usize: 60 }"
    );

    let parse_result = AutomaticSubTestStruct::deserialize(&mut reader);

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
fn simple_automatic_test_struct_basic_serialisation_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "AutomaticTestStruct{ a_string: \"Hello\", a_i32: -34, a_usize: 50, a_sub: AutomaticSubTestStruct{ a_usize: 67 } }"
    );

    let parse_result = AutomaticTestStruct::deserialize(&mut reader);

    let test_struct = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_automatic_test_struct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.a_string, "Hello",
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_string,
               "Hello");

    assert_eq!(test_struct.a_usize, 50,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_usize,
               50);

    assert_eq!(test_struct.a_i32, -34,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               -34);

    assert_eq!(test_struct.a_sub.a_usize, 67,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               67);
}
