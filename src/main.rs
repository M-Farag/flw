use flw::*;

fn main() {
    println!("Hello, world!");
    let app = Arguments::get();
    println!("{:?}", app);
    app.run();
}
