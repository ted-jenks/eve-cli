use std::error::Error;

use crate::{client::model::Model, error::error::EveError};
use curl::easy::Easy;
use serde_json::Value;

pub(crate) struct Client {
    api_key: String,
    model: Model,
}

impl Client {
    pub(crate) fn new(api_key: &str, model: Model) -> Client {
        Client {
            api_key: api_key.to_string(),
            model,
        }
    }

    pub(crate) fn get_response(
        &self,
        prompt: &str,
        temperature: f32,
    ) -> Result<String, Box<dyn Error>> {
        let mut easy = Easy::new();
        let url = "https://api.openai.com/v1/completions";
        let payload = self.get_payload(prompt, temperature);

        easy.url(url)?;
        easy.post(true)?;
        easy.post_field_size(payload.len() as u64)?;
        easy.http_headers(self.get_headers())?;
        easy.post_fields_copy(payload.as_bytes())?;

        let mut response_data = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }

        let response_string: String = String::from_utf8(response_data).unwrap();
        let response_value = Client::retrieve_text_from_json(response_string.as_str())?;
        Ok(response_value)
    }

    fn get_headers(&self) -> curl::easy::List {
        let mut headers = curl::easy::List::new();
        headers
            .append(("Authorization: Bearer ".to_owned() + &self.api_key).as_str())
            .unwrap();
        headers.append("Content-Type: application/json").unwrap();
        headers
    }

    fn get_payload(&self, prompt: &str, temperature: f32) -> String {
        serde_json::json!({
            "model": self.model.to_string(),
            "prompt": prompt,
            "max_tokens": 400,
            "temperature": temperature,
            "stop": "***"
        })
        .to_string()
    }

    fn retrieve_text_from_json(json_string: &str) -> Result<String, EveError> {
        let parsed: Value = serde_json::from_str(json_string)
            .map_err(|e| EveError::new(format!("{}", e).as_str()))?;
        let choices = parsed
            .get("choices")
            .ok_or(EveError::new("Field 'choices' not found"))?
            .as_array()
            .ok_or(EveError::new("Field 'choices' is not an array"))?;
        let choice_text = choices[0]
            .get("text")
            .ok_or(EveError::new("Field 'text' not found in choices"))?
            .as_str()
            .ok_or(EveError::new("Field 'text' is not a valid string"))?
            .to_string();
        Ok(choice_text)
    }
}
