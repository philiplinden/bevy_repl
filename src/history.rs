use bevy::prelude::*;

pub(crate) struct ReplHistoryPlugin;

impl Plugin for ReplHistoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReplHistory>();
    }
}

#[derive(Resource, Default)]
struct ReplHistory {
    pub(crate) history: VecDeque<String>,
    pub(crate) history_index: usize,
}

impl ReplHistory {
    fn new() -> Self {
        Self { history: Vec::new() }
    }
}
