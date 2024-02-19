const { listen } = window.__TAURI__.event

document.getElementById("fileLocation").innerHTML = "hey";

// listen for file location from rust backend
const unlistenfile = await listen('file_path_changed', (event) => {
  document.getElementById("fileLocation").innerHTML = event.payload.message;
  console.log("changing file label to : ", event.payload.message);
})

// listen for folder location from rust backend
const unlistenfolder = await listen('folder_path_changed', (event) => {
  document.getElementById("folderLocation").innerHTML = event.payload.message;
  console.log("changing folder label to : ", event.payload.message);
})