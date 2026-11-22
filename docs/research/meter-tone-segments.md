# Meters a game can tell apart: a tone per resource and notches that count

Researched 2026-11-22 (iteration 107). Every `meter()` fills in the skin's one `accent`, so a HUD with health, stamina and
mana draws three bars of one colour and a player tells them apart by position alone; and a bar of 100 is a smooth slab
that cannot say "6 of 10". Games do both (a colour per resource, notches per unit); a game built on this crate has to
hand-roll them today.

## Sources

1. Game Accessibility Guidelines, *Ensure no essential information is conveyed by a fixed colour alone* —
   https://gameaccessibilityguidelines.com/ensure-no-essential-information-is-conveyed-by-a-fixed-colour-alone/ (craft).
2. Hugo Martin, creative director of *DOOM Eternal*, on its HUD, in VG24/7's interview (quoted at
   https://mspoweruser.com/doom-eternal-ui-explained-big-bold-loud/) and Game Developer's *How Doom Eternal Owns the
   "Contrivances" of Video Games* (shipping game).
3. W3C, *Cascading Style Sheets Level 2 Revision 1*, §10.1 *Definition of "containing block"* (technical: where an absolute
   child's percentages and insets are measured); W3C WAI, *Understanding SC 1.4.11 Non-text Contrast* (technical).

## Craft principle

The guidelines: "use colour as a back-up for another means of communicating the information, such as text or a symbol,
pattern or shape", and "If color reliance is unavoidable, providing players the option to assign their own color choice
for individual game elements is important." A tone per bar is colour; the bar's position, length and the notches are the
pattern behind it, so a tone is never the only cue. The tone is data in the skin (`GooeySkin::tones`), so a player's own
palette is one skin change.

## How shipping games do it

Martin on *DOOM Eternal*: "The colour of the wheel is that way so that, in a millisecond glance, you know what ammo you
have, how much you have." Each ammunition type has its own colour; health is blue and armour green in its glows. A resource
is read by its colour first and its length second.

## Model adopted

- **M1. `MeterTone(&'static str)`**, a component beside `Meter`: the fill takes the skin's tone of that name
  (`GooeySkin::tones`), else one of the skin's own colours (`danger`, `accent`, ...), the same names `tone_problems` takes.
  An unknown name warns once and the fill stays `accent`. No tone, no change: `accent`.
- **M2. The trail of a toned meter is the tone lifted toward `text`** (`model::lift_toward`), because a red health bar with a
  `danger` trail reads as one longer bar and the lost length would be invisible. An untoned meter keeps `danger`.
- **M3. `MeterSegments(u32)`**: `n >= 2` draws `n - 1` notches, each one line weight wide, in the track's colour, standing at
  `i / n` of the track's inside and over the fill, so a partly full unit shows partly full. 0 and 1 are none. More than
  `METER_MAX_SEGMENTS` warns and draws none. The notches are one `Role::MeterNotchLane` child (absolute, inset by the
  frame's padding so its percentages are the fill's) of `Role::MeterNotch` children, `Pickable::IGNORE`, hidden from the
  accessibility tree (the meter's value is the number).
- **M4. A host checks its own tone** with `GooeySkin::tone_problems` (fill on `surface_raised` at 3:1); the crate does not
  hold a tone it does not know.
- **M5. Cost.** Notches are built when `MeterSegments` changes and written by `apply_skin` like any part; a settled HUD
  writes nothing.

## What great looks like

A Souls or Hollow Knight bar: three differently coloured bars in a corner, each readable by hue at a glance; the stamina
bar cut into ten equal cells by hairline gaps of the track's own colour, a cell half full showing half. The lost length
lingers as a lighter band of the bar's own hue.

## Parameters

| Name | Value | Why |
|---|---|---|
| `METER_MAX_SEGMENTS` | 64 | past it a notch has under a line weight of fill between it and the next at 160 px |
| `METER_TRAIL_LIFT_L` | 22 CIE L\* | a step well over `STATE_STEP` (4), so a band reads as another colour, not shading |
| notch width | one line weight | the frame's weight: one visual language |

## Validation targets

1. `MeterTone("health")` paints the fill in `skin.tone("health")`; the trail is that colour lifted, not `danger`.
2. A skin change (a theme step) repaints it; removing the component gives `accent` back.
3. `MeterSegments(10)` makes nine notches at `i/10` of the fill's own width (a laid-out test), each a whole line weight wide.
4. Changing 10 to 4 rebuilds; removing the component removes the lane; 0, 1 and over the cap draw none.
5. An unknown tone warns once and the fill is `accent`.
6. Nothing is written for 30 idle frames; `Children[0]` of a meter is still its fill.

## Deviations

None from a citation; the numbers are design defaults.

## Pending index

home-still's distill answered 500 `gpu busy: yielded to inkling-2-mini@big` for the whole pass, so no corpus stem was
searched: the three sources above were read from the web. The Game Accessibility Guidelines pages and the *DOOM Eternal*
interview are to be indexed.
