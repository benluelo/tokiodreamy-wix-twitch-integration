use bevy::{
    core_pipeline::{self, AlphaMask3d, Opaque3d, Transparent3d},
    prelude::*,
    render::{
        camera::{ActiveCamera, CameraTypePlugin, RenderTarget},
        render_graph::{self, NodeRunError, RenderGraph, RenderGraphContext, SlotValue},
        render_phase::RenderPhase,
        renderer::RenderContext,
        RenderApp, RenderStage,
    },
    window::{CreateWindow, PresentMode, WindowId},
};
use crossbeam::channel::{bounded, Receiver, Sender};

#[tokio::main]
async fn runtime(sender: Sender<MessagesFromServer>, receiver: Receiver<MessagesToServer>) {

}

/// This example creates a second window and draws a mesh from two different cameras, one in each window
fn main() {
    App::new()
        // ClearColor must have 0 alpha, otherwise some color will bleed through
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins)
        .add_plugin(SecondWindowCameraPlugin)
        .add_startup_system(setup)
        .add_startup_system(create_new_window)
        .add_system(button_system)
        .add_system(read_stream)
        .run();
}

struct SecondWindowCameraPlugin;
impl Plugin for SecondWindowCameraPlugin {
    fn build(&self, app: &mut App) {
        // adds the `ActiveCamera<SecondWindowCamera3d>` resource and extracts the camera into the render world
        app.add_plugin(CameraTypePlugin::<SecondWindowCamera>::default());

        let render_app = app.sub_app_mut(RenderApp);

        // add `RenderPhase<Opaque3d>`, `RenderPhase<AlphaMask3d>` and `RenderPhase<Transparent3d>` camera phases
        render_app.add_system_to_stage(RenderStage::Extract, extract_second_camera_phases);

        // add a render graph node that executes the 3d subgraph
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        let second_window_node = render_graph.add_node("second_window_cam", SecondWindowDriverNode);
        render_graph
            .add_node_edge(
                core_pipeline::node::MAIN_PASS_DEPENDENCIES,
                second_window_node,
            )
            .unwrap();
        render_graph
            .add_node_edge(core_pipeline::node::CLEAR_PASS_DRIVER, second_window_node)
            .unwrap();
    }
}

struct SecondWindowDriverNode;
impl render_graph::Node for SecondWindowDriverNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        _: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        if let Some(camera) = world.resource::<ActiveCamera<SecondWindowCamera>>().get() {
            graph.run_sub_graph(
                core_pipeline::draw_3d_graph::NAME,
                vec![SlotValue::Entity(camera)],
            )?;
        }

        Ok(())
    }
}

fn extract_second_camera_phases(
    mut commands: Commands,
    active: Res<ActiveCamera<SecondWindowCamera>>,
) {
    if let Some(entity) = active.get() {
        commands.get_or_spawn(entity).insert_bundle((
            RenderPhase::<Opaque3d>::default(),
            RenderPhase::<AlphaMask3d>::default(),
            RenderPhase::<Transparent3d>::default(),
        ));
    }
}

#[derive(Component, Default)]
struct SecondWindowCamera;

fn create_new_window(mut create_window_events: EventWriter<CreateWindow>, mut commands: Commands) {
    let window_id = WindowId::new();

    // sends out a "CreateWindow" event, which will be received by the windowing backend
    create_window_events.send(CreateWindow {
        id: window_id,
        descriptor: WindowDescriptor {
            // width: 800.,
            // height: 600.,
            present_mode: PresentMode::Immediate,
            title: "Second window".to_string(),
            transparent: true,
            decorations: false,
            ..default()
        },
    });

    // second window camera
    commands.spawn_bundle(UiCameraBundle::default());
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Deref)]
struct StreamReceiver(Receiver<u32>);

// This system reads from the receiver and sends events to Bevy
fn read_stream(receiver: ResMut<StreamReceiver>, mut events: EventWriter<()>) {
    for from_stream in receiver.try_iter() {
        events.send(());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let (client_tx, client_rx) = bounded::<()>(10);
    let (server_tx, server_rx) = bounded::<()>(10);
    std::thread::spawn(move || runtime(client_tx, server_rx));

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}
