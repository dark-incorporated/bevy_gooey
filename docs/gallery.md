# Gallery

Every picture here is a real capture of an example in [`examples/`](../examples), made with the crate's own capture path: [`media/README.md`](media/README.md) has each one's command, date and the version it shows. The still and the animation at the top of the [README](../README.md) are the gallery example itself; GIFs for the other widgets are in the page of the widget (the [guide index](guide/README.md)), and the toast actions and the journal are [in the toasts page](guide/toasts.md).

## Moving around and typing

![The gallery's settings panel: the ring moves from Quit onto the lit Audio tab, Right turns to Display (two choices), Right to Controls (two rebinding rows, Jump and Use) and Right wraps back to Audio, Down enters the page on the Volume slider, and Ctrl+PageDown turns to Display and then Controls with the ring landing on each new page's first widget](media/tabs.gif)

*The tabs: Right on a focused tab, then Ctrl+PageDown from inside a page (a pad's RB). The panel never moves.*

![The gallery's settings panel after the Controls tab is clicked: the ring goes down to Jump, whose keycap says Space; Enter lights the keycap amber with "Press a key, Esc exits", Q turns it to Q, and Down to Use (A), whose keycap lights with "Press a button, Menu exits" until Escape puts it back](media/binding.gif)

*Rebinding: `binding("Jump", Binding::Key(KeyCode::Space))` waits for the next press and says it once as a `ValueChange<Binding>`. The panel never moves, and nothing else hears the key.*

![The shortcut badges example: a Level editor panel with the commands New, Open, Save, Save all, Undo, Redo (dimmed) and Help, the ring on New; holding Alt shows a white chip with the chord at the right of New (Ctrl+N), Open, Save, Save all (Ctrl+Shift+S) and Undo, none on Redo or Help, and letting go hides them; Tab moves the ring to Open, Alt shows the chips again, Shift added hides them (Alt+Shift is not the hold) and letting go of Shift shows them again](media/shortcut-badges.gif)

*Shortcuts: `(button("Save"), Shortcut::primary(KeyCode::KeyS))` and holding Alt, or any `Reveal::Key`, for 0.3 s (`ShortcutBadges`, `BadgesShown`) shows the chord on the button, inside its frame so the ring stays whole, and `ShortcutLegend` lists the chords that act on nothing; a `binding` row whose value is a `Binding::Chord` captures a chord, `model::shortcut_conflicts` finds two commands on one, `Shortcut::just_pressed` runs it, `Tooltip::with_shortcut` shows it; `BindingSettings { chord_glyphs: ChordGlyphs::Words, .. }` spells every chord in ASCII (`Cmd+S`) for a face with none of a mac's symbols.*

![The gallery's Load game list with the Save as field under it: the field rests as a quiet outline over the muted hint "Name this save", its frame turns blue with a caret while "Aria" is typed, Enter shows the toast "Saved as Aria" at the window's corner, and Up moves the ring onto the save slot Cellar, 01:48 with the field back to its quiet outline and its text kept](media/text-input.gif)

*Text input: `text_input("Save as", "")` takes the keys the moment it has focus, says each edit once as a `ValueChange<String>` and the end of the edit (Enter, or focus leaving) with `is_final`. Up leaves it: a field never traps focus, and the row never moves while the player types.*

![The inspector example: a Turret panel built from a reflected component with no panel written for it: an Online switch, a Mode choice, a Rate number field, an Aim vector of X, Y and Z, a Tint colour field and a Barrel section with its Length and Label; the ring goes to Online and Enter turns it off, Right steps Mode from Track to Fire, and Up on Rate steps it from 6.0 to 7.8 while the readout beside it shows each change as the game sees it; a field's doc comment is its tooltip ("Tracks targets.", "Shots a second.")](media/inspector.gif)

*Inspector: `inspector(InspectTarget::component::<Turret>(entity))` makes a `toggle`, `choice`, `number_field`, `vec_field`, `color_field`, `text_input` or `section` for each reflected field (a `#[reflect(@RangeInclusive::<f32>::new(0.0, 20.0))]` is its range, a doc comment its tooltip), writes an edit back through `ReflectMut` and sends one `InspectorChange { path }`; a value the game writes moves the row.*

## Player options and skins

![The options example: a panel with the rows Text size, Interface size, Controller and Theme and the legend in the bottom-left corner; the ring on Text size steps 100%, 125%, 150% and 200% and the panel and the legend grow with each step, then back to 100%; Interface size steps to 125% and 150% and back; a pad button turns the legend to "A Select, B Back" and Controller steps to PlayStation, where it reads "Cross Select, Circle Back", then Switch and back to Xbox; Theme steps to Parchment and to High contrast, where the screen turns black with white frames, and back](media/options.gif)

*Player options: `text_size`, `interface_size`, `pad_style` and `skin_preset` each edit the crate's own settings and follow them, so a loaded save shows on them and the whole screen answers the frame the player steps one.*

![The gallery's settings panel, Load game list and legend: the Audio tab turns to Display and the ring goes down to Quality and Theme; Right steps Theme from Dark to Parchment, where the whole screen turns cream and brown, then to High contrast, where it turns black with white frames, amber fills and a cyan ring, and Left twice steps it back to Parchment and Dark; every widget changes in the same frame and nothing moves](media/theme.gif)

*Theme: `skin_preset("Theme")` is the high-contrast mode Xbox Accessibility Guideline 102 asks of a game, in one line: every colour at 7:1 or more. A step writes the colours and no size, so the player's text size stays.*

![The scope example: an Options panel with Text size, Theme, Subtitles, Volume and Play beside a smaller cream and brown Preview panel; Theme steps to Parchment and High contrast and Text size to 125% and 150%, the Options panel changing each time while the Preview stays cream, brown and at the default size; the ring goes down to Play and across to the Preview's own Play, where it is blue, and Enter takes the scope off: the Preview turns black with white frames, amber fills and 150% text, with a cyan ring](media/scope.gif)

*Scoped skin: `SkinScope(GooeySkin::parchment())` on the Preview panel is the only thing that keeps it in its own look; removing it hands the panel back to the player's skin in the same frame.*

![The density example: an Inspector panel with a Compact switch, Theme, Visible, Space, Opacity and two buttons in the dark skin; Enter turns Compact on and the panel gets shorter in the same frame, Down and Right step Theme to Parchment, High contrast (white frames, amber fills) and Tool (blue fills), Up and Enter turn Compact off, where the rows spread out again, and on](media/density.gif)

*Density: `skin.density = Density::Compact` is one field; every row halves its padding and stands at least 24 px high (WCAG 2.2 SC 2.5.8's floor, still checked), and the colours, the ring and the keys do not change: here the panel goes from 528 px to 456 px at the same 20 px text. `SkinPreset::Tool` is a blue-on-grey look with 14 px text and tighter sizes: the gallery's settings panel is 294 px tall in it against 520 px in dark.*

![The ladder example: a card "Surface ladder" in dark whose four nested cards, Surface 1 to Surface 4, each step lighter than the one it sits in and each labelled with its CIE L* and the step it makes (12.9, 18.3, 23.6, 31.6), the Theme row in the innermost; Right steps Theme to Parchment (cream cards on tan), High contrast (grey steps on black) and Tool (tighter cards in blue-grey), and Left steps it back to Dark](media/ladder.gif)

*Tokens: `surface(n)` is step `n` of `skin.surfaces`, and `problems()` reports an `ElevationStep` when two neighbours are closer than ΔL\* 2 (the eye's threshold for a flat field); the four presets sit at 5 or more. The cards' padding and corners are `skin.space(3)` and `skin.corner(1)`.*

## Menus, popovers and the palette

![The popover example: a menu bar with File, View and Edit over a canvas; Enter opens File's menu under it with the ring on New, Down twice moves it to Save and Escape closes it with the ring back on File; Edit at the window's right edge opens its menu ending at the button's edge; a right-click near the bottom-right corner opens a context menu whose corner is at the pointer, and a click on the empty ground closes it](media/popover.gif)

*Popovers: `popover_modal(file_button, Placement::Below)` opens a menu under its anchor that holds focus and gives it back; at the window's edge it slides or flips and never leaves the window, and Escape or a press anywhere else closes it with one `PopoverClosed`.*

![The menus example: a bar with File, Edit, View and Help; F10 puts the ring on File, Down opens its menu with Ctrl+N, Ctrl+O and Ctrl+S in a column at the right edge, Down reaches Recent and Right opens its submenu of three levels, Left closes it, Escape closes File, Right then Down opens Edit where the disabled Redo shows "Nothing to redo" in a tooltip beside the menu, and Escape closes it](media/menus.gif)

*Menus: `menu_bar(children![menu_title("File", children![menu_item("Open...").with_shortcut(..), ..])])`, F10 and the arrows to walk it, Right into a submenu, Escape one level at a time; a disabled item says why.*

![The context example: a right-click on a crate opens its menu at the pointer, with Locked ticked and the chord I beside Inspect; a right-click on the crate in the bottom-right corner opens the whole menu above and to the left of the pointer; Down and Shift+F10 open an inventory row's menu below the row, Right on Move to opens its submenu, and Enter on Chest 2 says at the bottom which row was moved](media/context-menu.gif)

*Context menus: `ContextMenu(menu)` on a crate, a row or any entity; a right-click opens the menu at the pointer (flipping at the window's edge), Shift+F10 or the Menu key below the focused widget, and `ContextTarget` says which thing a command was for.*

![The palette example: Ctrl+Shift+P opens a field over a small editor's commands, "opn prj" lights File: Open Project with its chord Ctrl+O at the row's end, Down and Up move the lit row and Enter runs it, and the line behind reads "Last command: File: Open Project"; opened again it lists that command first, "sav" shows File: Save and File: Save As... dimmed with "No project is open" where their chords would be, Down puts the ring on the second and Escape closes the palette](media/command-palette.gif)

*The command palette: `CommandList` holds the commands, `command_palette()` is the modal, "opn prj" is a fuzzy match, Up and Down move the lit row while focus stays in the field, and Enter or a click sends one `RunCommand`; a command that is off shows why and runs nothing.*

## Motion and the dialog

![The gallery's Health meter over the Audio tab: a hit takes the amber fill from full to 60% in one frame and the lost 40% stays behind it in red for a beat and then drains to the fill; a second hit does the same from 60% to 20%; a heal eases the fill back up to 60% with no red; a last hit leaves a red band again](media/meter.gif)

*Motion: a hit drops a `meter()`'s fill to the new value in that frame and leaves the lost length behind it in the skin's danger colour for 0.45 s before it drains, a heal rises up with no trail, a dialog's card rises 12 px and a menu or popover opens 6 px out of its anchor, each on a spring from the skin's motion palette (`GooeySkin::motion`) that carries on from where it is drawn if it is interrupted, all on real time with nothing to write; `MotionSettings::reduced()` is the player's "reduce motion" (nothing travels, fades stay) and `off()` makes every one instant, and `MotionTarget` does the same for your own panel. Recorded with `GOOEY_MOTION=2` and `GOOEY_KEYS=hit,hit,heal,hit` (twice slower, so the trail reads in a GIF); the dialog's rise is in the [motion guide](guide/motion.md), the inventory's demo in its [guide](guide/inventory.md), the radial menu's in [its own](guide/radial.md).*

![The Quit dialog in the dark and the parchment preset: the title "Leave the game?", a line of body, and the choices Leave and Stay in a framed panel over the dimmed screen, the ring on Stay](media/dialog.png)

*The dialog in both presets, 540 px wide here, the width of its line of body (a dialog is 420 to 720 px at the default size and grows with the text size); its edge is the skin's `outline` colour.*

![A dialog holding a toggle and a slider whose labels are longer than its panel, in the dark and the parchment preset: both labels break between words inside the frame, the switch centred against its two lines](media/dialog-label.png)

*Its panel is bounded, so a control's label too long for it breaks between words inside the frame; on a screen, where the panel is as wide as its widest child, the same label stays on one line.*

## Scrolling and the journal

![The scroll example: a scroll_region of thirty buttons beside a scroll_viewport that starts on a Volume slider, with the line ScrollPosition.y under it; Tab, Page Down, End and Home move the ring down and back up the region and the column follows; in the right panel Page Down and End scroll the column (the line reads about 130, then about 1230), Home goes back to the slider, and End and Home on the slider set it to 10 and 0 while the column stays put](media/scroll.gif)

*Scroll regions: `scroll_region(..)` and `scroll_viewport(..)` take PageUp, PageDown, Home and End unless the focused widget keeps them (a slider does). [Layout](guide/layout.md#a-scrolling-column-that-fills-its-parent).*

![The notifications example: Delete item spawns the toast "Item deleted" with an Undo button and a journal row with an Undo of its own, Save game pressed three times shows one "Game saved x3" while the journal lists all three, Break something shows a sticky red toast wrapped over four lines with Show and Dismiss buttons, and Tab walks the ring down the journal to the Undo on "Item deleted" where Enter makes the game hear "undo (entry 1)"](media/notifications.gif)

*Toast actions and the notification centre: a toast and its journal row are one entry. [Toasts](guide/toasts.md).*
