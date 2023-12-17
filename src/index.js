const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let is_enter;
let is_drag; 

async function init(){
  const [lantitude, longitude, apitoken] = await invoke("get_info");
  let res = await invoke("get_weather",{lantitude: lantitude, longitude: longitude, apitoken: apitoken});
  document.getElementById("icon").setAttribute("src", res["icon"]);
  document.getElementById("weather").innerHTML = res["weather"];
  document.getElementById("temp").innerHTML = res["temp"];
}

window.addEventListener("DOMContentLoaded", () => {
  document.addEventListener('DOMContentLoaded', init());

  document.getElementById("container").addEventListener("contextmenu", (e) => {
    e.preventDefault();
    window.close();
  });

  document.getElementById("container").addEventListener("mouseenter", (e) => {
    e.preventDefault();
    is_enter = true;
  });

  document.getElementById("container").addEventListener("mouseleave", (e) => {
    e.preventDefault();
    if (is_drag === false){
      is_enter = false;
    }
  });

  window.addEventListener('mousemove', (e) => {
    if (is_enter && is_drag){
      invoke("move_window", {sx: e.screenX, sy: e.screenY});
    }
  });
  window.addEventListener('mousedown', (e) => {
    is_drag = true;
  });
  window.addEventListener('mouseup', (e) => {
    is_drag = false;
  });

  setInterval(() => { init(); }, 60000);
});
