extern crate lxc;

fn main() {
    println!("LXC version: {}\n", ::lxc::version());

    println!("Wait states:");
    for state in ::lxc::wait_states() {
        println!("- {}", state);
    }
}
