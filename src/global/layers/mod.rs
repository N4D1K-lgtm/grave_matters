use bevy::render::view::visibility::RenderLayers;
use strum_macros::{EnumIter, EnumString};

pub const TOTAL_LAYERS: usize = 32;

#[derive(Debug, Clone, Copy, EnumIter, EnumString, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    Game,
    Ui,
    Background,
    Foreground,
}

pub trait RenderLayerExt {
    fn all() -> RenderLayers {
        let mut layers = RenderLayers::none();
        for layer_index in 0..TOTAL_LAYERS {
            layers = layers.with(layer_index);
        }
        layers
    }
}

impl RenderLayerExt for RenderLayers {}
