use axum::{BoxError, body::Bytes, extract::Multipart, http::StatusCode, response::Redirect};
use futures_util::{Stream, TryStreamExt};
use rand::{Rng, distr::Alphanumeric};
use std::{io, pin::pin};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};
use tokio_util::io::StreamReader;
use tracing::trace;

#[tracing::instrument(skip(multipart))]
pub async fn upload_handler(mut multipart: Multipart) -> Result<Redirect, (StatusCode, String)> {
    let id = generate_id();

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(name) = field.name()
            && name == "content"
        {
            stream_to_file(&id, field).await?;
            trace!("Created paste '{}'", id);
            return Ok(Redirect::to(&id));
        }
    }
    Err((
        StatusCode::BAD_REQUEST,
        "Field 'content' is missing".to_string(),
    ))    
}

async fn get_paste_type() -> std::io::Result<()> {
    let mut file = tokio::fs::File::open("path").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    // check velikosti
    // prvni musi byt json pak yaml
    // mozna se na tohle uplne vykaslat kdyz se pouziva magika
    // tokio spawn? asi ne omezime na 1 MB
    if serde_json::from_slice::<serde_json::Value>(&buffer).is_ok() {

    } else if serde_yaml::from_slice::<serde_yaml::Value>(&buffer).is_ok() {

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
