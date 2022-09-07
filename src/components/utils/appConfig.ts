import { invoke } from "@tauri-apps/api";
import { AppConfig } from "../models/Backend/AppConfig";

export async function saveAppConfig() {
    await invoke("save_app_config")
}

export async function setAppConfig(newConfig: AppConfig) {
    await invoke("set_app_config", {
        newConfig
    })
}


export async function getAppConfig(): Promise<AppConfig> {
    return await invoke<AppConfig>("get_app_config");
}