use std::collections::HashMap;
use std::env;
use tokio::io::AsyncWriteExt;

pub async fn get_unicode_data() -> Result<String, Box<dyn std::error::Error>> {
    use tokio::fs;

    let mut temp_dir = env::temp_dir();
    temp_dir.push("termicode");
    temp_dir.push("data.txt");
    
    if let Ok(bytes) = fs::read(&temp_dir).await {
        return Ok(String::from_utf8_lossy(&bytes).to_string());
    }

    println!("Updating cache...");

    let unicode_data = reqwest::get("https://unicode.org/Public/UNIDATA/UnicodeData.txt")
        .await?
        .text()
        .await?;

    temp_dir.pop();
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).await?;
    }
    temp_dir.push("data.txt");

    let mut file = fs::File::create(&temp_dir).await?;
    file.write_all(&unicode_data.as_bytes()).await?;

    Ok(unicode_data)
}

#[derive(Clone)]
pub struct UnicodeData {
    pub codepoint: u32,
    pub name: String,
    pub category: String,
}

fn get_codepoint_gaps(data: &String) -> Vec<(u32, u32)> {
    let nums = data.split("\n")
        .filter(|line| line.len() != 0)
        .map(|line| line.split(";").nth(0).unwrap());

    let mut gaps = vec![];
    let mut last_num = 0;
    for num_str in nums.skip(1) {
        if let Ok(num) = u32::from_str_radix(num_str, 16) {
            if last_num + 1 != num {
                gaps.push((last_num, num));
            }
            last_num = num;
        }
    }

    gaps
}

pub struct UnicodeFile {
    pub gaps: Vec<(u32, u32)>,
    pub map: HashMap<u32, UnicodeData>,
}

pub fn parse_unicode_data(data: &String) -> UnicodeFile {
    let gaps = get_codepoint_gaps(data);

    let map = data.split("\n")
        .into_iter()
        .filter(|line| line.len() != 0)
        .map(|line| {
            let mut data = line.split(";");

            let codepoint = u32::from_str_radix(data.next().unwrap(), 16).unwrap();

            let name = data.next().unwrap().to_owned();
            let category = data.next().unwrap().to_owned();

            (codepoint, UnicodeData { codepoint, name, category })
        })
        .collect();

    UnicodeFile { map, gaps }
}