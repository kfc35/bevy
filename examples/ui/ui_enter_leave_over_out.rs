//! Demonstrates Enter, Exit, Out, and Over events

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component, Default)]
struct OverCount(u32);

#[derive(Component, Default)]
struct EnterCount(u32);

#[derive(Component, Default)]
struct LeaveCount(u32);

#[derive(Component, Default)]
struct OutCount(u32);

#[derive(Component, Default)]
#[require(OverCount, EnterCount, LeaveCount, OutCount)]
struct EventCounter(String);

#[derive(Component)]
struct TextToUpdate(Entity);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|grand_parent| {
            let gp_text_entity = grand_parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: px(10),
                    left: px(10),
                    ..default()
                },
                Text::new("Red [Over: 0, Enter: 0, Leave: 0, Out: 0]"),
            )).id();
            let p_text_entity = grand_parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: px(40),
                    left: px(10),
                    ..default()
                },
                Text::new("Green [Over: 0, Enter: 0, Leave: 0, Out: 0]"),
            )).id();
            let b_text_entity = grand_parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: px(70),
                    left: px(10),
                    ..default()
                },
                Text::new("Indigo [Over: 0, Enter: 0, Leave: 0, Out: 0]"),
            )).id();
            grand_parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Percent(20.)),
                        border: UiRect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(1., 0., 0., 0.9)),
                    EventCounter("Red".to_string()),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                    TextToUpdate(gp_text_entity)
                ))
                .observe(handle_over)
                .observe(handle_enter)
                .observe(handle_leave)
                .observe(handle_out)
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            bottom: px(-50),
                            right: px(0),
                            padding: UiRect::all(Val::Px(50.)),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgba(0., 1., 0., 0.9)),
                        EventCounter("Green".to_string()),
                        Pickable {
                            should_block_lower: false,
                            is_hoverable: true,
                        },
                        TextToUpdate(p_text_entity)
                    ))    
                    .observe(handle_over)
                    .observe(handle_enter)
                    .observe(handle_leave)
                    .observe(handle_out);

                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            bottom: px(-50),
                            right: px(50),
                            padding: UiRect::all(Val::Px(50.)),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgba(0., 0., 1., 0.9)),
                        EventCounter("Indigo".to_string()),
                        Pickable {
                            should_block_lower: false,
                            is_hoverable: true,
                        },
                        TextToUpdate(b_text_entity)
                    ))    
                    .observe(handle_over)
                    .observe(handle_enter)
                    .observe(handle_leave)
                    .observe(handle_out);
                });
        });
}

fn handle_over(
    on_over: On<Pointer<Over>>,
    mut query: Query<(
        &EventCounter,
        &mut OverCount,
        &EnterCount,
        &LeaveCount,
        &OutCount,
        &TextToUpdate,
    )>,
    mut text_query: Query<&mut Text>
) {
    if let Ok((event_counter, mut over_count, enter_count, leave_count, out_count, text_to_update)) =
        query.get_mut(on_over.event_target())
    {
        over_count.0 += 1;
        text_query.get_mut(text_to_update.0).unwrap().0 = format!(
            "{} [Over: {}, Enter: {}, Leave: {}, Out: {}]",
            event_counter.0, over_count.0, enter_count.0, leave_count.0, out_count.0
        );
    }
}

fn handle_enter(
    on_over: On<Pointer<Enter>>,
    mut query: Query<(
        &EventCounter,
        &OverCount,
        &mut EnterCount,
        &LeaveCount,
        &OutCount,
        &TextToUpdate,
    )>,
    mut text_query: Query<&mut Text>
) {
    if let Ok((event_counter, over_count, mut enter_count, leave_count, out_count, text_to_update)) =
        query.get_mut(on_over.event_target())
    {
        enter_count.0 += 1;
        text_query.get_mut(text_to_update.0).unwrap().0 = format!(
            "{} [Over: {}, Enter: {}, Leave: {}, Out: {}]",
            event_counter.0, over_count.0, enter_count.0, leave_count.0, out_count.0
        );
    }
}

fn handle_leave(
    on_over: On<Pointer<Leave>>,
    mut query: Query<(
        &EventCounter,
        &OverCount,
        &EnterCount,
        &mut LeaveCount,
        &OutCount,
        &TextToUpdate,
    )>,
    mut text_query: Query<&mut Text>
) {
    if let Ok((event_counter, over_count, enter_count, mut leave_count, out_count, text_to_update)) =
        query.get_mut(on_over.event_target())
    {
        leave_count.0 += 1;
        text_query.get_mut(text_to_update.0).unwrap().0 = format!(
            "{} [Over: {}, Enter: {}, Leave: {}, Out: {}]",
            event_counter.0, over_count.0, enter_count.0, leave_count.0, out_count.0
        );
    }
}

fn handle_out(
    on_over: On<Pointer<Out>>,
    mut query: Query<(
        &EventCounter,
        &OverCount,
        &EnterCount,
        &LeaveCount,
        &mut OutCount,
        &TextToUpdate,
    )>,
    mut text_query: Query<&mut Text>
) {
    if let Ok((event_counter, over_count, enter_count, leave_count, mut out_count, text_to_update)) =
        query.get_mut(on_over.event_target())
    {
        out_count.0 += 1;
        text_query.get_mut(text_to_update.0).unwrap().0 = format!(
            "{} [Over: {}, Enter: {}, Leave: {}, Out: {}]",
            event_counter.0, over_count.0, enter_count.0, leave_count.0, out_count.0
        );
    }
}
