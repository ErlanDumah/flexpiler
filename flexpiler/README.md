# Flexpiler

This crate aims to provide deserialisation (and eventually serialisation) in such a way that the user can customise the format in which rust objects are written and read on-the-fly. The idea is to reduce the load on a private developer who wants to jump-start their own project and not bother too much about how their data is stored.

Currently this crate is in its early stages and only supports deserialisation of a reduced form of rust structs and enums with a format that mirrors how structs and enums are initialised in rust itself.


## How to use

The project uses its sister crate ```flexpiler_derive``` to provide a ```#[derive]``` macro. This macro generates code such that the struct / enum being derived gains an impl of ```flexpiler::Deserialize```. From thereon you can call ```YourType::deserialize(reader_mut_ref)``` to deserialize the type from a string or file reader.

```rust
extern crate flexpiler;

#[derive(flexpiler::Deserialize)]
struct MyPoint {
    x: i32,
    y: i32,
}

#[test]
fn basic() {
	// Trait that implements deserialize<...>(...)
    use flexpiler::Deserialize;
    // namespace of common flexpiler reader implementations
    use flexpiler::common::reader;

    // Provide data via reading a string
    // You may at this point provide a file handle or similar instead
    let mut reader = reader::String::from(
        "MyPoint{x:5,y:7}"
    );

    // Try and deserialize our object, modifying the reader in the process
    let parse_result = MyPoint::deserialize(&mut reader);
	// flexpiler deserialization ends in a std::result::Result
    let my_point = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "parade() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    // Check if our deserialized object contains the correct values for its fields
    assert_eq!(my_point.x, 5);
    assert_eq!(my_point.y, 7);
}

```


## Current limitations

Updated last: version ``0.2.0``

The project does not support as many standard types as I'd like. In particular ```HashMap<KeyType, DataType>``` and various primitive types are on the menu to be implemented by me for now.

Flexpiler currently uses a custom ```flexpiler::reader::Trait``` implementation for its parsing, and expects the user to convert their filereader or stringreader or other types of readers into a type that implements it. The project should instead contain various common implementations and possibly support implicit conversion from common practice types such as ```BufReader```.

rust unions are not supported as of now.

The project cannot handle generics as of yet.


## Plans for the future

Being an alpha crate, the API of this project is subject to change at each "minor" increment (x.y.z => x.y+1.z). However for now the desired API to be used from outside follows the upcoming non well defined vision:

```rust
use flexpiler::{Deserialize}

struct CakeFormat {
	// [ some error handling functionality of an unspecified format ]
}

#[derive(flexpiler::Deserialize)]
#[flexpiler_format = CakeFormat]
#[flexpiler_format_style = "{[glazing]}\n{[cream]}\n{[flour_in_grams]}"]
struct MyCustomizedType {
	glazing: std::string,
	cream: MyCreamType,
	flour_in_grams:i32,
}


[#test]
fn test_flexpiler() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "{\"Strawberry\"}\n{MyCreamType::Vanilla}\n{525}"
    );

    let parse_result = MyCustomizedType::deserialize(&mut reader);
    let my_customized_type = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test_flexpiler() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(my_customized_type.glazing.as_str(), "Strawberry");
    assert_eq!(my_customized_type.cream, MyCreamType::Vanilla);
    assert_eq!(my_customized_type.flour_in_grams, 525);
}


```

Notice that the project cannot do this yet. But this is the kind of thing we are aiming for in the future.



## How it works

In essence, the derive macro featured creates an intermediate struct holding your data until final construction of the object. It then writes the entire ```deserialize``` function including parsing and error forwarding from scratch assuming every sub-type has its own ```Deserialize``` implementation.

```Deserialize``` is already implemented in ```flexpiler::common::deserializer``` for various common types such as i32, std::string. However currently for some standard classes you would need to provide a ```flexpiler::Deserialization``` implementation.

