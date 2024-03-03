const { invoke } = window.__TAURI__.tauri;

invoke("debug_call", { message: 'JS init' });

function select_file_button_pressed(){
    invoke("select_file_button")
}

function select_folder_button_pressed(){
    invoke("select_folder_button")
}

function slice_button_pressed(){
    invoke("slice_button", {chapter: document.getElementById("chapterList").value, fileformat: document.getElementById("fileFormatSelect").value})
}

function about_button_pressed(){
    invoke("about_button")
}

document.getElementById("folderButton").addEventListener("click", select_folder_button_pressed);
document.getElementById("fileButton").addEventListener("click", select_file_button_pressed);
document.getElementById("sliceButton").addEventListener("click", slice_button_pressed);
document.getElementById("aboutButton").addEventListener("click", about_button_pressed);