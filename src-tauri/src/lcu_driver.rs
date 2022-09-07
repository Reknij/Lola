use futures_util::lock::Mutex;
pub use serde_json::Value as JsonValue;
use std::{sync::Arc, time::Duration};
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::Manager;
use tracing::{info, debug, error};

use base64::encode;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient, Url,
};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::ws::{OnJsonApiEventData, WSClient};

#[derive(Debug)]
pub struct LcuDriver {
    pub config: LcuDriverConfig,
    http: HttpClient,
    http_base_url: Url,
    ws_rx: Arc<Mutex<UnboundedReceiver<OnJsonApiEventData>>>,
}

#[derive(Clone, Debug)]
pub struct LcuDriverConfig {
    pub remoting_app_port: u16,
    pub auth_token: String,
}

impl LcuDriver {
    pub async fn connect(config: &LcuDriverConfig, duration: Duration) -> LcuDriver {
        let pem_bytes = include_bytes!("../certs/riotgames.pem");
        let cert_http = reqwest::Certificate::from_pem(pem_bytes)
            .expect("Create certificate from bytes of pem file error!");
        let full_token = encode(format!("riot:{}", &config.auth_token));
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", &full_token))
                .expect("Create AUTHORIZATION Header value error!"),
        );

        let http = reqwest::Client::builder()
            .add_root_certificate(cert_http)
            .default_headers(headers)
            .build()
            .expect("Create http client error!");

        let mut con = WSClient::connect(&config).await;
        while let Err(err) = con {
            error!("{:?}", err);
            tokio::time::sleep(duration).await;
            con = WSClient::connect(&config).await;
        }
        let (_ws, ws_rx) = con.unwrap();

        LcuDriver {
            config: config.clone(),
            http,
            http_base_url: Url::parse(&format!("https://127.0.0.1:{}", config.remoting_app_port))
                .expect("Parse http base url error!"),
            ws_rx: Arc::new(Mutex::new(ws_rx)),
        }
    }

    pub async fn get(&self, sub_url: &str) -> Result<JsonValue, reqwest::Error> {
        let result = self
            .http
            .get(self.http_base_url.join(sub_url).expect("Parse url error!"))
            .send()
            .await?
            .json::<JsonValue>()
            .await;
        match result {
            Ok(r) => Ok(r),
            Err(err) => {
                info!("Json desrialize get error: {}", err);
                Ok(JsonValue::Null)
            }
        }
    }

    pub async fn post(&self, sub_url: &str, body: &JsonValue) -> Result<JsonValue, reqwest::Error> {
        let result = self
            .http
            .post(self.http_base_url.join(sub_url).expect("Parse url error!"))
            .json(&body)
            .send()
            .await?
            .json()
            .await;
        match result {
            Ok(r) => Ok(r),
            Err(err) => {
                info!("Json desrialize get error: {}", err);
                Ok(JsonValue::Null)
            }
        }
    }

    pub async fn put(&self, sub_url: &str, body: &JsonValue) -> Result<JsonValue, reqwest::Error> {
        let result = self
            .http
            .put(self.http_base_url.join(sub_url).expect("Parse url error!"))
            .json(&body)
            .send()
            .await?
            .json()
            .await;
        match result {
            Ok(r) => Ok(r),
            Err(err) => {
                info!("Json desrialize get error: {}", err);
                Ok(JsonValue::Null)
            }
        }
    }

    pub async fn patch(&self, sub_url: &str, body: &JsonValue) -> Result<JsonValue, reqwest::Error> {
        let result = self
            .http
            .patch(self.http_base_url.join(sub_url).expect("Parse url error!"))
            .json(&body)
            .send()
            .await?
            .json()
            .await;
        match result {
            Ok(r) => Ok(r),
            Err(err) => {
                info!("Json desrialize get error: {}", err);
                Ok(JsonValue::Null)
            }
        }
    }

    pub async fn delete(&self, sub_url: &str) -> Result<JsonValue, reqwest::Error> {
        let result = self
            .http
            .delete(self.http_base_url.join(sub_url).expect("Parse url error!"))
            .send()
            .await?
            .json()
            .await;
        match result {
            Ok(r) => Ok(r),
            Err(err) => {
                info!("Json desrialize get error: {}", err);
                Ok(JsonValue::Null)
            }
        }
    }

    pub async fn hook_on_window(&self, handle: tauri::AppHandle) {
        let ws_rx = self.ws_rx.clone();
        tokio::spawn(async move {
            while let Some(json) = ws_rx.lock().await.recv().await {
                if json.closed {
                    info!("Restart because disconnected");
                    handle.restart();
                }
                if json.uri != "" {
                    handle.emit_all("lcu_events", &json).unwrap();
                }
            }
        });
    }
}

impl LcuDriverConfig {
    pub fn from_client() -> Result<Self, String> {
        let s = System::new_all();
        let lolc = s.processes_by_exact_name("LeagueClientUx.exe").next();

        match lolc {
            None=>Err(String::from("Can't found League of Legends client.")),
            Some(lolc) => {
                let command_line = lolc.cmd();
                debug!("{:?}", &command_line);
                if command_line.len() == 0 {
                    return Err(String::from("League of Legends client command line is null."))
                }

                let mut remoting_app_port = 0;
                let mut auth_token = String::new();
                for command in command_line {
                    if command.starts_with("--app-port=") {
                        remoting_app_port = command.replace("--app-port=", "").parse().expect("Parse remoting_app_port failed!");
                    }
                    else if command.starts_with("--remoting-auth-token=") {
                        auth_token = command.replace("--remoting-auth-token=", "");
                    }
                }
                let config = LcuDriverConfig {
                    remoting_app_port,
                    auth_token,
                };
                info!("Lcu driver config: {:?}", &config);
                Ok(config)
            }
        }

       
    }

    // fn get_command_line_str(app_name: &str) -> String {
    //     let output = Command::new("cmd")
    //         .arg("/C")
    //         .arg(format!(
    //             "wmic PROCESS WHERE name='{}' GET commandline",
    //             app_name
    //         ))
    //         .output()
    //         .expect("Failed run wmic to get 'LeagueClientUx.exe' commandline.");

    //     String::from_utf8(output.stdout).expect("Convert bytes to string error.")
    // }
}
