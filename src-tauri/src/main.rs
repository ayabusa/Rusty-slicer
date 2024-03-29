// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use native_dialog::FileDialog;
use tauri::{Manager, PhysicalSize, Size};
use std::{env, io::{Error, ErrorKind}, path::PathBuf, sync::Mutex};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[macro_use]
extern crate lazy_static;

// define file and folder path variable, don't know if it's the right way of doing it
lazy_static! {
    static ref FILE_PATH: Mutex<String> = Mutex::new("".to_string());
    static ref FOLDER_PATH: Mutex<String> = Mutex::new("".to_string());
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn select_file_button(app: tauri::AppHandle) {
    FILE_PATH.lock().unwrap().replace_range(.., &choose_file());
    println!("{}",FILE_PATH.lock().unwrap());
    let _ = app.emit_all("file_path_changed", Payload { message: FILE_PATH.lock().unwrap().to_string() });
}

#[tauri::command]
async fn select_folder_button(app: tauri::AppHandle) {
    FOLDER_PATH.lock().unwrap().replace_range(.., &choose_folder());
    println!("{}",FOLDER_PATH.lock().unwrap());
    let _ = app.emit_all("folder_path_changed", Payload { message: FOLDER_PATH.lock().unwrap().to_string() });
}

#[tauri::command]
async fn slice_button(app: tauri::AppHandle, chapter: String, fileformat: String){
    // Check the user selected a file and a folder
    let mut cancel_slicing = false;
    if FILE_PATH.lock().unwrap().to_owned() == "" || FILE_PATH.lock().unwrap().to_owned() == "None"{
        println!("error no file selected, slicing aborted");
        app.emit_all("backend_error", Payload { message: "file_empty".to_owned() }).unwrap();
        cancel_slicing = true;
    }
    if FOLDER_PATH.lock().unwrap().to_owned() == "" || FOLDER_PATH.lock().unwrap().to_owned() == "None"{
        println!("error no folder selected, slicing aborted");
        app.emit_all("backend_error", Payload { message: "folder_empty".to_owned() }).unwrap();
        cancel_slicing = true;
    }
    if cancel_slicing == true { return; }

    // Try to format the chapters and panic if it was not able to
    let formated_chapters = match format_chapter(&chapter) {
        Ok(res) => res,
        Err(error) => {
            println!("error formating chapters, slicing aborted");
            app.emit_all("formatting_error", Payload { message: error.to_string() }).unwrap();
            return;
        },
    };

    let time_codes: Vec<String> = formated_chapters.0;
    let title_names: Vec<String> = formated_chapters.1;

    println!("time codes: \n{:?}\ntitle names: \n{:?}", time_codes, title_names);

    // create folder if it does not exist
    /*match fs::create_dir_all(FOLDER_PATH.lock().unwrap().to_owned()) {
        Ok(res) => res,
        Err(error) => panic!("Problem creating directory : {:?}", error),
    };*/

    // create the progress window
    let _progress_window = tauri::WindowBuilder::new(
        &app,
        "progress", /* the unique window label */
        tauri::WindowUrl::App("progress.html".into())
        ).build().expect("failed to create progress window");
        _progress_window.set_title("Slicing progress").unwrap();
        _progress_window.set_size(Size::Physical(PhysicalSize { width: 400, height: 100 })).unwrap();

    for i in 0..time_codes.len(){
        let args: Vec<String>;
        // create the path to the output file
        let mut output_file: PathBuf = PathBuf::from(&FOLDER_PATH.lock().unwrap().to_owned());
        output_file.push(format!("{:02} - {}", i+1, title_names[i]));
        output_file.set_extension(&fileformat);

        if i+1<time_codes.len() {
            args = vec![
                "-y".to_owned(), // overwrite existing files
                "-i".to_owned(), 
                FILE_PATH.lock().unwrap().to_owned(),
                "-ss".to_owned(),
                time_codes[i].to_owned(),
                "-to".to_owned(),
                time_codes[i+1].to_owned(),
                "-vn".to_owned(), // no video
                //format!("{:?}", output_file),
                output_file.display().to_string()];
        }else { // case for the last song
            args = vec![
                "-y".to_owned(), // overwrite existing files
                "-i".to_owned(), 
                FILE_PATH.lock().unwrap().to_owned(),
                "-ss".to_owned(),
                time_codes[i].to_owned(),
                "-vn".to_owned(), // no video
                //format!("{:?}", output_file),
                output_file.display().to_string()];
        }

        // launch the final ffmpeg command
        launch_ffmpeg(app.clone(), args);

        // update progress bar on frontend
        app.emit_all("progress_state_changed", Payload { message: format!("{}", (i+1)*100/time_codes.len()) }).unwrap();
    }
    _progress_window.close().unwrap(); // close the progress window, before creating the done one

    let _done_window = tauri::WindowBuilder::new(
        &app,
        "done", /* the unique window label */
        tauri::WindowUrl::App("done.html".into())
        ).build().expect("failed to create done window");
        _done_window.set_title("Rusty Slicer").unwrap();
        _done_window.set_size(Size::Physical(PhysicalSize { width: 400, height: 100 })).unwrap();
}

#[tauri::command]
async fn about_button(handle: tauri::AppHandle) {
  let _about_window = tauri::WindowBuilder::new(
    &handle,
    "about", /* the unique window label */
    tauri::WindowUrl::App("about.html".into())
    ).build().expect("failed to create about window");
    _about_window.set_title("About Rusty Slicer").unwrap();
    _about_window.set_size(Size::Physical(PhysicalSize { width: 400, height: 600 })).unwrap();
}

#[tauri::command]
fn debug_call(message: &str){
    println!("[DBG] {}", message);
}

/// Separate time codes from title and return it to a tuple of vector of string
/// # Example
/// ```
/// let formated_chapters = match format_chapter(chapter) {
///    Ok(res) => res,
///     Err(error) => panic!("Problem slicing chapter: {:?}", error),
/// };
/// let time_codes: Vec<String> = formated_chapters.0;
/// let title_names: Vec<String> = formated_chapters.1;
/// ```
fn format_chapter(chapter: &str) -> Result<(Vec<String>, Vec<String>), Error>{
    let lines: Vec<&str> = chapter.split("\n").collect();
    let mut time_code: Vec<String> = vec![];
    let mut title_names: Vec<String> = vec![];
    if lines.len() < 2 { return Err(Error::new(ErrorKind::Other, "Lines are empty")); }

    for l in lines.iter(){
        if l.is_empty() { break; }
        if l.len() > 200 { return Err(Error::new(ErrorKind::Other, "Line is too big")); }
        let splited_line = l.split(" - ").collect::<Vec<&str>>();
        if splited_line.len()<2 || splited_line[1] == "" { // To avoid blank title
            return Err(Error::new(ErrorKind::Other, "No title associated with the time code")); 
        }
        if !is_time_code_valid(&splited_line[0].to_owned()){
            return Err(Error::new(ErrorKind::Other, "one or multiple time code is invalid"));
        }
        time_code.push(splited_line[0].to_owned());
        title_names.push(splited_line[1..].join(" - "));
    }
    Ok((time_code, title_names))
}

/// return true if the time code is valid, and false otherwise
fn is_time_code_valid(time_code: &String) -> bool{
    let allowed_char = "0123456789:";
    for c in time_code.chars(){
        if !allowed_char.contains(c) { return false; }
    }
    true
}

// prompt user file chooser using native_dialogue crate
fn choose_file() -> String{
    println!("Let's choose a file !");
    let path = FileDialog::new()
        .show_open_single_file()
        .unwrap();
    format!("{:?}", path).replace("Some(\"", "").replace("\")", "") // turn the FileDialog into a string and remove Some("")
}

fn choose_folder() -> String{
    println!("Let's choose a folder !");
    let path = FileDialog::new()
        .show_open_single_dir()
        .unwrap();
    format!("{:?}", path).replace("Some(\"", "").replace("\")", "") // turn the FileDialog into a string
}

fn launch_ffmpeg(app: tauri::AppHandle, args: Vec<String>) {
    let bin_name = if cfg!(target_os = "macos") {
        PathBuf::from("ffmpeg-macos")
      } else if cfg!(windows) {
        PathBuf::from("ffmpeg-windows.exe")
      } else if cfg!(unix) {
        PathBuf::from("ffmpeg-linux")
      } else {
        panic!("[RUSTY SLICER] can't find what os it is !")
      };
    let mut bin_path = PathBuf::from("resources");
    bin_path.push(bin_name);

    // get the path from the bundled binary
    let resource_path = app.path_resolver()
      .resolve_resource(bin_path)
      .expect("failed to resolve resource");

    println!("using ffmpeg binary : {}\nwith the following argument : {:?}", resource_path.display(), args);
    // launch the command
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new(resource_path.as_os_str())
                     .args(args)
                     .creation_flags(0x08000000) // avoid the terminal from showing up
                     .output()
                     .expect("failed to execute process");
    #[cfg(not (target_os = "windows"))]
    let output = std::process::Command::new(resource_path.as_os_str())
                     .args(args)
                     .output()
                     .expect("failed to execute process");

    // print the output of the ffmpeg command
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn main() {
    env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    // generate the tauri app
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_file_button, select_folder_button, debug_call, slice_button, about_button])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
