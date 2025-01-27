use clap::ValueEnum;
use kalosm::language::LlamaSource;
use std::fmt;

#[derive(Debug, Clone, ValueEnum)]
pub enum Model {
    Llama323bChat,
    Phi35Mini4kInstruct,
    Phi3Mini4kInstruct,
    Phi4,
    Qwen2515bInstruct,
    Qwen253bInstruct,
    TinyLlama11bChat,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model_name = match self {
            Model::Llama323bChat => "Llama-3.2-3B-chat",
            Model::Phi35Mini4kInstruct => "Phi-3.5-mini-4k-instruct",
            Model::Phi3Mini4kInstruct => "Phi-3-mini-4k-instruct",
            Model::Phi4 => "phi-4",
            Model::Qwen2515bInstruct => "Qwen2.5-1.5B-instruct",
            Model::Qwen253bInstruct => "Qwen2.5-3B-instruct",
            Model::TinyLlama11bChat => "Tiny-Llama-1.1b-chat",
        };
        write!(f, "{}", model_name)
    }
}

impl Model {
    pub fn get_llama_source(&self) -> LlamaSource {
        match self {
            Model::Llama323bChat => LlamaSource::llama_3_2_3b_chat(),
            Model::Phi35Mini4kInstruct => LlamaSource::phi_3_5_mini_4k_instruct(),
            Model::Phi3Mini4kInstruct => LlamaSource::phi_3_mini_4k_instruct(),
            Model::Phi4 => LlamaSource::phi_4(),
            Model::Qwen2515bInstruct => LlamaSource::qwen_2_5_1_5b_instruct(),
            Model::Qwen253bInstruct => LlamaSource::qwen_2_5_3b_instruct(),
            Model::TinyLlama11bChat => LlamaSource::tiny_llama_1_1b_chat(),
        }
    }
}
