use k4a_orbbec_sys::*;

pub enum ImageFormat {

}

pub struct Device {
    pub handle: k4a_device_t
}

impl Device {
    // Should return a result
    pub fn open(index: u32) -> Self {
        let mut handle: k4a_device_t = unsafe { std::mem::zeroed() };
        unsafe { k4a_device_open(index, &mut handle); }
        Self {
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

    pub fn get_calibration(&self, depth_mode: k4a_depth_mode_t, color_resolution: k4a_color_resolution_t) -> Calibration {
        unsafe {
            let mut cal = std::mem::zeroed();
            k4a_device_get_calibration(self.handle, depth_mode,  color_resolution, &mut cal);
            Calibration::from_handle(cal)
        }
    }

    pub fn stop_cameras(&self) { unsafe {k4a_device_stop_cameras(self.handle) } }
    pub fn close(&self) { unsafe { k4a_device_close(self.handle) } }
}

pub struct Capture {
    pub handle: k4a_capture_t
}

impl Capture {
    pub fn from_handle(handle: k4a_capture_t) -> Self { Self { handle} }
    pub fn get_depth_image(&self)  -> Image { Image::from_handle(unsafe { k4a_capture_get_depth_image(self.handle) } ) }
    pub fn release(&self) { unsafe { k4a_capture_release(self.handle) } }
}

pub struct Image {
    pub handle: k4a_image_t,

    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub size: u64,
}

impl Image {    
    fn from_handle(handle: k4a_image_t) -> Self {
        unsafe {
            let width = k4a_image_get_width_pixels(handle);
            let height = k4a_image_get_height_pixels(handle);
            let stride = k4a_image_get_stride_bytes(handle);
            let size = k4a_image_get_size(handle);

            Self {
                handle,
                width,
                height,
                stride,
                size
            }
        }
        
    }

    pub fn create(format: k4a_image_format_t, width: i32, height: i32, stride: i32) -> Self {
        unsafe {
            let mut handle : k4a_image_t = std::mem::zeroed();
            k4a_image_create(format, width, height, stride, &mut handle);
            Self {
                handle,
                width,
                height,
                stride,
                size: (stride * height) as u64
            }
        }
        
    }

    pub fn get_buffer(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(k4a_image_get_buffer(self.handle), k4a_image_get_size(self.handle) as usize) }
    }

    pub fn release(&self) { unsafe { k4a_image_release(self.handle) } }
}


pub struct Calibration {
    pub handle: k4a_calibration_t
}

impl Calibration {
    pub fn from_handle(handle: k4a_calibration_t) -> Self { Self { handle} }
}

pub struct Transformation {
    pub handle: k4a_transformation_t
}

impl Transformation {
    pub fn create(cal: &Calibration) -> Self {
        let handle = unsafe { k4a_transformation_create(&cal.handle) };
        Self {
            handle
        }
    }

    pub fn depth_image_to_point_cloud(&self, depth: &Image) -> Image {
        let pcl = Image::create(k4a_image_format_t::K4A_IMAGE_FORMAT_CUSTOM, depth.width, depth.height, depth.width * 3 * 2);

        unsafe {
            k4a_transformation_depth_image_to_point_cloud(self.handle, depth.handle, k4a_calibration_type_t::K4A_CALIBRATION_TYPE_DEPTH, pcl.handle);
        }

        pcl
    }

    pub fn destroy(&self) { unsafe { k4a_transformation_destroy(self.handle) } }
}

pub fn device_get_installed_count() -> u32 { unsafe { k4a_device_get_installed_count() } }
