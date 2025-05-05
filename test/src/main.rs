use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

fn main() {
    let mut service = db::DbService::new();
    let stock = NewStock {
        isin: "SI0031103805".to_string(),
    };

    service.create_stock(stock);

    println!("{:?}", service.list_stocks());
}




/*#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ids = [
        "SI0031103805",
        "SI0031102120",
        "SI0031101346",
        "SI0021117344",
        "SI0031102153",
        "SI0021110513",
        "SI0031104290",
        "SI0021111651",

        "HRADPLRA0006",
        "HRARNTRA0004",
        "HRATGRRA0003",
        "HRHT00RA0005",
        "HRPODRRA0004",
        "HRRIVPRA0000",
    ];

    let stocks_dir = Path::new("data_stocks");
    if !stocks_dir.exists() {
        fs::create_dir(stocks_dir)?;
    }

    for id in ids.iter() {
        let mut url = format!(
            "https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/security-history/XLJU/{}/2018-01-03/2025-04-30/json",
            id
        );

        if id.starts_with("HR") {
            url = format!(
                "https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/security-history/XZAG/{}/2018-01-03/2025-04-30/json",
                id
            );
        }

        println!("Fetching data for ISIN: {}", id);

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        // Parse the JSON to extract the "symbol" field
        let json: serde_json::Value = serde_json::from_str(&body)?;
        let symbol = json["symbol"]
            .as_str()
            .unwrap_or("unknown_symbol");

        let file_path = stocks_dir.join(format!("{}.json", symbol));
        let mut file = File::create(&file_path).await?;
        file.write_all(body.as_bytes()).await?;

        println!("Saved to: data/{}.json", symbol);
    }


    // INDEXES:
    let indexes = [
        "SI0026109882",
        "SI0028409892",

        "HRZB00ICBEX6",
        "HRZB00ICBTR6",
        "HRZB00ICBE11",
        "HRZB00ICB103",
        "HRZB00ICBPR4",
        "HRZB00ICBEP2",

        "HRZB00IADPR4",
    ];

    let indexes_dir = Path::new("data_indexes");
    if !indexes_dir.exists() {
        fs::create_dir(indexes_dir)?;
    }

    for id in indexes.iter() {
        let mut url = format!(
            "https://rest.ljse.si/web/Bvt9fe2peQ7pwpyYqODM/index-history/XLJU/{}/2018-01-03/2025-04-30/json",
            id
        );

        if id.starts_with("HR") {
            url = format!(
                "https://rest.zse.hr/web/Bvt9fe2peQ7pwpyYqODM/index-history/XZAG/{}/2018-01-03/2025-04-30/json",
                id
            );
        }

        println!("Fetching data for ISIN: {}", id);

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        // Parse the JSON to extract the "symbol" field
        let json: serde_json::Value = serde_json::from_str(&body)?;
        let symbol = json["symbol"]
            .as_str()
            .unwrap_or("unknown_symbol");

        let file_path = indexes_dir.join(format!("{}.json", symbol));
        let mut file = File::create(&file_path).await?;
        file.write_all(body.as_bytes()).await?;

        println!("Saved to: data/{}.json", symbol);
    }

    Ok(())
}
*/