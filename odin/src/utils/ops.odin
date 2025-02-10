package utils


/*
vm: positions arranges as [x0, y0, z0, x1, y1, z1, ...]
R: rotation 4x4 matrix
T: translation offset as [x, y, z]
dst: The output is stored there, must be of the same shape as vm
*/

mullti_vector_rotate_translate :: proc(vm: ^[dynamic]f32, R: matrix[4,4]f32, T:[3]f32, dst: ^[dynamic]f32){
    for i := 0; i<len(vm); i+=3 {
        x := vm[i] * cast(f32) R[0][0] + vm[i+1] * cast(f32) R[0][1] + vm[i+2] * cast(f32) R[0][2]
        y := vm[i] * cast(f32) R[1][0] + vm[i+1] * cast(f32) R[1][1] + vm[i+2] * cast(f32) R[1][2]
        z := vm[i] * cast(f32) R[2][0] + vm[i+1] * cast(f32) R[2][1] + vm[i+2] * cast(f32) R[2][2]
        dst[i + 0] = x+T[0]
        dst[i + 1] = y+T[1]
        dst[i + 2] = z+T[2]
    }
}
