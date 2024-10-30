use hex;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub struct SignatureService {
    api_key: String,
    api_secret: String,
    client: reqwest::Client,
    headers: HeaderMap,
}

impl SignatureService {
    pub fn new(api_key: String, api_secret: String) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert("X-MEXC-APIKEY", HeaderValue::from_str(&api_key)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Ok(SignatureService {
            api_key,
            api_secret,
            client,
            headers,
        })
    }

    pub fn generate_signature(&self, query_string: &str) -> Result<String, Box<dyn Error>> {
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())?;
        mac.update(query_string.as_bytes());
        let result = mac.finalize().into_bytes();
        Ok(hex::encode(result))
    }

    pub async fn get_api_restrictions(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string();
        let query_string = format!("timestamp={}", timestamp);
        let signature = self.generate_signature(&query_string)?;

        let url = format!(
            "https://api.mexc.com/api/v3/kyc/status?timestamp={}&signature={}",
            timestamp, signature
        );

        // Используем предопределенные клиент и заголовки
        let response = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .send()
            .await?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await?;
            println!(">>>>>>>> {}", json);
            Ok(json)
        } else {
            let error_text = response.text().await?;
            Err(format!("Error from MEXC: {}", error_text).into())
        }
    }
}
