use super::{components::*, *};
use crate::arcade_game::SetupMapEvent;
use bevy::app::AppExit;
use iyes_loopless::state::NextState;
use kayak_ui::prelude::widgets::*;

pub fn spawn_menu(mut commands: Commands, preload_resource: Res<PreloadResource>) {
    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    let handle_click_close = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut exit: EventWriter<AppExit>| {
            match event.event_type {
                EventType::Click(..) => {
                    exit.send(AppExit);
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let handle_click_play = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut commands: Commands,
              mut ev_writer_setup: EventWriter<SetupMapEvent>| {
            match event.event_type {
                EventType::Click(..) => {
                    commands.insert_resource(NextState(GameState::InGame));
                    ev_writer_setup.send(SetupMapEvent);
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: preload_resource.images[0].clone(),
                    border: Edge::all(25.0),
                }}
                styles={KStyle {
                    width: Units::Pixels(350.0).into(),
                    height: Units::Pixels(512.0).into(),
                    left: Units::Stretch(1.0).into(),
                    right: Units::Stretch(1.0).into(),
                    top: Units::Stretch(1.0).into(),
                    bottom: Units::Stretch(1.0).into(),
                    padding: Edge::new(
                        Units::Pixels(20.0),
                        Units::Pixels(20.0),
                        Units::Pixels(50.0),
                        Units::Pixels(20.0),
                    ).into(),
                    ..KStyle::default()
                }}
            >
                <KImageBundle
                    image={KImage(preload_resource.images[1].clone())}
                    styles={KStyle {
                        width: Units::Pixels(310.0).into(),
                        height: Units::Pixels(104.0).into(),
                        top: Units::Pixels(25.0).into(),
                        bottom: Units::Pixels(25.0).into(),
                        ..KStyle::default()
                    }}
                />
                <TextWidgetBundle
                    text={TextProps {
                        content: "The Adventures of Gunther the Pig".into(),
                        alignment: Alignment::Middle,
                        size: 28.0,
                        ..Default::default()
                    }}
                    styles={KStyle {
                        width: Units::Pixels(310.0).into(),
                        height: Units::Pixels(78.0).into(),
                        top: Units::Stretch(0.5).into(),
                        bottom: Units::Stretch(0.5).into(),
                        ..KStyle::default()
                    }}
                />
                <MenuButtonBundle
                    button={MenuButton { text: "Play".into() }}
                    on_event={handle_click_play} />
                <MenuButtonBundle button={MenuButton { text: "Options".into() }} />
                <MenuButtonBundle
                    button={MenuButton { text: "Quit".into() }}
                    on_event={handle_click_close}
                />
            </NinePatchBundle>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}

pub fn despawn_menu(root_context: Query<Entity, With<KayakRootContext>>, mut commands: Commands) {
    if root_context.is_empty() {
        dbg!("kayak root context empty");
        return;
    }
    let root_context = root_context.single();
    commands.entity(root_context).despawn_recursive();
}
