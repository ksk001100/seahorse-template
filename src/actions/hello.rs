use seahorse::*;

pub fn action(c: &Context) {
    if c.bool_flag("bye") {
        println!("Bye {}", c.args.join(" "));
    } else {
        println!("Hello {}", c.args.join(" "));
    }
}
