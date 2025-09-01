use bevy::prelude::*;

pub struct LeptosBevyCanvasPlugin;
impl Plugin for LeptosBevyCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LeptosBevyCanvasCleanup>();
        app.add_systems(First, cleanup);
    }
}

#[derive(Event, Debug)]
pub struct LeptosBevyCanvasCleanup;
fn cleanup(
    mut cleanup_events: EventReader<LeptosBevyCanvasCleanup>,
    mut exit: EventWriter<AppExit>,
) {
    for _event in cleanup_events.read() {
        exit.write(AppExit::Success);
    }
}
