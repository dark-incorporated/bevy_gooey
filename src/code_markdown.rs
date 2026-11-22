//! Markdown as [`Highlights`] (#114): a line scanner for the marks a writer needs to see, so a
//! prose pane reads as Markdown without a parser or a grammar. It is engine-free; the editor
//! calls it for a [`CodeMarkdown`](crate::CodeMarkdown) editor and it is public for a host that
//! wants the same spans elsewhere.
//!
//! What it knows, in CommonMark's terms: ATX headings, fenced code (the fence carries across
//! lines), block quotes, bullet and ordered list markers, thematic breaks, code spans, emphasis
//! and strong emphasis, and inline links. Anything else is plain. It never fails: text it cannot
//! read as Markdown is left unstyled, and an unclosed mark ends at the end of its line.
//!
//! The roles are the skin's own ([`GooeySkin::syntax`](crate::GooeySkin::syntax)): a heading is a
//! [`SyntaxRole::Keyword`], code a [`SyntaxRole::String`], emphasis a [`SyntaxRole::Emphasis`],
//! strong a [`SyntaxRole::Strong`], a link's words a [`SyntaxRole::Reference`] and its address a [`SyntaxRole::Comment`], a list
//! marker a [`SyntaxRole::Number`], and quote marks, rules and fences a [`SyntaxRole::Comment`].

use std::ops::Range;

use crate::code_decor::{Highlights, SyntaxRole};

/// The spans of `text`, a CommonMark document, as [`Highlights`] of `revision`.
///
/// ```
/// use bevy_gooey::{SyntaxRole, markdown_highlights};
/// let text = "# Title\nplain `code` text";
/// let spans = markdown_highlights(text, 1);
/// assert_eq!(spans.spans()[0], (0..7, SyntaxRole::Keyword));
/// assert!(spans.spans().iter().any(|(r, role)| &text[r.clone()] == "`code`" && *role == SyntaxRole::String));
/// ```
pub fn markdown_highlights(text: &str, revision: u64) -> Highlights {
    let mut spans: Vec<(Range<usize>, SyntaxRole)> = Vec::new();
    let mut fence: Option<(u8, usize)> = None;
    let mut at = 0;
    for raw in text.split_inclusive('\n') {
        let body = raw.trim_end_matches(['\n', '\r']);
        let line = at..at + body.len();
        at += raw.len();
        let indent = body.len() - body.trim_start_matches(' ').len();
        let rest = &body[indent.min(body.len())..];

        if let Some((mark, count)) = fence {
            let closes = rest.bytes().take_while(|&b| b == mark).count() >= count
                && rest.trim_start_matches(mark as char).trim().is_empty();
            let role = if closes {
                SyntaxRole::Comment
            } else {
                SyntaxRole::String
            };
            spans.push((line, role));
            if closes {
                fence = None;
            }
            continue;
        }
        if indent < 4
            && let Some(&mark) = rest.as_bytes().first()
            && (mark == b'`' || mark == b'~')
        {
            let count = rest.bytes().take_while(|&b| b == mark).count();
            if count >= 3 {
                fence = Some((mark, count));
                spans.push((line, SyntaxRole::Comment));
                continue;
            }
        }
        if indent < 4 && is_heading(rest) {
            spans.push((line, SyntaxRole::Keyword));
            continue;
        }
        if indent < 4 && is_rule(rest) {
            spans.push((line, SyntaxRole::Comment));
            continue;
        }

        // Block markers at the start of the line, then the inline marks of what follows them.
        let mut start = line.start + indent.min(body.len());
        let mut tail = rest;
        loop {
            if let Some(after) = tail.strip_prefix('>') {
                spans.push((start..start + 1, SyntaxRole::Comment));
                let skip = 1 + after.len() - after.trim_start_matches(' ').len();
                start += skip;
                tail = &tail[skip..];
                continue;
            }
            let marker = list_marker(tail);
            if marker > 0 {
                spans.push((start..start + marker, SyntaxRole::Number));
                let skip =
                    marker + (tail.len() - marker) - tail[marker..].trim_start_matches(' ').len();
                start += skip;
                tail = &tail[skip..];
            }
            break;
        }
        inline(tail, start, &mut spans);
    }
    Highlights::new(revision, spans)
}

/// An ATX heading: one to six `#`, then a blank or the end of the line.
fn is_heading(rest: &str) -> bool {
    let hashes = rest.bytes().take_while(|&b| b == b'#').count();
    (1..=6).contains(&hashes)
        && rest[hashes..]
            .chars()
            .next()
            .is_none_or(char::is_whitespace)
}

/// A thematic break: three or more of `-`, `*` or `_`, alone with blanks.
fn is_rule(rest: &str) -> bool {
    let Some(mark) = rest.chars().next().filter(|c| matches!(c, '-' | '*' | '_')) else {
        return false;
    };
    let mut count = 0;
    for c in rest.chars() {
        if c == mark {
            count += 1;
        } else if c != ' ' && c != '\t' {
            return false;
        }
    }
    count >= 3
}

/// The length of a list marker at the start of `tail` (`-`, `*`, `+` or digits and `.`/`)`),
/// followed by a blank; 0 when there is none.
fn list_marker(tail: &str) -> usize {
    let bytes = tail.as_bytes();
    let length = match bytes.first() {
        Some(b'-' | b'*' | b'+') => 1,
        Some(b'0'..=b'9') => {
            let digits = bytes.iter().take_while(|b| b.is_ascii_digit()).count();
            match bytes.get(digits) {
                Some(b'.' | b')') if digits <= 9 => digits + 1,
                _ => return 0,
            }
        }
        _ => return 0,
    };
    if bytes.get(length).is_some_and(|&b| b == b' ' || b == b'\t') {
        length
    } else {
        0
    }
}

