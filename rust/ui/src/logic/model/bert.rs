use std::error::Error;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use gloo_console::log;
use tokenizers::{Encoding, PaddingParams, Tokenizer};
use wasm_bindgen::JsError;

pub struct Model {
    bert: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl Model {
    pub fn load(weights: Vec<u8>, tokenizer: Vec<u8>, config: Vec<u8>) -> Result<Model, JsError> {
        log!("loading model");
        let device = Device::Cpu;
        let vb = VarBuilder::from_buffered_safetensors(weights, DType::F64, &device)?;
        let config: Config = serde_json::from_slice(&config)?;
        let tokenizer =
            Tokenizer::from_bytes(&tokenizer).map_err(|m| JsError::new(&m.to_string()))?;
        let bert = BertModel::load(vb, &config)?;

        Ok(Self {
            bert,
            tokenizer,
            device,
        })
    }

    fn tokenize(&mut self, sentences: Vec<String>) -> Result<Vec<Encoding>, JsError> {
        if let Some(pp) = self.tokenizer.get_padding_mut() {
            pp.strategy = tokenizers::PaddingStrategy::BatchLongest
        } else {
            let pp = PaddingParams {
                strategy: tokenizers::PaddingStrategy::BatchLongest,
                ..Default::default()
            };
            self.tokenizer.with_padding(Some(pp));
        }

        let tokens = self
            .tokenizer
            .encode_batch(sentences.to_vec(), true)
            .map_err(|m| JsError::new(&m.to_string()))?;

        Ok(tokens)
    }

    fn collate(&mut self, tokens: Vec<Encoding>) -> Result<(Tensor, Tensor, Tensor), JsError> {
        let (token_ids, mask) = tokens
            .iter()
            .map(|tokens| {
                let mask = tokens.get_attention_mask();
                let tokens = tokens.get_ids();
                Ok((
                    Tensor::new(tokens, &self.device)?,
                    Tensor::new(mask, &self.device)?,
                ))
            })
            .collect::<Result<(Vec<Tensor>, Vec<Tensor>), _>>()
            .map_err(|e: candle_core::Error| JsError::new(&e.to_string()))?;

        let token_ids = Tensor::stack(&token_ids, 0)?;
        let mask = Tensor::stack(&mask, 0)?;
        let token_type_ids = token_ids.zeros_like()?;
        Ok((token_ids, mask, token_type_ids))
    }

    pub fn embed(&mut self, input: Params) -> Result<Tensor, JsError> {
        let sentences = input.sentences;
        let normalize_embeddings = input.normalize_embeddings;

        let tokens = self.tokenize(sentences)?;
        let (input_ids, tokens_type_ids, attention_mask) = self.collate(tokens)?;

        let embeddings = self
            .bert
            .forward(&input_ids, &tokens_type_ids, Some(&attention_mask))?;

        // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
        let embeddings = if normalize_embeddings {
            embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?
        } else {
            embeddings
        };
        Ok(embeddings)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Params {
    sentences: Vec<String>,
    normalize_embeddings: bool,
}
