extern crate flexpiler;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod examples {

    #[derive(flexpiler::Deserialize)]
    struct MyPoint {
        x: i32,
        y: i32,
    }

    #[test]
    fn parade() {
        use flexpiler::Deserialize;
        use flexpiler::common::reader;

        let mut reader = reader::String::from(
            "MyPoint{x:5,y:7}"
        );

        let parse_result = MyPoint::deserialize(&mut reader);

        let my_point = match parse_result {
            Ok(value) => value,
            Err(error) => {
                assert!(false, "parade() test ended in a failed deserialization:\n{}", error);
                return;
            }
        };

        assert_eq!(my_point.x, 5);
        assert_eq!(my_point.y, 7);
    }
}