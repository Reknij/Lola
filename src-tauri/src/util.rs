use std::path::Path;

use tokio::{fs::File, io::AsyncWriteExt};

pub async fn write_to_file(path: &Path, data: &str) -> Result<(), String> {
    let file = File::create(path).await;

    match file {
        Ok(mut file) => {
            file.write_all(data.as_bytes())
                .await
                .map_err(|err| err.to_string())?;
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}