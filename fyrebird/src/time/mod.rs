use std::time::{Duration, Instant};

use specs::{Component, Read, VecStorage, World, WorldExt};
use specs::{Entities, Join, System, Write, WriteStorage};
use specs_derive::Component;

/// Global time resource to track delta time between frames
#[derive(Debug)]
pub struct Time {
    /// Program start time
    start_time: Instant,

    /// Last Frame Instant
    last_frame: Instant,

    /// Time elapsed since last frame in seconds
    pub delta_time: f32,
    /// Fixed timestep for physics and fixed updates
    pub fixed_timestep: f32,
    /// Accumulated time for fixed updates
    pub fixed_time_accumulator: f32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),

            delta_time: 0.0,
            fixed_timestep: 1.0 / 60.0,
            fixed_time_accumulator: 0.0,
        }
    }
}

impl Time {
    pub fn new() -> Self {
        Time::default()
    }

    /// Returns true if it's time for a fixed update
    pub fn should_fixed_update(&mut self) -> bool {
        self.fixed_time_accumulator >= self.fixed_timestep
    }

    /// Consumes one fixed timestep and returns the remainder
    pub fn consume_fixed_update(&mut self) {
        self.fixed_time_accumulator -= self.fixed_timestep;
    }

    /// Updates the time resource with a new delta time
    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta_time = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.fixed_time_accumulator += self.delta_time;
    }

    pub fn total_time(&self) -> f32 {
        let now = Instant::now();

        now.duration_since(self.start_time).as_secs_f32()
    }
}

/// Timer component for entities that need to track time
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Timer {
    /// Is the timer currently running
    pub running: bool,
    /// Duration in seconds
    pub duration: f32,
    /// Current elapsed time in seconds
    pub elapsed: f32,
    /// Should the timer loop when completed
    pub looping: bool,
    /// Has the timer completed (reached duration)
    pub completed: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            running: false,
            duration: 1.0,
            elapsed: 0.0,
            looping: false,
            completed: false,
        }
    }
}

impl Timer {
    /// Create a new timer with the specified duration
    pub fn new(duration: f32) -> Self {
        Self {
            running: true,
            duration,
            elapsed: 0.0,
            looping: false,
            completed: false,
        }
    }

    /// Create a new looping timer
    pub fn new_looping(duration: f32) -> Self {
        Self {
            running: true,
            duration,
            elapsed: 0.0,
            looping: true,
            completed: false,
        }
    }

    /// Start or resume the timer
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Pause the timer
    pub fn pause(&mut self) {
        self.running = false;
    }

    /// Reset the timer
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.completed = false;
    }

    /// Get normalized progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.elapsed / self.duration
    }
}

/// System that updates all timers based on the global time
pub struct TimeSystem;

impl<'a> System<'a> for TimeSystem {
    type SystemData = (Entities<'a>, Read<'a, Time>, WriteStorage<'a, Timer>);

    fn run(&mut self, (entities, time, mut timers): Self::SystemData) {
        // Update all timers using the global delta time
        for (_entity, timer) in (&entities, &mut timers).join() {
            if timer.running && !timer.completed {
                timer.elapsed += time.delta_time;

                // Check for completion
                if timer.elapsed >= timer.duration {
                    timer.completed = true;

                    // Handle looping timers
                    if timer.looping {
                        timer.elapsed %= timer.duration;
                        timer.completed = false;
                    }
                }
            }
        }
    }
}
