use std::fs::File;
use std::io::BufReader;

use atom_syndication::Feed;

async fn get_channel() -> anyhow::Result<Feed> {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:97.0) Gecko/20100101 Firefox/97.0";
    let client = reqwest::Client::builder().user_agent(ua).build()?;
   let content = client
       .get("https://www.sec.gov/Archives/edgar/xbrlrss.all.xml")
       .send()
       .await?
       .bytes()
       .await?;
 dbg!(&content);
    let feed = Feed::read_from(&content[..])?;
// let file = File::open("example.xml")?;
// let feed = Feed::read_from(BufReader::new(file))?;
    Ok(feed)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = get_channel().await?;
    for item in result.entries() {
        for cat in &item.categories {
            println!("{:?}", cat)
        }
    }
    Ok(())
}
