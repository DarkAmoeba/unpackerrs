use packed_struct::prelude::*;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use serde::{Serialize, Deserialize};
use serde_json;
use bincode;

#[derive(serde::Serialize, serde::Deserialize, PackedStruct)]
#[packed_struct(bit_numbering="msb0")]
pub struct TestPack {
    #[packed_field(bits="0..=2")]
    tiny_int: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="3")]
    enabled: bool
}

#[derive(serde::Serialize, serde::Deserialize, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", size_bytes = "12", endian = "lsb")]
pub struct Vec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}


#[derive(Serialize, Deserialize)]
struct MyData {
    // Define your data structure
    name: String,
    age: u32,
}

#[pyfunction]
fn deserialize_from_binary(binary_data: Vec<u8>) -> PyResult<String> {
    // Deserialize binary data to Rust struct
    let data: MyData = bincode::deserialize(&binary_data)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize binary data: {}", e)))?;

    // Serialize struct to JSON string
    let json_str = serde_json::to_string(&data)
        .map_err(|e| PyValueError::new_err(format!("Failed to serialize to JSON: {}", e)))?;

    Ok(json_str)
}

#[pyfunction]
fn serialize_to_binary(json_str: &str) -> PyResult<Vec<u8>> {
    // Deserialize JSON string to Rust struct
    let data: MyData = serde_json::from_str(json_str)
        .map_err(|e| PyValueError::new_err(format!("Failed to deserialize JSON: {}", e)))?;

    // Serialize struct to binary
    let encoded: Vec<u8> = bincode::serialize(&data)
        .map_err(|e| PyValueError::new_err(format!("Failed to serialize to binary: {}", e)))?;

    Ok(encoded)
}


/// A Python module implemented in Rust.
#[pymodule]
fn unpackerrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(deserialize_from_binary, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_to_binary, m)?)?;
    Ok(())
}
