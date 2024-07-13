use candid::Principal;
use sha2::{Digest, Sha256};

pub fn convert_to_u32(principal: Principal) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(principal);
    let result = hasher.finalize();
    println!("{:?}", result);

    // Truncate the hash to the first 4 bytes (assuming little-endian byte order)
    u32::from_le_bytes(result[..4].try_into().unwrap())
}

#[cfg(test)]
mod tests {
    // Access functions from parent module

    use candid::Principal;

    use crate::principal_conversion::convert_to_u32;

    #[test]
    fn test_convert_to_u32_valid_data() {
        let data: Principal = Principal::from_text("upy75-k72qv-qc4hu-s3kwb-yd46z-f3vm4-4kgke-pkoce-ayjzn-gy35m-7ae").unwrap();
        let result: u32 = convert_to_u32(data);

        assert!(result == 2002624089);
    }
}
