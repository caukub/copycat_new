use axum::{
    BoxError,
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    response::Redirect,
};
use futures_util::{Stream, TryStreamExt};
use rand::{Rng, distr::Alphanumeric};
use serde::Serialize;
use std::{io, pin::pin};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};
use tokio_util::io::StreamReader;
use tracing::trace;

use crate::AppState;

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum PasteType {
    Log,
    Yaml,
    Json,
    Other,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
struct PasteMetadata {
    #[serde(rename = "type")]
    paste_type: PasteType,
}

#[tracing::instrument(skip(multipart, state))]
pub async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    let id = generate_id();

    let _ = get_paste_type(&state).await;

    // save max 5 MB, check if json/yaml only if < 1.5 MB

    let mut paste_type = PasteType::Other;

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(name) = field.name()
            && name == "content"
        {
            stream_to_file(&id, field).await?;
            trace!("Created paste '{}'", id);
        }
    }

    let paste_content = tokio::fs::read_to_string(state.configuration.data.directory.join(&id))
        .await
        .unwrap();
    if paste_content.clone().contains("INFO]") {
        paste_type = PasteType::Log;
    }

    // x needs to be first because every x is valid y
    if serde_json::from_str::<serde_json::Value>(&paste_content).is_ok() {
        paste_type = PasteType::Json;
    } else if serde_yaml::from_str::<serde_yaml::Value>(&paste_content).is_ok() {
        paste_type = PasteType::Yaml;
    }

    let metadata = PasteMetadata { paste_type };

    let json_metadata = serde_json::to_string(&metadata).unwrap();

    let mut file = tokio::fs::File::create(
        state
            .configuration
            .data
            .directory
            .join(".metadata")
            .join(&id),
    )
    .await
    .unwrap();
    file.write_all(json_metadata.as_bytes()).await.unwrap();

    Ok(Redirect::to(&id))
}

// check velikosti
// prvni musi byt json pak yaml
// mozna se na tohle uplne vykaslat kdyz se pouziva magika
// tokio spawn? asi ne omezime na 1 MB
async fn get_paste_type(state: &AppState) -> std::io::Result<()> {
    #[cfg(feature = "magika")]
    if let Some(ref magika_mutex) = state.magika {
        let mut session = magika_mutex
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Mutex poisoning"))?;

        let result = session
            .identify_content_sync(&b"#!/bin/sh\necho hello"[..])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        error!("{}", result.info().label);
    }
    Ok(())
}

async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    const UPLOADS_DIRECTORY: &str = "data";

    async {
        let body_with_io_error = stream.map_err(io::Error::other);
        let mut body_reader = pin!(StreamReader::new(body_with_io_error).take(2_000_000));

        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        tokio::io::copy(&mut body_reader, &mut file).await?;

        file.flush().await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

fn generate_id() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
