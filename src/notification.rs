//! The notification center: the rows of [`crate::notification_center`], kept in step with the
//! [`crate::ToastLog`]. Its look is the list's and the button's.

use bevy::input_focus::InputFocus;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy::ui_widgets::Activate;

use crate::model::clock_text;
use crate::skin::Role;
use crate::toast::{ToastKind, ToastLog, ToastLogRecord};

/// A [`crate::notification_center`]'s root. Query for it to find the center.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
pub struct NotificationCenter;

/// On a row [`update_notification_centers`] spawned: a button for a log entry, or the line that says
/// there is none. [`NotificationRow::entry`] is the id of the [`crate::ToastLogRecord`] it draws, so a
/// system that hears a press can say which entry it was for, however the log was capped.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotificationRow {
    entry: Option<u64>,
}

impl NotificationRow {
    /// The id of the log entry the row draws; `None` for the "No notifications yet" line.
    pub fn entry(&self) -> Option<u64> {
        self.entry
    }
}

/// What a row was drawn from: the entry as it was. The row is rebuilt only when the entry is no longer
/// equal to this.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct NotificationRowDrawn(ToastLogRecord);

/// On a button in a row: which entry it belongs to and the id it sends.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct NotificationActionButton {
    entry: u64,
    id: String,
}

/// A button of a [`crate::notification_center`] row was pressed. Sent once per press, observable
/// globally: `entry` is the [`crate::ToastLogRecord::id`] of the row's entry and `id` the id the
/// action was added with ([`crate::ToastLogRecord::add_action`]). A press on the row's words sends
/// nothing.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_gooey::*;
/// fn on_action(action: On<NotificationAction>) {
///     info!("{} on entry {}", action.id, action.entry);
/// }
/// ```
#[derive(Event, Clone, Debug, PartialEq, Eq)]
pub struct NotificationAction {
    /// The log entry whose row held the button.
    pub entry: u64,
    /// The id the action was added with.
    pub id: String,
}

/// A row button was activated: send its action. A global observer; a press on anything else (a
/// row's words) is not one of ours and sends nothing.
pub(crate) fn notification_on_activate(
    activate: On<Activate>,
    buttons: Query<&NotificationActionButton>,
    mut commands: Commands,
) {
    if let Ok(button) = buttons.get(activate.entity) {
        commands.trigger(NotificationAction {
            entry: button.entry,
            id: button.id.clone(),
        });
    }
}

/// A row's button is going: if it has focus, focus goes with it, so it never rests on nothing. A
/// global observer.
pub(crate) fn notification_button_removed(
    removed: On<Remove<NotificationActionButton>>,
    mut focus: ResMut<InputFocus>,
) {
    if focus.get() == Some(removed.entity) {
        focus.clear();
    }
}

/// The words of a row: `m:ss  words`, an alert's with `! ` before them, a repeat's with `  (x3)` after.
fn row_words(record: &ToastLogRecord) -> String {
    let mark = if record.kind == ToastKind::Alert {
        "! "
    } else {
        ""
    };
    let count = if record.count() > 1 {
        format!("  (x{})", record.count())
    } else {
        String::new()
    };
    format!(
        "{}  {mark}{}{count}",
        clock_text(record.seconds),
        record.text
    )
}

/// Spawns the row of one entry: a button of its words, then a button for each action.
fn spawn_row(commands: &mut Commands, record: &ToastLogRecord) -> Entity {
    let row = commands
        .spawn((
            Node {
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            bevy::ui_widgets::Button,
            Role::Button,
            Hovered::default(),
            TabIndex(0),
            AutoDirectionalNavigation::default(),
            NotificationRow {
                entry: Some(record.id()),
            },
            NotificationRowDrawn(record.clone()),
        ))
        .id();
    commands.spawn((
        Node {
            flex_grow: 1.0,
            ..default()
        },
        Text::new(row_words(record)),
        Role::ButtonText,
        ChildOf(row),
    ));
    for (label, id) in record.actions() {
        commands.spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            bevy::ui_widgets::Button,
            Role::Button,
            NotificationActionButton {
                entry: record.id(),
                id: id.clone(),
            },
            Hovered::default(),
            TabIndex(0),
            AutoDirectionalNavigation::default(),
            ChildOf(row),
            children![(Text::new(label.clone()), Role::ButtonText)],
        ));
    }
    row
}

/// Keeps every [`crate::notification_center`] as the [`crate::ToastLog`] says: a row for each entry,
/// the newest first, as `m:ss  words` (an alert's words start with `! `: the mark is not only a
/// colour; a count over 1 ends `  (x3)`) with a button for each of the entry's actions, or "No
/// notifications yet" while the log is empty. **A row whose entry did not change is left as it is**:
/// the same entity, with its focus and its hover; one whose entry changed is rebuilt, one whose entry
/// is gone is despawned, and a new one is spawned. Rebuilt when the log changes or a center is added;
/// a log that has not changed writes nothing. Runs in [`crate::GooeySystems::Style`] before the list
/// is laid out.
pub fn update_notification_centers(
    log: Option<Res<ToastLog>>,
    centers: Query<(Entity, Ref<NotificationCenter>)>,
    children: Query<&Children>,
    roles: Query<&Role>,
    rows: Query<(&NotificationRow, Option<&NotificationRowDrawn>)>,
    mut commands: Commands,
) {
    let Some(log) = log else {
        return;
    };
    for (center, marker) in &centers {
        if !log.is_changed() && !marker.is_added() {
            continue;
        }
        // The column the list holds its rows in is found by its role under the center.
        let mut stack = vec![center];
        let mut column = None;
        while let Some(at) = stack.pop() {
            if roles.get(at) == Ok(&Role::ListRows) {
                column = Some(at);
                break;
            }
            if let Ok(kids) = children.get(at) {
                stack.extend(kids.iter());
            }
        }
        let Some(column) = column else {
            // The list has not been spawned yet (the center was added this frame).
            continue;
        };
        let present: Vec<Entity> = children
            .get(column)
            .into_iter()
            .flat_map(|k| k.iter())
            .collect();
        let mut wanted: Vec<Entity> = Vec::new();
        let mut keep: Vec<Entity> = Vec::new();
        if log.is_empty() {
            // The one line says "none" and is kept while there are none.
            let line = present
                .iter()
                .copied()
                .find(|&kid| rows.get(kid).is_ok_and(|(row, _)| row.entry.is_none()));
            wanted.push(line.unwrap_or_else(|| {
                commands
                    .spawn((
                        Text::new("No notifications yet"),
                        Role::Label,
                        NotificationRow { entry: None },
                    ))
                    .id()
            }));
        } else {
            for record in log.entries().rev() {
                let same = present.iter().copied().find(|&kid| {
                    rows.get(kid).is_ok_and(|(row, drawn)| {
                        row.entry == Some(record.id()) && drawn.is_some_and(|d| &d.0 == record)
                    })
                });
                wanted.push(same.unwrap_or_else(|| spawn_row(&mut commands, record)));
            }
        }
        keep.extend(wanted.iter().copied());
        for kid in &present {
            if rows.contains(*kid) && !keep.contains(kid) {
                commands.entity(*kid).despawn();
            }
        }
        // Anything that is not a row stays, after them.
        let others: Vec<Entity> = present
            .iter()
            .copied()
            .filter(|kid| !rows.contains(*kid))
            .collect();
        let order: Vec<Entity> = wanted.into_iter().chain(others).collect();
        if order != present {
            commands.entity(column).replace_children(&order);
        }
    }
}
