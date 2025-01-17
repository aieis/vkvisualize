#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_devices() {
        unsafe {
            let dev_count = k4a_device_get_installed_count();
            println!("Number of Devices: {dev_count}");
        }
    }
}
    
