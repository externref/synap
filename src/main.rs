mod configs;
mod info;

fn main() {
    let configs = configs::InfoConfigs::default();
    let info = info::SystemInfo::create();

    println!("{:?}", configs);
}