/// Code spans, emphasis and links in `text`, which starts at byte `base` of the document.
fn inline(text: &str, base: usize, spans: &mut Vec<(Range<usize>, SyntaxRole)>) {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'`' => {
                let run = bytes[i..].iter().take_while(|&&b| b == b'`').count();
                match find_run(bytes, i + run, b'`', run) {
                    Some(end) => {
                        spans.push((base + i..base + end + run, SyntaxRole::String));
                        i = end + run;
                    }
                    None => i += run,
                }
            }
            mark @ (b'*' | b'_') => {
                let run = bytes[i..].iter().take_while(|&&b| b == mark).count();
                let word_before = i > 0 && bytes[i - 1].is_ascii_alphanumeric();
                let opens = bytes.get(i + run).is_some_and(|b| !b.is_ascii_whitespace())
                    && !(mark == b'_' && word_before);
                let width = run.min(2);
                match opens
                    .then(|| find_run(bytes, i + run, mark, width))
                    .flatten()
                {
                    Some(end) if end > i + run => {
                        let role = if width == 1 {
                            SyntaxRole::Emphasis
                        } else {
                            SyntaxRole::Strong
                        };
                        spans.push((base + i..base + end + width, role));
                        i = end + width;
                    }
                    _ => i += run,
                }
            }
            b'[' => match link(bytes, i) {
                Some((text_end, url_end)) => {
                    spans.push((base + i..base + text_end, SyntaxRole::Reference));
                    spans.push((base + text_end..base + url_end, SyntaxRole::Comment));
                    i = url_end;
                }
                None => i += 1,
            },
            _ => i += 1,
        }
    }
}

/// The start of the next run of exactly `width` of `mark` at or after `from`, none longer.
fn find_run(bytes: &[u8], from: usize, mark: u8, width: usize) -> Option<usize> {
    let mut i = from;
    while i < bytes.len() {
        if bytes[i] == b'\\' && mark != b'`' {
            i += 2;
            continue;
        }
        if bytes[i] == mark {
            let run = bytes[i..].iter().take_while(|&&b| b == mark).count();
            if run == width {
                return Some(i);
            }
            i += run;
        } else {
            i += 1;
        }
    }
    None
}

/// `[words](address)` at `open`: where the words end (after the `]`) and the address ends.
fn link(bytes: &[u8], open: usize) -> Option<(usize, usize)> {
    let mut depth = 0usize;
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 1,
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        i += 1;
    }
    if i >= bytes.len() || bytes.get(i + 1) != Some(&b'(') {
        return None;
    }
    let close = bytes[i + 2..].iter().position(|&b| b == b')')?;
    Some((i + 1, i + 2 + close + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roles(text: &str) -> Vec<(&str, SyntaxRole)> {
        markdown_highlights(text, 0)
            .spans()
            .iter()
            .map(|(r, role)| (&text[r.clone()], *role))
            .collect()
    }

    #[test]
    fn a_heading_is_one_keyword_span() {
        assert_eq!(
            roles("## Two words\nbody"),
            [("## Two words", SyntaxRole::Keyword)]
        );
        assert!(roles("#nospace").is_empty());
        assert!(roles("####### seven").is_empty());
    }

    #[test]
    fn a_fence_carries_over_lines_and_its_content_is_not_read_as_markdown() {
        let got = roles("```rust\n# not a heading\n**x**\n```\n# after");
        assert_eq!(
            got,
            [
                ("```rust", SyntaxRole::Comment),
                ("# not a heading", SyntaxRole::String),
                ("**x**", SyntaxRole::String),
                ("```", SyntaxRole::Comment),
                ("# after", SyntaxRole::Keyword),
            ]
        );
    }

    #[test]
    fn inline_code_emphasis_and_links() {
        let got = roles("a `b*c` **bold** and _it_ [site](http://x.y) end");
        assert_eq!(
            got,
            [
                ("`b*c`", SyntaxRole::String),
                ("**bold**", SyntaxRole::Strong),
                ("_it_", SyntaxRole::Emphasis),
                ("[site]", SyntaxRole::Reference),
                ("(http://x.y)", SyntaxRole::Comment),
            ]
        );
    }

    #[test]
    fn emphasis_and_strong_are_two_roles() {
        let got = roles("*em* and **strong**");
        assert_eq!(
            got,
            [
                ("*em*", SyntaxRole::Emphasis),
                ("**strong**", SyntaxRole::Strong),
            ]
        );
    }

    #[test]
    fn list_markers_and_quotes_are_marked_and_their_text_is_read() {
        let got = roles("- one `c`\n12. two\n> quote **b**");
        assert_eq!(
            got,
            [
                ("-", SyntaxRole::Number),
                ("`c`", SyntaxRole::String),
                ("12.", SyntaxRole::Number),
                (">", SyntaxRole::Comment),
                ("**b**", SyntaxRole::Strong),
            ]
        );
    }

    #[test]
    fn marks_that_do_not_close_or_sit_inside_a_word_are_plain() {
        assert!(roles("snake_case_name and 2 * 3 and `open").is_empty());
        assert_eq!(roles("---"), [("---", SyntaxRole::Comment)]);
    }

    #[test]
    fn offsets_are_bytes_of_the_whole_text() {
        let text = "é\n**é**";
        let spans = markdown_highlights(text, 7);
        assert_eq!(spans.revision(), 7);
        assert_eq!(&text[spans.spans()[0].0.clone()], "**é**");
    }
}
