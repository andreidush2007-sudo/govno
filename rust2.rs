use reqwest::Url;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Введите URL: ");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    let url_str = input.trim();
    
    match check_secure_connection(url_str).await {
        Ok(is_secure) => {
            if is_secure {
                println!("✅ Соединение защищено (HTTPS)");
            } else {
                println!("❌ Соединение НЕ защищено (HTTP)");
            }
        }
        Err(e) => {
            println!("⚠️ Ошибка при проверке: {}", e);
        }
    }
    
    Ok(())
}

async fn check_secure_connection(url_str: &str) -> Result<bool, Box<dyn Error>> {
    // Добавляем схему по умолчанию, если её нет
    let url_str = if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
        format!("https://{}", url_str)
    } else {
        url_str.to_string()
    };
    
    let url = Url::parse(&url_str)?;
    
    // Проверяем схему URL
    let has_https_scheme = url.scheme() == "https";
    
    if !has_https_scheme {
        return Ok(false);
    }
    
    // Пытаемся установить соединение и проверить сертификат
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(false) // Не принимаем невалидные сертификаты
        .build()?;
    
    match client.get(url).send().await {
        Ok(response) => {
            // Если получили ответ, значит HTTPS работает
            Ok(response.url().scheme() == "https")
        }
        Err(e) => {
            // Проверяем, является ли ошибка связанной с сертификатом
            if e.is_connect() || e.is_timeout() {
                Err(format!("Ошибка соединения: {}", e).into())
            } else {
                // Другие ошибки (например, проблемы с сертификатом)
                Ok(false)
            }
        }
    }
}

[package]
name = "url_checker"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }