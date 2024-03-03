const { listen } = window.__TAURI__.event

// listen for progress update from rust backend
const unlistenprogress = await listen('progress_state_changed', (event) => {
  document.getElementById("progress_label").innerHTML = "Slicing : "+event.payload.message+"%";
  document.getElementById("progress_bar").value = event.payload.message;
  console.log("changing progress bar state to : ", event.payload.message);
})