package main;

import "core:math"

UE_DOUBLE_PI : f64 : 3.141592653589793238462643383279502884197169399

Rotator :: struct {
    pitch: f64,
    yaw: f64,
    roll: f64
};

Quat :: struct {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

Rotator_Quat :: proc(rotator: Rotator) -> Quat {
    DEG_TO_RAD :: UE_DOUBLE_PI / (180.0)
    RADS_DIVIDED_BY_2 :: DEG_TO_RAD / 2.0

    PitchNoWinding := math.mod(rotator.pitch, 360.0)
    YawNoWinding := math.mod(rotator.yaw, 360.0)
    RollNoWinding := math.mod(rotator.roll, 360.0)

    SP, CP := math.sincos(PitchNoWinding * RADS_DIVIDED_BY_2)
    SY, CY := math.sincos(YawNoWinding * RADS_DIVIDED_BY_2)
    SR, CR := math.sincos(RollNoWinding * RADS_DIVIDED_BY_2)

    return Quat {
        x = CR * SP * SY - SR * CP * CY,
        y = -CR * SP * CY - SR * CP * SY,
        z = CR * CP * SY - SR * SP * CY,
        w = CR * CP * CY + SR * SP * SY,
    }
}

Quat_Normalized :: proc(quat: Quat) -> Quat {
    squaresum := quat.x * quat.x + quat.y * quat.y + quat.z * quat.z + quat.w * quat.w;

    if (squaresum >= 1.0e-8) {
	scale := 1.0 / math.sqrt_f64(squaresum);
        return Quat {
            quat.x * scale,
	    quat.y * scale,
	    quat.z * scale,
	    quat.w * scale,
        }
    } else {
        return Quat { 0.0, 0.0, 0.0, 1.0 }
    }
}

Quat_Matrix :: proc(quat: Quat) -> matrix[4, 4] f64 {
    R := matrix [4, 4] f64 {};

    x2 := quat.x + quat.x;
    y2 := quat.y + quat.y;
    z2 := quat.z + quat.z;
    xx := quat.x * x2;
    xy := quat.x * y2;
    xz := quat.x * z2;
    yy := quat.y * y2;
    yz := quat.y * z2;
    zz := quat.z * z2;
    wx := quat.w * x2;
    wy := quat.w * y2;
    wz := quat.w * z2;

    R[0][0] = 1.0 - (yy + zz);          R[1][0] = xy - wz;                      R[2][0] = xz + wy;                      R[3][0] = 0.0;
    R[0][1] = xy + wz;                  R[1][1] = 1.0 - (xx + zz);              R[2][1] = yz - wx;                      R[3][1] = 0.0;
    R[0][2] = xz - wy;                  R[1][2] = yz + wx;                      R[2][2] = 1.0 - (xx + yy);              R[3][2] = 0.0;
    R[0][3] = 0.0;                      R[1][3] = 0.0;                          R[2][3] = 0.0;                          R[3][3] = 1.0;

    return R
}

