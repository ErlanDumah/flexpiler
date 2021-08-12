

struct PrimitivesCollection {

    a_bool: bool,

    a_i8: i8,
    a_i16: i16,
    a_i32: i32,
    a_i64: i64,
    a_i128: i128,
    a_isize: isize,

    a_u8: u8,
    a_u16: u16,
    a_u32: u32,
    a_u64: u64,
    a_u128: u128,
    a_usize: usize,

    a_char: char,

    a_f32: f32,
    a_f64: f64,

    a_unit_type: (),

    a_string: std::string::String,

    a_option: std::option::Option<i32>,
    a_result: std::result::Result<i32, i32>,

    a_array: [i32; 5],

    a_vec: std::vec::Vec<i32>,
    a_vec_deque: std::collections::VecDeque<i32>,
    a_linked_list: std::collections::LinkedList<i32>,
    a_hashmap: std::collections::HashMap<i32, i32>,
    a_hashset: std::collections::HashSet<i32>,
    a_btree_map: std::collections::BTreeMap<i32, i32>,
    a_btree_set: std::collections::BTreeSet<i32>,
    a_binary_heap: std::collections::BinaryHeap<i32>,

}


#[test]
fn i32_positive_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "5"
    );

    let test_i32 = match i32::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_i32, 5, "result was expected to be 5 but was {}", test_i32);
}


#[test]
fn i32_positive_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\n5 \t\n"
    );

    let test_i32 = match i32::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_i32, 5, "result was expected to be 5 but was {}", test_i32);
}


#[test]
fn string_positive_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "\"baka\""
    );

    let test_string = match std::string::String::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_string.as_str(), "baka", "result was expected to be \"baka\" but was {}", test_string);
}


#[test]
fn string_positive_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\n\"baka\" \t\n"
    );

    let test_string = match std::string::String::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_string.as_str(), "baka", "result was expected to be \"baka\" but was {}", test_string);
}
