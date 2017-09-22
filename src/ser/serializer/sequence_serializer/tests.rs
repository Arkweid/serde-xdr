use serde::ser::Serializer as SerdeSerializer;

use super::*;
use super::super::tests::*;

#[test]
fn serialize_sequence() {
    let mut buffer = Vec::new();

    let length = 3;
    let first_element: i32 = -20;
    let second_element = "hello";
    let third_element: i32 = -45;

    {
        let mut sequence_serializer =
            Serializer::new(&mut buffer).serialize_seq(Some(length)).unwrap();

        sequence_serializer.serialize_element(&first_element).unwrap();
        sequence_serializer.serialize_element(second_element).unwrap();
        sequence_serializer.serialize_element(&third_element).unwrap();
        sequence_serializer.end().unwrap();
    }

    let mut expected_bytes = bytes_of(length as u32);

    expected_bytes.append(&mut bytes_of(first_element as u32));
    expected_bytes.append(&mut bytes_of_str(second_element, 2));
    expected_bytes.append(&mut bytes_of(third_element as u32));

    assert_eq!(buffer, expected_bytes);
}

