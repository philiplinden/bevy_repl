use bevy::prelude::*;
use bevy_repl::prompt::PromptPlugin;
use bevy_repl::test_support::{RendererUnderTest, TestAppBuilder};

pub struct PrettyRendererUT;
impl RendererUnderTest for PrettyRendererUT {
    fn name(&self) -> &'static str { "pretty" }
    fn add_to_app(&self, app: &mut App) {
        app.add_plugins(PromptPlugin::pretty());
    }
}

pub struct SimpleRendererUT;
impl RendererUnderTest for SimpleRendererUT {
    fn name(&self) -> &'static str { "simple" }
    fn add_to_app(&self, app: &mut App) {
        app.add_plugins(PromptPlugin::simple());
    }
}

#[cfg(test)]
mod smoke {
    use super::*;

    #[test]
    fn can_build_pretty_app() {
        let _app = TestAppBuilder::new(PrettyRendererUT).build();
    }

    #[test]
    fn can_build_simple_app() {
        let _app = TestAppBuilder::new(SimpleRendererUT).build();
    }
}
