use std::env;
use std::process;
use std::process::Command;

fn main() {
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive");
    qemu.arg(format!(
        "format=raw,file={}",
        env::var("BIOS_IMAGE").unwrap()
    ));
    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}
