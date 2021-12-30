use byecache::cmd::Command;

fn main() {
    println!(" -- BYECACHE --");
    println!("This tool can sweep away your cache.");
    println!("You would need `root` privilege to make this work properly.");
    sudo::escalate_if_needed().expect("need root to run.");
    Command::Rm.execute();
}
