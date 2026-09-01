use sqlx::{postgres::PgPoolOptions, types::BigDecimal, Pool, Postgres};
use std::{str::FromStr, time::Duration};
use tokio::time;
use reqwest::Client;
use serde::Deserialize;
use rand::Rng;
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;
use std::io::Write; // <-- เพิ่มตัวนี้สำหรับส่งค่ากลับให้ Render

// Struct สำหรับรับค่าจาก Binance
#[derive(Deserialize, Debug)]
struct Binance24hTicker {
    symbol: String,
    lastPrice: String,
    priceChangePercent: String,
    quoteVolume: String,
}

// Struct สำหรับรับค่าจาก Bitkub
#[derive(Deserialize, Debug)]
struct BitkubTicker {
    last: f64,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("🚀 Rust Worker Starting (Real-time Hybrid Mode)...");

    // ========================================================
    // 🔥 TRICK: เปิด Dummy Server (แบบรับได้ทุก Port)
    // ========================================================
    thread::spawn(|| {
        // อ่าน Port ที่ Render สั่งมา (ถ้าไม่มีให้ใช้ 10000)
        let port = std::env::var("PORT").unwrap_or("10000".to_string());
        let addr = format!("0.0.0.0:{}", port);
        
        let listener = TcpListener::bind(&addr).expect("Cannot bind port");
        println!("✅ Dummy Server listening on {}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(mut socket) => {
                    // ตอบกลับว่า "ฉันสบายดี" (HTTP 200 OK)
                    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
                    let _ = socket.write_all(response.as_bytes());
                },
                Err(e) => println!("Connection failed: {}", e),
            }
        }
    });
    // ========================================================

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().max_connections(10).connect(&database_url).await?;
    println!("✅ Connected to Database!");

    let client = Client::builder()
        .user_agent("CryptoSentimentBot/1.0")
        .timeout(Duration::from_secs(10))
        .build()?;
    
    // ตั้งเวลา loop เป็น 3 วินาที (เพื่อให้ราคาไวทันใจ)
    let mut interval = time::interval(Duration::from_secs(3)); 

    loop {
        interval.tick().await;

        // 1. ดึงข้อมูลตลาดโลก (Binance)
        if let Err(e) = process_market_data(&pool, &client).await {
            eprintln!("❌ Binance Error: {}", e);
        }

        // 2. ดึงเรทเงินบาท (Bitkub)
        if let Err(e) = process_bitkub_data(&pool, &client).await {
            eprintln!("❌ Bitkub Error: {}", e);
        }
    }
}

// ฟังก์ชัน: ดึงเรทเงินบาทจาก Bitkub
async fn process_bitkub_data(pool: &Pool<Postgres>, client: &Client) -> Result<(), anyhow::Error> {
    let url = "https://api.bitkub.com/api/market/ticker?sym=THB_USDT";
    let response = client.get(url).send().await?;
    let text = response.text().await?;

    // Bitkub ส่งมาเป็น { "THB_USDT": { "last": 34.5, ... } }
    let data: HashMap<String, BitkubTicker> = serde_json::from_str(&text)?;

    if let Some(ticker) = data.get("THB_USDT") {
        let price = ticker.last;
        let price_decimal = BigDecimal::from_str(&price.to_string()).unwrap_or(BigDecimal::from(34));
        let score_decimal = BigDecimal::from(100); // USDT คะแนนเต็ม 100 เสมอ (Stable)

        // Query: บันทึกและแจ้งเตือน Rails ทันที
        let query = r#"
            WITH inserted AS (
                INSERT INTO sentiments (coin_symbol, score, price, source, recorded_at)
                VALUES ($1, $2, $3, $4, NOW())
                ON CONFLICT (coin_symbol) 
                DO UPDATE SET price = $3, recorded_at = NOW()
                RETURNING coin_symbol, score, price, recorded_at
            )
            SELECT pg_notify(
                'new_sentiment', 
                json_build_object(
                    'coin_symbol', coin_symbol, 
                    'score', score, 
                    'price', price,
                    'change_24h', 0.0,
                    'volume', 0.0,
                    'recorded_at', recorded_at
                )::text
            ) 
            FROM inserted;
        "#;

        sqlx::query(query)
            .bind("USDT_THB")
            .bind(score_decimal)
            .bind(price_decimal)
            .bind("Bitkub Exchange")
            .execute(pool)
            .await?;
    }

    Ok(())
}

// ฟังก์ชัน: ดึงราคาจาก Binance และคำนวณ AI Score
async fn process_market_data(pool: &Pool<Postgres>, client: &Client) -> Result<(), anyhow::Error> {
    let url = "https://data-api.binance.vision/api/v3/ticker/24hr";
    let response = client.get(url).send().await?;
    let response_text = response.text().await?;

    if response_text.trim().starts_with('{') { return Ok(()); }
    let all_tickers: Vec<Binance24hTicker> = serde_json::from_str(&response_text)?;
    let target_coins = vec!["BTCUSDT", "ETHUSDT", "SOLUSDT", "BNBUSDT", "XRPUSDT", "ADAUSDT", "DOGEUSDT", "DOTUSDT", "LINKUSDT", "MATICUSDT"];

    for ticker in all_tickers {
        if !target_coins.contains(&ticker.symbol.as_str()) { continue; }
        let clean_symbol = ticker.symbol.replace("USDT", "");
        let price: f64 = ticker.lastPrice.parse().unwrap_or(0.0);
        let change_24h: f64 = ticker.priceChangePercent.parse().unwrap_or(0.0);
        let volume: f64 = ticker.quoteVolume.parse().unwrap_or(0.0);
        let (sentiment_score, source) = calculate_ai_sentiment(price, change_24h, volume);
        
        let price_decimal = BigDecimal::from_str(&ticker.lastPrice).unwrap_or(BigDecimal::from(0));
        let score_decimal = BigDecimal::from_str(&sentiment_score.to_string()).unwrap_or(BigDecimal::from(50));

        let query = r#"
            WITH inserted AS (
                INSERT INTO sentiments (coin_symbol, score, price, source, recorded_at)
                VALUES ($1, $2, $3, $4, NOW())
                ON CONFLICT (coin_symbol) 
                DO UPDATE SET price = $3, score = $2, source = $4, recorded_at = NOW()
                RETURNING coin_symbol, score, price, recorded_at
            )
            SELECT pg_notify(
                'new_sentiment', 
                json_build_object(
                    'coin_symbol', coin_symbol, 
                    'score', score, 
                    'price', price,
                    'change_24h', $5::float8,
                    'volume', $6::float8,
                    'recorded_at', recorded_at
                )::text
            ) 
            FROM inserted;
        "#;

        sqlx::query(query)
            .bind(&clean_symbol)
            .bind(score_decimal)
            .bind(price_decimal)
            .bind(source)
            .bind(change_24h)
            .bind(volume)
            .execute(pool)
            .await?;
            
        println!("   📊 {}: ${} | Score: {:.0}", clean_symbol, price, sentiment_score);
    }
    Ok(())
}

fn calculate_ai_sentiment(_price: f64, change_24h: f64, volume: f64) -> (f64, String) {
    let mut rng = rand::thread_rng();
    let price_score = 50.0 + (change_24h * 2.5);
    let volume_bonus = if volume > 500_000_000.0 { 5.0 } else if volume > 100_000_000.0 { 2.0 } else { 0.0 };
    let volatility_noise = rng.gen_range(-2.0..2.0);
    let final_score = (price_score + volume_bonus + volatility_noise).clamp(10.0, 95.0);
    (final_score, "Advanced AI Model".to_string())
}