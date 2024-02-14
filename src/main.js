const { invoke } = window.__TAURI__.tauri;

function myFunction() {
  document.getElementById("demo").innerHTML = "Hello World";
  invoke("slice")
}
