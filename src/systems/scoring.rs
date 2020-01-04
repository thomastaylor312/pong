use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, World, Write, WriteStorage},
    ecs::Read,
    ui::UiText,
};
use crate::components::{Ball, ScoreBoard, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};
use crate::resources::{play_score_sound, Sounds};
use std::ops::Deref;
use rand::distributions::{Bernoulli, Distribution};
use rand::SeedableRng;
use rand::rngs::SmallRng;

#[derive(SystemDesc)]
pub struct ScoringSystem {
    sampler: Bernoulli,
    rand_gen: SmallRng,
}

impl Default for ScoringSystem {
    fn default() -> Self {
        ScoringSystem {
            sampler: Bernoulli::new(0.5).unwrap(),
            rand_gen: SmallRng::from_entropy(),
        }
    }
}

impl<'s> System<'s> for ScoringSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text,
        storage,
        sounds,
        audio_output
    ): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Limit to 999 so the text doesn't overlap
                scores.score_right = (scores.score_right + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                // TODO: Add random negative to Y velocity
                // Reverse direction and play the sound
                ball.velocity[0] = -ball.velocity[0];
                // Randomly choose a negative or positive flip
                if self.sampler.sample(&mut self.rand_gen) {
                    ball.velocity[1] = -ball.velocity[1]
                }
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
            }
        }
    }
}
