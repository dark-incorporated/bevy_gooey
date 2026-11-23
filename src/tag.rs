//! `tag`: a short word on a plate whose colour names a kind of thing (`docs/research/tag.md`).
//!
//! A tag is read-only: no input, no focus. [`TagTone`] names the plate's colour and the skin picks
//! the ink from it, so a host's tone is never unreadable; the words are what say the kind, so the
//! colour is never the only cue (WCAG 2.2 SC 1.4.1). A tag whose words are empty is not drawn:
//! [`update_tags`] gives it `Display::None` and gives it back with the words.

use bevy::prelude::*;

/// A [`tag()`](crate::tag)'s root, so a host can find one and [`update_tags`] can hide an empty one.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component, Debug, Default, Clone)]
pub struct Tag;

/// The name of a tag's colour: a tone of [`GooeySkin::tones`](crate::GooeySkin::tones), else one of
/// the skin's own colours (`"accent"`, `"danger"`, `"warning"`, `"live"`, ...), as a
/// [`MeterTone`](crate::MeterTone) is resolved. No tone, or a name the skin does not know, is the
/// quiet plate: [`GooeySkin::surface_raised`](crate::GooeySkin::surface_raised) with the text's ink.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_gooey::*;
/// # fn ui(mut commands: Commands) {
/// commands.spawn((tag("ELITE"), TagTone("danger")));
/// # }
/// ```
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component, Debug, PartialEq, Clone)]
pub struct TagTone(pub &'static str);

/// Hide each [`Tag`] whose words are empty (`Display::None`) and show it again when it has some,
/// writing a node only on a difference.
pub fn update_tags(mut tags: Query<(&mut Node, &Children), With<Tag>>, texts: Query<&Text>) {
    for (node, children) in &mut tags {
        let empty = children
            .iter()
            .find_map(|child| texts.get(child).ok())
            .is_none_or(|text| text.0.is_empty());
        let wanted = if empty { Display::None } else { Display::Flex };
        if node.display != wanted {
            let mut node = node;
            node.display = wanted;
        }
    }
}
