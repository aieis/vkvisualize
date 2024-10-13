#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#include "librealsense2/h/rs_config.h"
#include "librealsense2/h/rs_context.h"
#include "librealsense2/h/rs_device.h"
#include "librealsense2/h/rs_frame.h"
#include "librealsense2/h/rs_option.h"
#include "librealsense2/h/rs_sensor.h"
#include "librealsense2/h/rs_types.h"
#include "librealsense2/h/rs_pipeline.h"
#include "librealsense2/rs.h"
#include "librealsense2/rsutil.h"

#include "utils/record.h"

void check_error(rs2_error* e);
void print_device_info(rs2_device* dev);
bool find_depth_sensor(rs2_sensor** sensor, rs2_sensor_list* sensors, rs2_error** err);

int main(int argc, char** argv) {
    printf("Running RealSense Record tool.\n");
    rs2_error* err = 0;
    rs2_context* ctx = rs2_create_context(RS2_API_VERSION, &err);
    
    rs2_device_list* dl = rs2_query_devices(ctx, &err);
    
    int n = rs2_get_device_count(dl, &err);

    printf("Discovered %d devices.\n", n);

    rs2_device* dev = rs2_create_device(dl, 0, &err);

    print_device_info(dev);

    rs2_sensor_list* sensors = rs2_query_sensors(dev, &err);
    rs2_sensor* sensor;
    if (!find_depth_sensor(&sensor, sensors, &err)) {
        printf("Could not find depth sensor\n");
        return 0;
    }
    

    rs2_set_option((rs2_options*) sensor, RS2_OPTION_DEPTH_UNITS, 0.0001, &err);

    rs2_pipeline* pipe = rs2_create_pipeline(ctx, &err);

    rs2_config* conf = rs2_create_config(&err);
    rs2_config_enable_stream(conf, RS2_STREAM_DEPTH, 0, 640, 480, RS2_FORMAT_Z16, 30, &err);

    rs2_pipeline_profile* profile = rs2_pipeline_start_with_config(pipe, conf, &err);
    rs2_stream_profile_list* stream_profiles = rs2_pipeline_profile_get_streams(profile, &err);
    const rs2_stream_profile* stream_profile = rs2_get_stream_profile(stream_profiles, 0, &err);

    rs2_intrinsics intrin = {};
    rs2_get_video_stream_intrinsics(stream_profile, &intrin, &err);
    float* proj = malloc(640*480 * 3 * sizeof(float));
    printf("Populating projection data.\n");
    for (int i = 0; i < 640 * 480 - 1; i++) {
        int x = i % 640;
        int y = i / 640;
        rs2_deproject_pixel_to_point(proj + i * 3, &intrin, (float[2]) {x, y}, 1.0);
    }

    printf("Creating frame data.\n");
    const int frame_count = 30 * 10;
    uint16_t* frame_data = malloc(frame_count * 640 * 480 * sizeof(uint16_t));

    int current_frame_count = 0;
    
    while (current_frame_count < frame_count) {
        printf("Recording frame: %d\n", current_frame_count);
        rs2_frame* frames = rs2_pipeline_wait_for_frames(pipe, RS2_DEFAULT_TIMEOUT, &err);
        check_error(err);
        int nframes = rs2_embedded_frames_count(frames, &err);

        for (int i = 0; i < nframes; i++) {
            rs2_frame* frame = rs2_extract_frame(frames, i, &err);
            const void* data = rs2_get_frame_data(frame, &err);
            memcpy(frame_data + current_frame_count * 640 * 480, data, 640*480*sizeof(uint16_t));
            rs2_release_frame(frame);
            current_frame_count++;
        }
        rs2_release_frame(frames);
    }

    RecordData record_data = {
        640,
        480,
        30,
        (float*) proj,
        (uint16_t*) frame_data
    };

    const char* out_file = argc > 1? argv[1] : "assets/recordings/data.rdbin";
    if (!write_record_data(&record_data, out_file)) {
        printf("Failed to write output to file: '%s'\n", out_file);
    }

    rs2_delete_device(dev);
    rs2_delete_device_list(dl);
    rs2_delete_context(ctx);
}

bool find_depth_sensor(rs2_sensor** sensor_ptr, rs2_sensor_list* sensors, rs2_error** err) {
    int sensor_count = rs2_get_sensors_count(sensors, err);    
    for (int i = 0; i < sensor_count; i++) {
        rs2_sensor* sensor = rs2_create_sensor(sensors, i, err);
        if (rs2_supports_option((rs2_options*) sensor, RS2_OPTION_DEPTH_UNITS, err)) {
            *sensor_ptr = sensor;
            return true;
        }

        rs2_delete_sensor(sensor);
    }

    return false;
    
}



void check_error(rs2_error* e)
{
    if (e)
    {
        printf("rs_error was raised when calling %s(%s):\n", rs2_get_failed_function(e), rs2_get_failed_args(e));
        printf("    %s\n", rs2_get_error_message(e));
        printf("\n");
        exit(EXIT_FAILURE);
    }
}

void print_device_info(rs2_device* dev)
{
    rs2_error* e = 0;
    printf("\nDevice:  %s\n", rs2_get_device_info(dev, RS2_CAMERA_INFO_NAME, &e));
    check_error(e);
    printf("    Serial number: %s\n", rs2_get_device_info(dev, RS2_CAMERA_INFO_SERIAL_NUMBER, &e));
    check_error(e);
    printf("    Firmware version: %s\n\n", rs2_get_device_info(dev, RS2_CAMERA_INFO_FIRMWARE_VERSION, &e));
    check_error(e);
}
