use myuchip::{Args, Core, Parser};

fn main() {
    let mut core = Core::new(Args::parse());

    core.run();
}
