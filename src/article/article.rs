use clap::{Parser, Subcommand};
use kalosm::language::*;
use serde::Deserialize;

#[derive(Clone, Debug, Parse, Schema, Deserialize)]
pub enum ArticleType {
    ArretePrefectoral,
    ArreteMinisteriel,
    Arrete,
}

impl ToString for ArticleType {
    fn to_string(&self) -> String {
        match self {
            ArticleType::ArretePrefectoral => "Arrêté Préfectoral".to_string(),
            ArticleType::ArreteMinisteriel => "Arrêté Ministériel".to_string(),
            ArticleType::Arrete => "Arrêté".to_string(),
        }
    }
}

#[derive(Parse, Clone, Debug, Schema, Deserialize)]
pub struct Article {

    #[parse(pattern = r"[a-zA-Z,.?!\d ]+")]
    pub reasoning: String,

    pub article_type: ArticleType,

    #[parse(pattern = r"[a-zA-Z,.?!\d ]+")]
    pub content: String,

    // An optional article number, which can include alphanumeric characters and dashes (e.g., "123-ABC").
    #[parse(pattern = r"[a-zA-Z\d-]+")]
    pub article_number: String,

    // An optional date field in UTC format (e.g., "2023-12-31").
    #[parse(pattern = r"\d{4}-\d{2}-\d{2}")]
    pub date: String,
}

impl ToString for Article {
    fn to_string(&self) -> String {
        format!(
            "Reasoning: {}\nArticle Type: {}\nContent: {}\nArticle Number: {}\nDate: {}",
            self.reasoning,
            self.article_type.to_string(),
            self.content,
            self.article_number,
            self.date
        )
    }
}
