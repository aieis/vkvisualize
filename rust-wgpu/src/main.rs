pub mod k4a;

use k4a_orbbec_sys::*;

const OUT_WIDTH : usize = 60;
const OUT_HEIGHT : usize = 40;
const D : [char; 11] = [' ', '.', ',', '*', '+', ':', ';', '#', '@', '%', ' '];

fn main() {
    let dev_count = k4a::device_get_installed_count();
    
    println!("Number of Devices: {dev_count}");
    if dev_count <= 0 {
        println!("No devices connected. Exiting");
        return;
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

    //let mut calib : k4a_calibration_t;
    let dev = k4a::Device::open(0);
    dev.start_cameras(device_config);

    for i in 0..500 {
        let cap = dev.get_capture(33);
        
        println!("Frame {i}");

        let depth = cap.get_depth_image();
        print!("\x1B[2J\x1B[1;1H");
        print_depth(depth);
    }

    dev.stop_cameras();
    dev.close();
}


fn print_depth(depth: k4a::Image) {
    if depth.width == 0 {
        return;
    }

    let depth_buf = unsafe { depth.get_buffer() };

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
