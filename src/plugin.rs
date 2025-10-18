use bevy::prelude::*;

pub struct LeptosBevyCanvasPlugin;
impl Plugin for LeptosBevyCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LeptosBevyCanvasCleanup>();
        app.add_systems(First, cleanup);
    }
}

#[derive(Message, Debug)]
pub struct LeptosBevyCanvasCleanup;
fn cleanup(
    mut cleanup_messages: MessageReader<LeptosBevyCanvasCleanup>,
    mut exit: MessageWriter<AppExit>,
) {
    for _ in cleanup_messages.read() {
        exit.write(AppExit::Success);
    }
}
