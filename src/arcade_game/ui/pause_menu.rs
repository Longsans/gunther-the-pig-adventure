use super::{components::*, *};
use crate::arcade_game::CleanupMapEvent;
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

    let handle_click_main_menu = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut commands: Commands,
              mut ev_writer_cleanup: EventWriter<CleanupMapEvent>| {
            match event.event_type {
                EventType::Click(..) => {
                    commands.insert_resource(NextState(GameState::MainMenu));
                    ev_writer_cleanup.send(CleanupMapEvent);
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let handle_click_resume = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut commands: Commands| {
            match event.event_type {
                EventType::Click(..) => commands.insert_resource(NextState(GameState::InGame)),
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
                        content: "Paused".into(),
                        alignment: Alignment::Middle,
                        size: 28.0,
                        ..Default::default()
                    }}
                    styles={KStyle {
                        width: Units::Pixels(310.0).into(),
                        height: Units::Pixels(78.0).into(),
                        top: Units::Stretch(0.6).into(),
                        bottom: Units::Stretch(0.4).into(),
                        ..KStyle::default()
                    }}
                />
                <MenuButtonBundle
                    button={MenuButton { text: "Resume".into() }}
                    on_event={handle_click_resume} />
                <MenuButtonBundle button={MenuButton { text: "Options".into() }} />
                <MenuButtonBundle
                    button={MenuButton { text: "Main Menu".into() }}
                    on_event={handle_click_main_menu}
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
