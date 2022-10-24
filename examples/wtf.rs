#![allow(unused_variables,dead_code)]
enum Wtf { Alfa, Beta }

fn main() {
    let a = Wtf::Alfa;
    match a {
        Wtf::Alfa => {}
        Wtf::Beta => {}
        // This compiles, just with warnings - why? The warnings are:
        // * warning: unreachable pattern
        // * warning: variable `Nonsense` should have a snake case name
        Nonsense1 => {}
        Nonsense2 => {}
        /*
         Explained by `@bruh![moment]` at https://discord.com/channels/273534239310479360/273541522815713281/1034091312393224232
         The `Nonsense` is considered a catch-all variable and gets assigned the remaining cases.
         Previous discussion with Rust team was here: https://github.com/rust-lang/rust/issues/47811
         */
    }
}
