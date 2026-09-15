use std::num::NonZeroU32;

#[derive(Clone, Copy)]
pub struct RateLimitSettings {
    pub general_per_minute: NonZeroU32,
    pub login_per_minute: NonZeroU32,
    pub register_per_hour: NonZeroU32,
}
