use launcher_core::internet::fetch_internet_servers;

fn main() {
    for server in fetch_internet_servers() {
        println!("{:?}", server);
    }
}
