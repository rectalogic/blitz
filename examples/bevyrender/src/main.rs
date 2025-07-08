use std::ops::DerefMut;

use anyrender_bevy_vello::BevyVelloScenePainter;
use bevy::prelude::*;
use bevy_vello::{VelloPlugin, prelude::*};
use blitz_dom::DocumentConfig;
use blitz_html::HtmlDocument;
use blitz_paint::paint_scene;
use blitz_traits::shell::{ColorScheme, Viewport};

fn main() {
    let document = Document::new("examples/assets/border.html", 400, 300, 1.);
    App::new()
        .insert_non_send_resource(document)
        .add_plugins(DefaultPlugins)
        .add_plugins(VelloPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_document)
        .run();
}

struct Document {
    html: HtmlDocument,
    render_width: u32,
    render_height: u32,
    scale: f64,
}

impl Document {
    fn new(path: &str, width: u32, height: u32, scale: f64) -> Self {
        let file_content = std::fs::read_to_string(path).unwrap();

        // Create HtmlDocument
        let mut document = HtmlDocument::from_html(
            &file_content,
            DocumentConfig {
                viewport: Some(Viewport::new(
                    width * (scale as u32),
                    height * (scale as u32),
                    scale as f32,
                    ColorScheme::Light,
                )),
                ..Default::default()
            },
        );

        // Compute style, layout, etc for HtmlDocument
        document.as_mut().resolve();

        // Determine height to render
        let computed_height = document.as_ref().root_element().final_layout.size.height;
        let render_width = (width as f64 * scale) as u32;
        let render_height =
            ((computed_height as f64).max(height as f64).min(4000.0) * scale) as u32;

        Self {
            html: document,
            render_width,
            render_height,
            scale,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, VelloView));
    commands.spawn(VelloScene::new());
}

fn update_document(mut scene: Single<&mut VelloScene>, document: NonSend<Document>) {
    scene.reset();
    paint_scene(
        &mut BevyVelloScenePainter(scene.deref_mut()),
        document.html.as_ref(),
        document.scale,
        document.render_width,
        document.render_height,
    )
}
