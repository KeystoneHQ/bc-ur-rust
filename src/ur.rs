use crate::utils;

#[derive(Default)]
pub struct UR {
    _cbor_payload: Vec<u8>,
    _type: String,
}

impl UR {
    pub fn new(cbor_payload: Vec<u8>, ur_type: Option<String>) -> Result<UR, String> {
        let _ur_type = ur_type.unwrap_or("bytes".to_string());
        if utils::is_ur_type(&_ur_type) {
            Ok(UR{_cbor_payload: cbor_payload, _type: _ur_type})
        }
        else {
            Err(format!("Invalid ur type: {}", _ur_type))
        }
    }
}
