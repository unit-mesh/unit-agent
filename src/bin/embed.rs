use std::path::Path;
use std::sync::Arc;
use ndarray::Axis;

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

    // Origin: Hello, world!
    // ids: 101, 7592, 1010, 2088, 999, 102
    // map: [CLS] hello , world ! [SEP]
    println!("{:?}", tokenizer_output.get_ids());

    let session: Arc<ort::Session> = SessionBuilder::new(&environment)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(threads)?
        .with_model_from_file(model_dir.join("model.onnx"))?
        .into();

    let input_ids = tokenizer_output.get_ids();
    let attention_mask = tokenizer_output.get_attention_mask();
    let token_type_ids = tokenizer_output.get_type_ids();
    let length = input_ids.len();

    let inputs_ids_array = ndarray::Array::from_shape_vec(
        (1, length),
        input_ids.iter().map(|&x| x as i64).collect(),
    )?;

    let attention_mask_array = ndarray::Array::from_shape_vec(
        (1, length),
        attention_mask.iter().map(|&x| x as i64).collect(),
    )?;

    let token_type_ids_array = ndarray::Array::from_shape_vec(
        (1, length),
        token_type_ids.iter().map(|&x| x as i64).collect(),
    )?;

    let outputs = session.run([
        InputTensor::from_array(inputs_ids_array.into_dyn()),
        InputTensor::from_array(attention_mask_array.into_dyn()),
        InputTensor::from_array(token_type_ids_array.into_dyn()),
    ])?;

    let output_tensor: OrtOwnedTensor<f32, _> = outputs[0].try_extract().unwrap();
    let sequence_embedding = &*output_tensor.view();
    let pooled = sequence_embedding.mean_axis(Axis(1)).unwrap();

    println!("{:?}", pooled.to_owned().as_slice().unwrap().to_vec());

    Ok(())
}
