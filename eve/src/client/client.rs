use crate::client::model::Model;
use curl::easy::Easy;

struct Client {
    api_key: String,
    model: Model,
}

impl Client {
    fn new(api_key: &str, model: Model) -> Client {
        Client {
            api_key: api_key.to_string(),
            model,
        }
    }

    pub fn get_response(&self, prompt: &str) -> Result<String, curl::Error> {
        let mut easy = Easy::new();
        let url = "https://api.openai.com/v1/chat/completions";
        let payload = self.get_payload(prompt);

        easy.url(url)?;
        easy.post(true)?;
        easy.post_field_size(payload.len() as u64)?;
        easy.http_headers(self.get_headers())?;

        let mut response_data = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }

        let response_string = String::from_utf8(response_data).unwrap();
        Ok(response_string)
    }

    fn get_headers(&self) -> curl::easy::List {
        let mut headers = curl::easy::List::new();
        headers
            .append(("Authorization: Bearer ".to_owned() + &self.api_key).as_str())
            .unwrap();
        headers.append("Content-Type: application/json").unwrap();
        headers
    }

    fn get_payload(&self, prompt: &str) -> String {
        format!("{{\"model\": \"{}\", \"object\": \"chat.completion\", \"prompt\": \"{}\", \"max_tokens\": 500, \"stop\": \"***\"}}", self.model.to_string() , prompt)
    }
}
