use bevy::{
    app::{App, AppExit, Plugin, Startup},
    input::InputPlugin,
    prelude::*,
    MinimalPlugins,
};

use super::{AutoplayPlugin, LoadFromFileAndPlay};

#[derive(Resource)]
struct TestSessionFilename(String);

#[derive(Message)]
pub enum TestResult {
    #[allow(dead_code)]
    Success,
    #[allow(dead_code)]
    Failure(String),
}

pub struct AutoplayTestPlugin(pub String);

impl Plugin for AutoplayTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MinimalPlugins, InputPlugin, AutoplayPlugin))
            .insert_resource(TestSessionFilename(self.0.clone()))
            .add_message::<TestResult>()
            .add_systems(Startup, playback_recording)
            .add_systems(Update, check_for_result);
    }
}

fn playback_recording(
    mut ev_load_play: MessageWriter<LoadFromFileAndPlay>,
    filename: Res<TestSessionFilename>,
    mut _time: ResMut<Time<Virtual>>,
) {
    // time.set_relative_speed(10.0); // TODO: Make this configurable
    ev_load_play.write(LoadFromFileAndPlay(filename.0.clone()));
}

fn check_for_result(mut exit: MessageWriter<AppExit>, mut ev_result: MessageReader<TestResult>) {
    if let Some(ev) = ev_result.read().next() {
        match ev {
            TestResult::Success => exit.write(AppExit::Success),
            TestResult::Failure(msg) => panic!("{}", msg),
        };
    }
}
