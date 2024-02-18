const { listen } = window.__TAURI__.event

document.getElementById("fileLocation").innerHTML = "hey";

// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
const unlisten = await listen('my_event', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  document.getElementById("fileLocation").innerHTML = "hello";
})

// emits the `click` event with the object payload
emit('click', {
  theMessage: 'Tauri is awesome!',
})