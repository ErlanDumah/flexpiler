

#[derive(flexpiler::Deserialize)]
pub struct SubClass {
    data: i32,
}


#[derive(flexpiler::Deserialize)]
pub enum SubClassEnum {
    NoData,
    SubClass(SubClass)
}


#[test]
fn no_data_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "SubClassEnum::NoData"
    );

    let sub_class_enum = match SubClassEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match sub_class_enum {
        SubClassEnum::NoData => {
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::NoData.");
        },
    }
}


#[test]
fn sub_class_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "SubClassEnum::SubClass(SubClass{data:5})"
    );

    let sub_class_enum = match SubClassEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match sub_class_enum {
        SubClassEnum::SubClass(sub_class) => {
            assert_eq!(sub_class.data, 5, "Deserialized SubClass object did not feature expected result 5, instead {}", sub_class.data);
        },
        _ => {
            assert!(false, "result was not a SubClassEnum::NoData.");
        },
    }
}


#[test]
fn sub_class_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "SubClassEnum::SubClass( \t\nSubClass \t\n{ \t\ndata: \t\n5 \t\n} \t\n)"
    );

    let sub_class_enum = match SubClassEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match sub_class_enum {
        SubClassEnum::SubClass(sub_class) => {
            assert_eq!(sub_class.data, 5, "Deserialized SubClass object did not feature expected result 5, instead {}", sub_class.data);
        },
        _ => {
            assert!(false, "result was not a SubClassEnum::SubClass.");
        },
    }
}

