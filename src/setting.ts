import { invoke } from "@tauri-apps/api/tauri";
import  { appWindow, LogicalSize }from "@tauri-apps/api/window";

window.onload = async function(){
    const [lantitude, longitude, apitoken] = await invoke("get_info") as any;
    (document.getElementById("lantitude") as HTMLInputElement).value = lantitude;
    (document.getElementById("longitude") as HTMLInputElement).value = longitude;
    (document.getElementById("apitoken") as HTMLInputElement).value = apitoken;
    await appWindow.setSize(new LogicalSize(350, 200));
    appWindow.center();
};
window.addEventListener("DOMContentLoaded", () => {
  document.getElementById("setting_btn_exec")?.addEventListener("click",(e) => {
    e.preventDefault;
    let lantitude = Number((document.getElementById("lantitude") as HTMLInputElement).value);
    let longitude = Number((document.getElementById("longitude") as HTMLInputElement).value);
    let apitoken = (document.getElementById("apitoken") as HTMLInputElement).value;
    invoke("set_data", {lantitude: lantitude, longitude: longitude, apitoken: apitoken});
    document.location.assign("index.html");
  });
  
  document.getElementById("setting_btn_close")?.addEventListener("click",(e) => {
    e.preventDefault;
    appWindow.close();
  });
});
