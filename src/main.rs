use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;

use clap::Parser;
use memfd::{FileSeal, MemfdOptions};
use rustix::io::FdFlags;

extern crate floaty_egui as _;

static APK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bridge.apk"));

#[derive(Parser)]
struct Arguments {
    #[arg(long)]
    hide_from_screenshots: bool,
}

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();
    let apk = MemfdOptions::default()
        .allow_sealing(true)
        .close_on_exec(false)
        .create("floaty-egui.apk")?;

    apk.as_file().write_all(APK)?;
    apk.add_seals(&[
        FileSeal::SealShrink,
        FileSeal::SealGrow,
        FileSeal::SealWrite,
        FileSeal::SealSeal,
    ])?;

    let executable = File::open("/proc/self/exe")?.close_on_exec(false)?;
    let error = Command::new("/system/bin/app_process")
        .arg(format!(
            "-Djava.class.path=/proc/self/fd/{}",
            apk.as_file().as_raw_fd()
        ))
        .arg("/system/bin")
        .arg("xyz.mufanc.floaty.Main")
        .arg(format!("/proc/self/fd/{}", executable.as_raw_fd()))
        .arg(arguments.hide_from_screenshots.to_string())
        .exec();

    Err(error.into())
}

trait FileExt: Sized {
    fn close_on_exec(self, close: bool) -> anyhow::Result<Self>;
}

impl FileExt for File {
    fn close_on_exec(self, close: bool) -> anyhow::Result<Self> {
        let mut flags = rustix::io::fcntl_getfd(&self)?;
        flags.set(FdFlags::CLOEXEC, close);
        rustix::io::fcntl_setfd(&self, flags)?;
        Ok(self)
    }
}
