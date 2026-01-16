use askama::Template;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};

use crate::{AppState, HtmlTemplate};

#[derive(Template)]
#[template(path = "paste/log.html")]
struct PasteTemplate {
    id: String,
    data: LogData,
}

pub struct LogData {
    pub version: Option<String>,
    pub platform: String,
    pub detections: Vec<Detection>,
    pub content: String,
}

pub struct Detection {
    pub header: String,
    pub detail: Option<String>,
    pub solutions: Option<Solutions>,
    pub line: usize,
}

pub struct Solutions {
    format: SolutionFormat,
    lines: Vec<String>,
}

#[derive(Default)]
pub enum SolutionFormat {
    Or,
    OrderedList,
    #[default]
    Raw,
}

pub async fn paste_handler(Path(id): Path<String>, State(state): State<AppState>) -> Response {
    let format = SolutionFormat::OrderedList;
    let lines = vec![
        "Vypněte server".to_string(),
        "Otevřete Správce Souborů/FTP".to_string(),
        "Smažte složku world".to_string(),
    ];

    let solutions = Solutions {
        format,
        lines: lines.clone(),
    };

    let solutionss = Solutions {
        format: SolutionFormat::default(),
        lines: vec!["V `server.properties` nastavte `level-name`".to_string()],
    };

    let content = tokio::fs::read_to_string(state.configuration.data.directory.join(&id))
        .await
        .unwrap();

    let lines: Vec<&str> = content.lines().collect();

    let parser = mclog::parser::Parser::new(&lines);

    let html = parser.html();

    let bytes: Vec<u8> = html.iter().flat_map(|entry| entry.bytes()).collect();

    let template = PasteTemplate {
        id,
        data: LogData {
            version: Some("1.20.1".to_string()),
            platform: "Paper".to_string(),
            detections: vec![
                Detection {
                    header: "Zjištěn zastaralý plugin".to_string(),
                    detail: None,
                    solutions: Some(solutions),
                    line: 1,
                },
                Detection {
                    header: "Svět nemá jméno".to_string(),
                    detail: None,
                    solutions: Some(solutionss),
                    line: 3,
                },
            ],
            content: String::from_utf8(bytes).unwrap(),
        },
    };

    HtmlTemplate(template).into_response()
}
