//! # Libuta Wrapper
//! This module provides a wrapper for the libuta library.
//! This module provides the functionality to derive a key from a string using the libuta library.
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;
use bindings::*;




/// Derive a key from a string using the libuta library.
/// # Arguments
/// * `derivation_string` - The string to derive the key from.
/// # Returns
/// * `Result<Vec<u8>>` -
/// Returns a `Vec<u8>` containing a bytstream derived from the derivation_string if successful otherwise an error is returned.
/// # Errors
/// * `Err` - An error occurred while deriving the key.
/// # Note
/// This function uses unsafe code to interact with the libuta library that is written in C.
/// Only up to the first eight characters of the derivation_string are used. The rest is ignored.
///
pub fn libuta_derive_key(derivation_string: &str) -> Result<Vec<u8>, String>{
    if derivation_string.is_empty() {
        return Err("Error: Derivation string must be at least 8 characters long".into());
    }
    unsafe {
        let mut uta: uta_api_v1_t = uta_api_v1_t {
            close: None,
            context_v1_size: None,
            derive_key: None,
            get_device_uuid: None,
            get_random: None,
            len_key_max: None,
            open: None,
        };

        //UTA Init
        let mut rc: uta_rc = uta_init_v1(&mut uta as *mut _);
        if rc != 0 {
            return Err("Error: UTA Init".into());
        }

        //UTA Open
        let mut context: uta_context_v1_t = _uta_context_v1_t { _unused: [] };
        rc = (uta.open.unwrap())(&mut context as *mut _);
        if rc != 0 {
            return Err("Error: UTA Open".into());
        }

        //UTA Get Device UUID
        let mut key_ptr = vec![0u8; 32];
        rc = (uta.get_device_uuid.unwrap())(&mut context as *mut _, key_ptr.as_mut_ptr());
        if rc != 0 {
            return Err("Error: UTA Get Device UUID".into());
        }

        //UTA Get Max Key Length
        let len_key_max = (uta.len_key_max.unwrap())();

        //UTA Derive Key
        rc = (uta.derive_key.unwrap())(
            &mut context as *mut _,
            key_ptr.as_mut_ptr(),
            len_key_max,
            derivation_string.as_ptr(),
            8,
            0,
        );
        if rc != 0 {
            return Err("Error: UTA Derive Key".into());
        }

        //UTA Close
        rc = (uta.close.unwrap())(&mut context as *mut _);
        if rc != 0 {
            return Err("Error: UTA Close".into());
        }

        if key_ptr.is_empty() {
            return Err("Error: Key is empty".into());
        }
        Ok(key_ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the libuta_derive_key function.
    #[test]
    fn test_libuta_derive_key() {
        let derivation_string = "test";
        let result = libuta_derive_key(derivation_string);
        assert!(result.is_ok());
    }
    ///Test the libuta_derive_key function with empty derivation_string.
    /// The function should return an error.
    #[test]
    fn test_libuta_derive_key_empty() {
        let derivation_string = "";
        let result = libuta_derive_key(derivation_string);
        assert!(result.is_err());
    }


    ///Test the libuta_derive_key function with multiple derivation_string with different length.
    /// The function should return an error if the key is the same for two different derivation_string with length <= 8.
    #[test]
    fn test_libuta_derive_key_length() {
        let mut derivation_string: String = "".to_owned();
        let mut result_old: Vec<u8> = vec![];
        for _ in 0..1000 {
            derivation_string = derivation_string.to_owned() + "a";
            let result = libuta_derive_key(&derivation_string);
            if result.is_err() {
                assert!(false);
            }
            else {
                let result = result.unwrap();
                if result == result_old && derivation_string.len() > 8{
                    assert!(true);
                    return;
                } else {
                    result_old = result;
                }

            }

        }
        assert!(false)
    }

    ///Test the libuta_derive_key function with multiple derivation_string with different content.
    ///The function should return a different key for different derivation_string.
    #[test]
    fn test_libuta_derive_key_content() {
        let derivation_string = "test";
        let derivation_string2 = "tset";
        let derivation_string3 = "Test";
        let result = libuta_derive_key(derivation_string);
        let result2 = libuta_derive_key(derivation_string2);
        let result3 = libuta_derive_key(derivation_string3);
        assert!(result.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        let result = result.unwrap();
        let result2 = result2.unwrap();
        let result3 = result3.unwrap();
        assert!(result != result2);
        assert!(result != result3);
        assert!(result2 != result3);

    }

    ///Test the libuta_derive_key function with the same derivation_string for consistency.
    /// The function should return the same key for the same derivation_string.
    #[test]
    fn test_libuta_derive_key_consistency() {
        let derivation_string = "test";
        let result = libuta_derive_key(derivation_string);
        let result2 = libuta_derive_key(derivation_string);
        assert!(result.is_ok());
        assert!(result2.is_ok());
        assert!(result.unwrap() == result2.unwrap());
    }
}
