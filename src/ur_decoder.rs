use crate::fountain_decoder::FountainDecoder;
use crate::ur::UR;
use crate::utils;

#[derive(Default)]
pub struct URDecoder {
    expected_type: String,
    result: Option<UR>,
    error: Option<UR>,
    fountain_decoder: FountainDecoder,

    pub ur_type: String,
}

impl URDecoder {
    pub fn new(fountain_decoder: Option<FountainDecoder>, ur_type: Option<String>) -> Result<URDecoder, String> {
        let _fountain_decoder = fountain_decoder.unwrap_or(FountainDecoder::new());
        let _ur_type = ur_type.unwrap_or("bytes".to_string());

        if utils::is_ur_type(&_ur_type) {
            Ok(URDecoder {
                expected_type: "".to_string(),
                fountain_decoder: _fountain_decoder,
                ur_type: _ur_type,
                ..Default::default()
            })
        }
        else {
            Err(format!("Invalid ur type: {}", _ur_type))
        }
    }

    // fn decode_body(ur_type: String, message: String) -> UR {
    //
    // }
}