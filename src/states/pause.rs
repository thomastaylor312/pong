use amethyst::assets::ProgressCounter;
use amethyst::ecs::prelude::Entity;
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use amethyst::ui::UiLoader;

#[derive(Default)]
pub struct PausedState {
    ui: Option<Entity>,
}

impl SimpleState for PausedState {
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Go back to the game
                let _ = data.world.delete_entity(self.ui.unwrap());
                return Trans::Pop;
            }
        }

        // Escape isn't pressed, so we stay in this `State`.
        Trans::None
    }

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // TODO: Make a constructor for PausedState that accepts the prefab handle instead of loading each time
        let ui = data
            .world
            .exec(|loader: UiLoader<'_>| loader.load("ui/paused.ron", &mut ProgressCounter::new()));
        self.ui = Some(data.world.create_entity().with(ui).build())
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        data.data.update(&data.world);
        Trans::None
    }
}
