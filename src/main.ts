import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
let isEnter = false;
let isDrag = false; 

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
    isEnter = true;
  });

  document.getElementById("container")?.addEventListener("mouseleave", (e) => {
    e.preventDefault();
    if (isDrag === false){ isEnter = false; }
  });

  document.getElementById("container")?.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    appWindow.close();
  });

  window.addEventListener('mousemove', (e) => {
    if (isEnter && isDrag){
      invoke("move_window", {sx: e.screenX, sy: e.screenY});
    }
  });
  window.addEventListener('mousedown', (e) => {
    if (e.button === 0){ isDrag = true; }
  });
  window.addEventListener('mouseup', (e) => {
    if (e.button === 0) { 
      isDrag = false; 
      invoke("save_window");
    } 
  });
  setInterval(() => { get_weather(); }, 60000);
});
