use flw::*;

fn main() {
    println!("Hello, world!");
    let app = Arguments::run();
    println!("{:?}", app);
    app.run_command();
}
