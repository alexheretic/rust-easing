use super::ease::Easing;
struct Back;

impl Easing for Back {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let s = 1.70158_f32;
        let inner_t = t / d;
        let post_fix = t;
        c * (post_fix) * inner_t * ((s + 1.0) * inner_t - s) + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let s = 1.70158_f32;
        let inner_t = (t / d) - 1.0;
        c * (inner_t * inner_t * ((s + 1.0) * inner_t + s) + 1.0) + b
    }

    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let mut s = 1.70158_f32;
        let mut inner_t = t / d / 2.0;
        if inner_t < 1.0 {
            s *= 1.525f32;
            return c / 2.0 * (inner_t * inner_t * ((s + 1.0) * inner_t - s)) + b;
        }
        inner_t -= 2.0;
        let post_fix: f32 = t;
        let inner_s = s * 1.525f32;
        return c / 2.0 * (post_fix * inner_t * ((inner_s + 1.0) * inner_t + inner_s) + 2.0) + b;
    }
}