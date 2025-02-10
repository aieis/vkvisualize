package utils

import "core:math"

Circle_Create :: proc(x: f32, y: f32, radius: f32, eighth_segments: u32) -> [dynamic]f32 {
    da := math.PI / 4 / cast(f32) eighth_segments

    circle := make([dynamic]f32, eighth_segments * 8 * 2)
    

    n := eighth_segments * 2
    
    for p in 0..<eighth_segments {
        s, c := math.sincos(da * cast(f32)p)
        px := radius * c
        py := radius * s
        
        i := p * 2
        circle[i]             = +px+x;                     circle[i+1]           = +py+y
        circle[(n-2-i)+n]     = +py+x;                     circle[(n-2-i)+n+1]   = +px+y
        circle[i+n*2]         = -py+x;                     circle[i+n*2+1]       = +px+y
        circle[(n-2-i)+n*3]   = -px+x;                     circle[(n-2-i)+n*3+1] = +py+y
        circle[i+n*4]         = -px+x;                     circle[i+n*4+1]       = -py+y
        circle[(n-2-i)+n*5]   = -py+x;                     circle[(n-2-i)+n*5+1] = -px+y
        circle[i+n*6]         = +py+x;                     circle[i+n*6+1]       = -px+y
        circle[(n-2-i)+n*7]   = +px+x;                     circle[(n-2-i)+n*7+1] = -py+y
    }

    return circle
}
