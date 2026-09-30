pub enum FreeFallVelocity {
    VelocityAtTime {
        gravity: f32,
        time: f32,
    },

    HeightAfterTime {
        gravity: f32,
        time: f32,
    },

    VelocityFromHeight {
        gravity: f32,
        height: f32,
    },

    FallTimeFromHeight {
        gravity: f32,
        height: f32,
    },

    InstantaneousVelocity {
        gravity: f32,
        time: f32,
        speed: f32,
        direction: bool,
    },

    Displacement {
        gravity: f32,
        time: f32,
        speed: f32,
        direction: bool,
    },
}

impl FreeFallVelocity {
    pub fn calculate(&self) -> f32 {
        match self {
            Self::VelocityAtTime { gravity, time } => {
                gravity * time
            }

            Self::HeightAfterTime { gravity, time } => {
                gravity * time.powi(2) / 2.0
            }

            Self::VelocityFromHeight { gravity, height } => {
                (2.0 * gravity * height).sqrt()
            }

            Self::FallTimeFromHeight { gravity, height } => {
                (2.0 * height / gravity).sqrt()
            }

            Self::InstantaneousVelocity {
                gravity,
                time,
                speed,
                direction,
            } => {
                if *direction {
                    speed + gravity * time
                } else {
                    speed - gravity * time
                }
            }

            Self::Displacement {
                gravity,
                time,
                speed,
                direction,
            } => {
                if *direction {
                    speed * time + gravity * time.powi(2) / 2.0
                } else {
                    speed * time - gravity * time.powi(2) / 2.0
                }
            }
        }
    }
}