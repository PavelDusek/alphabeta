use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arg {
    text: String,
}

fn transcribe(input: &str, next: Option<&str>) -> (char, bool) {
    /*
     * Looks at the input char and the next char.
     * If it is the digraph, then returns correspoding greek letter
     * and true value that the next char should be skipped.
     * If it isn't digraph, then returns corresponding greek letter
     * and false value, so that the next char should not be skipped.
     */
    match (input, next) {
        //digraphs
        ("t", Some("h")) => ('θ', true), // \u{3b8}
        ("p", Some("h")) => ('φ', true), // \u{3c6}
        ("c", Some("h")) => ('χ', true), // \u{3c7}
        ("p", Some("s")) => ('ψ', true), // \u{3c8}
        ("k", Some("s")) => ('ξ', true), // \u{3be}
        ("s", Some(" " | "." | "," | ";" | "!" | "?")) | ("s", None) => ('ς', false), // \u{3c2}

        //capital digraphs
        ("T", Some("h")) | ("T", Some("H")) => ('Θ', true), // \u{398}
        ("P", Some("h")) | ("P", Some("H")) => ('Φ', true), // \u{3a6}
        ("C", Some("h")) | ("C", Some("H")) => ('Χ', true), // \u{3a7}
        ("P", Some("s")) | ("P", Some("S")) => ('Ψ', true), // \u{3a8}
        ("K", Some("s")) | ("K", Some("S")) => ('Ξ', true), // \u{39e}

        //small letters
        ("a", _) => ('α', false), // \u{3b1}
        ("b", _) => ('β', false), // \u{3b2}
        ("g", _) => ('γ', false), // \u{3b3}
        ("d", _) => ('δ', false), // \u{3b4}
        ("e", _) => ('ε', false), // \u{3b5}
        ("z", _) => ('ζ', false), // \u{3b6}
        ("é", _) => ('η', false), // \u{3b7}
        // θ \u{3b8} is a digraph
        ("i", _) => ('ι', false), // \u{3b9}
        ("k", _) => ('κ', false), // \u{3ba}
        ("l", _) => ('λ', false), // \u{3bb}
        ("m", _) => ('μ', false), // \u{3bc}
        ("n", _) => ('ν', false), // \u{3bd}
        ("x", _) => ('ξ', false), // \u{3be}
        ("o", _) => ('ο', false), // \u{3bf}
        ("p", _) => ('π', false), // \u{3c0}
        ("r", _) => ('ρ', false), // \u{3c1}
        // ς \u{3c2} is considered digraph here (to check that it is at the end of a word).
        ("s", _) => ('σ', false), // \u{3c3}
        ("t", _) => ('τ', false), // \u{3c4}
        ("y", _) => ('υ', false), // \u{3c5}
        ("f", _) => ('φ', false), // \u{3c6}
        // χ \u{3c7} is a digraph
        // ψ \u{3c8} is a digraph
        ("ó", _) => ('ω', false), // \u{3c9}

        //capital letters
        ("A", _) => ('Α', false), // \u{391}
        ("B", _) => ('Β', false), // \u{392}
        ("G", _) => ('Γ', false), // \u{393}
        ("D", _) => ('Δ', false), // \u{394}
        ("E", _) => ('Ε', false), // \u{395}
        ("Z", _) => ('Ζ', false), // \u{396}
        ("É", _) => ('Η', false), // \u{397}
        // Θ \u{398} is a digraph
        ("I", _) => ('Ι', false), // \u{399}
        ("K", _) => ('Κ', false), // \u{39a}
        ("L", _) => ('Λ', false), // \u{39b}
        ("M", _) => ('Μ', false), // \u{39c}
        ("N", _) => ('Ν', false), // \u{39d}
        // Ξ \u{39e} is a digraph
        ("O", _) => ('Ο', false), // \u{39f}
        ("P", _) => ('Π', false), // \u{3a0}
        ("R", _) => ('Ρ', false), // \u{3a1}
        ("S", _) => ('Σ', false), // \u{3a3}
        ("T", _) => ('Τ', false), // \u{3a4}
        ("Y", _) => ('Υ', false), // \u{3a5}
        ("F", _) => ('Φ', false), // \u{3a6}
        ("X", _) => ('Χ', false), // \u{3a7}
        // Ψ \u{3a8} is a digraph
        ("Ó", _) => ('Ω', false), // \u{3c9}

        // TODO dasia, varia, oxia, perispomeni, prosgegrammeni, psili
        (_, _) => (input.chars().next().unwrap(), false),
    }
}

fn get_char(text: &str, index: usize) -> (&str, usize) {
    let mut end = index + 1;
    while end < text.len() && !text.is_char_boundary(end) {
        //search for next char boundary, it must be lesser than end of slice
        end += 1;
    }
    (&text[index..end], end)
}

fn main() {
    let arg = Arg::parse();
    let text = arg.text;
    let mut greek: char;
    let mut latin;
    let mut latin_next;
    let mut index: usize = 0;
    let mut next_index: usize;
    let mut skip: bool;

    while index < text.len() {
        if !text.is_char_boundary(index) {
            index += 1;
            continue;
        }
        if index + 1 == text.len() {
            //the ultimate character, can't look for next one
            (latin, _) = get_char(text.as_str(), index);
            (greek, skip) = transcribe(latin, None);
        } else {
            //can use this and the next character
            (latin, next_index) = get_char(text.as_str(), index);
            (latin_next, _) = get_char(text.as_str(), next_index);
            (greek, skip) = transcribe(latin, Some(latin_next));
        }
        print!("{greek}");
        // this char was digraph, so the next should be skipped
        if skip {
            index += 1;
        }
        index += 1;
    }
    println!("\n");
}
