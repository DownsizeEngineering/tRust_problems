pub fn run () -> f64 {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let mangle = minutes as f64 * 6.0;
        let hangle = hour as f64 * 30.0 + minutes as f64 / 2.0;

        let diff = (hangle - mangle).abs();
        if (360.0 - diff) < diff { return 360.0 - diff; }
        return diff;
    }
    angle_clock(12, 30)
}