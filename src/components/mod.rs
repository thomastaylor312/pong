pub mod ball;
pub mod camera;
pub mod paddle;
pub mod score;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub use ball::{Ball, BALL_RADIUS, BALL_VELOCITY_X, BALL_VELOCITY_Y};
pub use paddle::{Paddle, Side, PADDLE_HEIGHT, PADDLE_WIDTH};
pub use score::{ScoreBoard, ScoreText};

