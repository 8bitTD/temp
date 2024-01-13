import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
let is_enter = false;
let is_drag = false; 

async function get_weather(){
  const [lantitude, longitude, apitoken] = await invoke("get_info") as any;
  let res = await invoke("get_weather",{lantitude: lantitude, longitude: longitude, apitoken: apitoken}) as any;
  if (res.temp === "-- ? --"){document.location.assign("setting.html");}
  let weather: string = (res["weather"]);
  let temp: string = (res["temp"]);
  document.getElementById("icon")?.setAttribute("src", res["icon"]);
  (document.getElementById("weather") as HTMLLabelElement).innerHTML = weather;
  (document.getElementById("temp") as HTMLLabelElement).innerHTML = temp;
}

window.onload = function(){get_weather()};
window.addEventListener("DOMContentLoaded", () => {
  document.getElementById("container")?.addEventListener("mouseenter", (e) => {
    e.preventDefault();
    is_enter = true;
  });

  document.getElementById("container")?.addEventListener("mouseleave", (e) => {
    e.preventDefault();
    if (is_drag === false){ is_enter = false; }
  });

  document.getElementById("container")?.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    appWindow.close();
  });

  window.addEventListener('mousemove', (e) => {
    if (is_enter && is_drag){
      invoke("move_window", {sx: e.screenX, sy: e.screenY});
    }
  });
  window.addEventListener('mousedown', (e) => {
    if (e.button === 0){ is_drag = true; }
  });
  window.addEventListener('mouseup', (e) => {
    if (e.button === 0) { 
      is_drag = false; 
      invoke("save_window");
    } 
  });
  setInterval(() => { get_weather(); }, 60000);
});
