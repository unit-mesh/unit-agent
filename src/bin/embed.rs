use std::path::Path;
use std::sync::Arc;

use ort::{
    tensor::{FromArray, InputTensor, OrtOwnedTensor},
    Environment, ExecutionProvider, GraphOptimizationLevel, LoggingLevel, SessionBuilder,
};

fn main() -> Result<(), anyhow::Error> {
    let model_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("model");

    let environment = Arc::new(
        Environment::builder()
            .with_name("Encode")
            .with_log_level(LoggingLevel::Warning)
            .with_execution_providers([ExecutionProvider::cpu()])
            .build()?,
    );

    let threads = if let Ok(v) = std::env::var("NUM_OMP_THREADS") {
        str::parse(&v).unwrap_or(1)
    } else {
        1
    };

    let tokenizer: Arc<tokenizers::Tokenizer> = tokenizers::Tokenizer::from_file(model_dir.join("tokenizer.json"))
        .unwrap()
        .into();

    let sequence = "Hello, world!";
    let tokenizer_output = tokenizer.encode(sequence, true).unwrap();

    println!("{:?}", tokenizer_output.get_ids());

    let session: Arc<ort::Session> = SessionBuilder::new(&environment)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(threads)?
        .with_model_from_file(model_dir.join("model.onnx"))?
        .into();

    Ok(())
}