use super::UIAssets;
use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

#[derive(Default, Clone, PartialEq, Component)]
pub struct MenuButton {
    pub text: String,
}

impl Widget for MenuButton {}

#[derive(Bundle)]
pub struct MenuButtonBundle {
    pub button: MenuButton,
    pub styles: KStyle,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            styles: KStyle {
                bottom: Units::Pixels(20.0).into(),
                cursor: KCursorIcon(CursorIcon::Hand).into(),
                ..Default::default()
            },
            on_event: OnEvent::default(),
            widget_name: MenuButton::default().get_name(),
        }
    }
}

pub fn menu_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    ui_assets: Res<UIAssets>,
    menu_button_query: Query<&MenuButton>,
    state_query: Query<&ButtonState>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });

    let button_text = menu_button_query.get(entity).unwrap().text.clone();
    let button_image = ui_assets.images[super::BUTTON_INDEX].clone();
    let button_image_hover = ui_assets.images[super::HOVER_BUTTON_INDEX].clone();

    let on_event = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState>| {
            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    }
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    }
                    _ => {}
                }
            }
            (event_dispatcher_context, event)
        },
    );

    if let Ok(button_state) = state_query.get(state_entity) {
        let (button_image_handle, border) = if button_state.hovering {
            (button_image_hover, Edge::all(8.0_f32))
        } else {
            (button_image, Edge::all(4.0_f32))
        };

        let parent_id = Some(entity);
        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: button_image_handle,
                    border: border,
                }}
                styles={KStyle {
                    width: Units::Stretch(1.0).into(),
                    height: Units::Pixels(50.0).into(),
                    bottom: Units::Pixels(30.0).into(),
                    left: Units::Pixels(50.0).into(),
                    right: Units::Pixels(50.0).into(),
                    ..KStyle::default()
                }}
                on_event={on_event}
            >
                <TextWidgetBundle
                    styles={KStyle {
                        top: Units::Stretch(0.5).into(),
                        bottom: Units::Stretch(0.5).into(),
                        ..Default::default()
                    }}
                    text={TextProps {
                        alignment: Alignment::Middle,
                        content: button_text,
                        size: 20.0,
                        ..Default::default()
                    }}
                />
            </NinePatchBundle>
        };
    }
    true
}
