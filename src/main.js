const { invoke } = window.__TAURI__.tauri;


invoke("debug_call", { message: 'JS init' });

function select_file_button_pressed(){
    let t = invoke("select_file_button");
}

function select_folder_button_pressed(){
    invoke("select_folder_button")
}


document.getElementById("folderButton").addEventListener("click", select_folder_button_pressed);
document.getElementById("fileButton").addEventListener("click", select_file_button_pressed);

