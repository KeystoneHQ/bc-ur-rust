pub fn is_ur_type(ur_type: &String) -> bool {
    ur_type.chars().fold::<bool, fn(bool, char) -> bool>(true, |acc, x| {
        match x {
            'a'..='z' | '0'..='9' | '-' => {
                acc & true
            }
            _ => {
                acc & false
            }
        }
    })
}

pub fn get_crc_hex(message: Vec<u8>) -> String {
    format!("{:08x}", crc32fast::hash(message.as_slice()))
}

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn test_crc() {
        let crc1 = utils::get_crc_hex("Hello, world!".as_bytes().to_vec());
        assert_eq!(crc1, "ebe6c6e6");
        let crc2 = utils::get_crc_hex([1u8, 2u8, 3u8, 4u8].to_vec());
        assert_eq!(crc2, "b63cfbcd");
    }
}