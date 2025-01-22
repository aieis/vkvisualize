use k4a_orbbec_sys::*;
use crate::k4a;

const OUT_WIDTH : usize = 60;
const OUT_HEIGHT : usize = 40;
const D : [char; 11] = [' ', '.', ',', '*', '+', ':', ';', '#', '@', '%', ' '];

pub struct App {
    pub k4a_device: Vec<k4a::Device>,
    pub device: Vec<Device>,
    pub depth: Vec<Option<k4a::Image>>,
    pub proj: Vec<Option<k4a::Image>>
}

impl App {
    pub fn new() ->  Self {
        let dev_count = k4a::device_get_installed_count();

        println!("Number of Devices: {dev_count}");
        if dev_count <= 0 {
            println!("No devices connected. Exiting");
        }

        let device_config = k4a_device_configuration_t {
            camera_fps: k4a_fps_t::K4A_FRAMES_PER_SECOND_30,
            color_format: k4a_image_format_t::K4A_IMAGE_FORMAT_COLOR_MJPG,
            color_resolution: k4a_color_resolution_t::K4A_COLOR_RESOLUTION_OFF,
            depth_delay_off_color_usec: 0,
            depth_mode: k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_UNBINNED,
            subordinate_delay_off_master_usec: 0,
            synchronized_images_only: false,
            wired_sync_mode: k4a_wired_sync_mode_t::K4A_WIRED_SYNC_MODE_STANDALONE,
            disable_streaming_indicator: false,
        };

        let mut k4a_device = Vec::new();
        let mut device = Vec::new();
        let mut depth = Vec::new();
        let mut proj = Vec::new();

        for i in 0..dev_count {
            let dev = k4a::Device::open(i);
            dev.start_cameras(device_config);
            k4a_device.push(dev);

            let dev = Device {
                rotation: cgmath::Vector3::new(0.0, 0.0, 0.0),
                location: cgmath::Vector3::new(0.0, 0.0, 0.0),
            };

            device.push(dev);
            depth.push(None);
            proj.push(None);
        }

        Self {
            k4a_device,
            device,
            depth,
            proj
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.k4a_device.len() {

            /* TODO: Might be able to offload to caller */
            if self.depth[i].is_some() {
                self.depth[i].as_ref().unwrap().release();
                self.depth[i] = None;
            }
            
            if let Ok(cap) = self.k4a_device[i].get_capture(1) {
                let im = cap.get_depth_image();

                if self.proj[i].is_none() {
                    let depth_mode = k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_UNBINNED;
                    let color_resolution = k4a_color_resolution_t::K4A_COLOR_RESOLUTION_OFF;
                    let cal = self.k4a_device[i].get_calibration(depth_mode, color_resolution);
                    let trans = k4a::Transformation::create(&cal);
                    let pcl = create_proj(&trans, &im);
                    self.proj[i] = Some(pcl);                   
                    trans.destroy();
                }

                self.depth[i] = Some(im);
                cap.release();
            }
        }
    }

    pub fn stop(&self) {
        for dev in self.k4a_device.iter() {
            dev.stop_cameras();
            dev.close();
        }
    }
}

fn create_proj(trans: &k4a::Transformation, im: &k4a::Image) -> k4a::Image {
    let im = k4a::Image::create(k4a_image_format_t::K4A_IMAGE_FORMAT_DEPTH16, im.width, im.height, im.stride);
    unsafe {
        let buf = k4a_image_get_buffer(im.handle);
        for i in 0..im.size as isize / 2 {
            *buf.offset(i+1) = 1;
        }
    }

    trans.depth_image_to_point_cloud(&im)
}

#[allow(dead_code)]
fn print_depth(depth: k4a::Image) {
    if depth.width == 0 {
        return;
    }

    let depth_buf = depth.get_buffer();

    let stride = depth.width as usize * 2;
    for h in 0..OUT_HEIGHT {
        let py = ((depth.height as f64 / OUT_HEIGHT as f64) * h as f64) as usize;
        for c in 0..OUT_WIDTH {
            let px = ((depth.width as f64 / OUT_WIDTH as f64) * c as f64) as usize;
            let r = depth_buf[py * stride + px * 2];
            let v = ((r as f32 / 255.0) * 9.0) as usize;
            print!("{}", D[10 - v]);
        }
        println!();
    }
}

pub enum StreamType {
    Color,
    Depth
}

pub struct Stream {
    depth: k4a::Image,
    proj: Vec<Vec<f32>>
}

pub struct Device {
    pub rotation: cgmath::Vector3<f32>,
    pub location: cgmath::Vector3<f32>,
}
