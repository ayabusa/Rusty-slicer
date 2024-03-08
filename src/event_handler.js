const { listen } = window.__TAURI__.event

// listen for file location from rust backend
const unlistenfile = await listen('file_path_changed', (event) => {
  document.getElementById("fileLocation").style.color = "white";
  document.getElementById("fileLocation").innerHTML = event.payload.message;
  console.log("changing file label to : ", event.payload.message);
})

// listen for folder location from rust backend
const unlistenfolder = await listen('folder_path_changed', (event) => {
  document.getElementById("folderLocation").style.color = "white";
  document.getElementById("folderLocation").innerHTML = event.payload.message;
  console.log("changing folder label to : ", event.payload.message);
})

// listen for error from rust backend
const unlistenerror = await listen('backend_error', (event) => {
  switch (event.payload.message) {
    case "file_empty":
      document.getElementById("fileLocation").style.color = "red";
      document.getElementById("fileLocation").innerHTML = "please select an input file"
      console.log("please select an input file");
      break;
    case "folder_empty":
      document.getElementById("folderLocation").style.color = "red";
      document.getElementById("folderLocation").innerHTML = "please select an output folder"
      console.log("please select an output folder");
      break;
      case "formating_issue":
        document.getElementById("errorLabel").innerHTML = "error: wrong chapter format. use 0:13 - my title"
        console.log("error: wrong chapter format. use \n0:00 - first song title \n2:13 - second song title \n3:45 - third song title \n...");
        break;
  }
})