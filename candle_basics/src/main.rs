#![allow(unused)]
use hf_hub::api::sync::Api;
use candle_core::{Device, Tensor, DType};
use candle_nn::{Linear, Module};


fn main() {
    let api = Api::new().unwrap();
    let repo = api.model("bert-base-uncased".to_string());

    let weights: std::path::PathBuf = repo.get("model.safetensors").unwrap();

    let weights = candle_core::safetensors::load(weights, &Device::Cpu).unwrap();
    let weight = weights.get("bert.encoder.layer.0.attention.self.query.weight").unwrap();
    let bias = weights.get("bert.encoder.layer.0.attention.self.query.bias").unwrap();

    let linear = Linear::new(weight.clone(), Some(bias.clone()));

    let input_ids = Tensor::randn(0f32, 1.0, (3, 768), &Device::Cpu).unwrap();
    let output = linear.forward(&input_ids).unwrap().argmax(1);

    println!("{output:?}");
}