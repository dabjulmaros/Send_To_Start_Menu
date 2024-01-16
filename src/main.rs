#![windows_subsystem = "windows"]

use mslnk::ShellLink;
use std::{env::args, env::var_os, path::Path, path::PathBuf};

use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

//https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
fn get_appdata() -> Option<PathBuf> {
    var_os("APPDATA").map(PathBuf::from)
}
//https://docs.rs/rodio/latest/rodio/
fn play(path: &Path) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_millis(1500));
}

fn main() {
    let args: Vec<String> = args().collect();
    let target: &str;

    let mut path = get_appdata().expect("no appdata");
    path.push("Microsoft");
    path.push("Windows");

    let file_name: &str;

    if args.len() == 1 {
        target = Path::new(&args[0]).to_str().unwrap();
        path.push("SendTo");

        file_name = "Start Menu";
    } else {
        target = Path::new(&args[1]).to_str().unwrap();

        path.push("Start Menu");
        path.push("Programs");

        let path = Path::new(&args[1]);
        // if file check
        // https://stackoverflow.com/questions/30309100/how-to-check-if-a-given-path-is-a-file-or-directory
        if path.is_file() {
            file_name = path.file_name().unwrap().to_str().unwrap();
        } else {
            play(Path::new(r"C:\Windows\Media\Windows Ding.wav"));
            return;
        }
    }
    path.push(&file_name);
    path.set_extension("lnk");

    let lnk = path;
    let sl = ShellLink::new(target).unwrap();
    sl.create_lnk(lnk).unwrap();
    play(Path::new(r"C:\Windows\Media\Windows Notify.wav"));
}
