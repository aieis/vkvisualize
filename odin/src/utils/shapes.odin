package utils

import "core:math"

Circle_Create :: proc(x: f32, y: f32, radius: f32, quarter_segments: u32) -> [dynamic]f32 {
    dx := radius / cast (f32) quarter_segments

    circle := make([dynamic]f32, quarter_segments * 4 * 2)
    radius_2 := radius * radius

    for i in 0..<quarter_segments {
        px := radius - dx * cast(f32) i
        py := math.sqrt(radius_2 - px * px)

        idx := i * 8
        circle[idx+0] = -px + x
        circle[idx+1] = -py + y
        circle[idx+2] = -px + x
        circle[idx+3] = py  + y
        circle[idx+4] = py  + x
        circle[idx+5] = -px + y
        circle[idx+6] = py  + x
        circle[idx+7] = px  + y
    }

    return circle
}
