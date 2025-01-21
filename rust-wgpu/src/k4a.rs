use k4a_orbbec_sys::*;

pub enum ImageFormat {

}

pub struct Device {
    handle: k4a_device_t
}

impl Device {
    // Should return a result
    pub fn open(index: u32) -> Device {
        let mut handle: k4a_device_t = unsafe { std::mem::zeroed() };
        unsafe { k4a_device_open(index, &mut handle); }
        Device {
            handle
        }        
    }

    pub fn start_cameras(&self, config: k4a_device_configuration_t) -> k4a_result_t {
        unsafe {
            k4a_device_start_cameras(self.handle, &config)
        }
    }

    pub fn get_capture(&self, timeout_in_ms: i32) -> Result<Capture, k4a_wait_result_t> {
        unsafe {
            let mut cap: k4a_capture_t = std::mem::zeroed();
            let res = k4a_device_get_capture(self.handle, &mut cap, timeout_in_ms);
            match res {
                k4a_wait_result_t::K4A_WAIT_RESULT_SUCCEEDED => Ok(Capture::from_handle(cap)),
                _ => Err(res)
            }
        }
    }

    pub fn stop_cameras(&self) { unsafe {k4a_device_stop_cameras(self.handle) } }
    pub fn close(&self) { unsafe { k4a_device_close(self.handle) } }
}

pub struct Capture {
    handle: k4a_capture_t
}

impl Capture {
    pub fn from_handle(handle: k4a_capture_t) -> Capture { Capture { handle} }
    pub fn get_depth_image(&self)  -> Image { Image::from_handle(unsafe { k4a_capture_get_depth_image(self.handle) } ) }
    pub fn release(&self) { unsafe { k4a_capture_release(self.handle) } }
}

pub struct Image {
    handle: k4a_image_t,

    pub width: i32,
    pub height: i32,
}

impl Image {
    fn from_handle(handle: k4a_image_t) -> Image {
        unsafe {
            let width = k4a_image_get_width_pixels(handle);
            let height = k4a_image_get_height_pixels(handle);

            Image {
                handle,
                width,
                height
            }
        }
        
    }

    pub fn get_buffer(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(k4a_image_get_buffer(self.handle), k4a_image_get_size(self.handle) as usize) }
    }

    pub fn release(&self) { unsafe { k4a_image_release(self.handle) } }
}

pub fn device_get_installed_count() -> u32 { unsafe { k4a_device_get_installed_count() } }
