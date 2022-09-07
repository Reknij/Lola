use std::io::Cursor;
use std::sync::Arc;

use base64::encode;
use futures_util::{SinkExt, StreamExt};
use rustls::{ClientConfig, RootCertStore};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

use serde_json::Value as JsonValue;
use tracing::error;

use crate::lcu_driver::LcuDriverConfig;

pub struct WSClient {}

type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnJsonApiEventData {
    pub data: JsonValue,
    pub event_type: String,
    pub uri: String,

    #[serde(skip)]
    pub closed: bool,
}

impl OnJsonApiEventData {
    pub fn closed() -> Self {
        OnJsonApiEventData {
            closed: true,
            uri: String::new(),
            data: JsonValue::Null,
            event_type: String::new(),
        }
    }
}

impl WSClient {
    pub async fn connect(
        config: &LcuDriverConfig,
    ) -> Result<(Self, UnboundedReceiver<OnJsonApiEventData>), Error> {
        let rustls_cert_raw =
            rustls_pemfile::read_one(&mut Cursor::new(include_bytes!("../certs/riotgames.pem")))?;

        let rustls_cert_raw_bytes = match rustls_cert_raw {
            Some(rustls_pemfile::Item::X509Certificate(bytes)) => bytes,
            _ => {
                panic!("Read cert failed!");
            }
        };

        let rustls_cert = rustls::Certificate(rustls_cert_raw_bytes);

        let mut root_store = RootCertStore::empty();
        root_store.add(&rustls_cert).expect("add cert error!");

        let rustls_config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = tokio_tungstenite::Connector::Rustls(Arc::new(rustls_config));
        let mut url =
            format!("wss://localhost:{}", config.remoting_app_port).into_client_request()?;
        {
            let headers = url.headers_mut();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(
                    format!("Basic {}", encode(format!("riot:{}", config.auth_token))).as_str(),
                )?,
            );
        }

        let (ws_stream, _response) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, Some(connector)).await?;

        let (mut write, read) = ws_stream.split();
        write
            .send(Message::text(format!("[5, \"OnJsonApiEvent\"]")))
            .await?;

        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(read.for_each(move |message| {
            let tx = tx.clone();
            async move {
                match message {
                    Ok(data) => {
                        if data.is_close() {
                            tx.send(OnJsonApiEventData::closed()).unwrap();
                        } else if data.is_text() {
                            if let Ok(data) = data.into_text() {
                                if data.len() == 0 {
                                    return;
                                }
                                let text =
                                    data[..data.len() - 1].replace("[8,\"OnJsonApiEvent\",", "");

                                let json: OnJsonApiEventData = serde_json::from_str(&text)
                                    .expect("Convert websocket returned data error!");
                                tx.send(json).unwrap();
                            }
                        }
                    }
                    Err(err) => {
                        error!("{:?}", err);
                        tx.send(OnJsonApiEventData::closed()).unwrap();
                    }
                }
            }
        }));

        Ok((Self {}, rx))
    }
}
