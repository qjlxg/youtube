use tokio::fs;

use anyhow::Result;
use serde_yaml::Value;
use serde_yaml::{self, Mapping};

mod request;
mod types;
use request::get_config;
use types::Config;
#[tokio::main]
async fn main() -> Result<()> {
    get().await?;

    let url1 = "https://github.com/qjlxg/aggregator/raw/main/data/clash.yaml";
    let url2 = "https://github.com/qjlxg/hy2/raw/main/configtg.txt";
    let url3 = "https://github.com/qjlxg/proxy-minging/raw/main/v2ray.txt";
    // 获取url1文件名
    let filename1 = url1.split("/").last().unwrap();
    let filename2 = url2.split("/").last().unwrap();
    let filename3 = url3.split("/").last().unwrap();

    // let filename1 = "free.yaml";
    // let filename2 = "rx.yaml";
    // let filename3 = "srx.yaml";

    let (r1, r2, r3) = tokio::join!(
        get_yaml(url1, filename1),
        get_yaml(url2, filename2),
        get_yaml(url3, filename3),
    );
    (r1?, r2?, r3?);

    println!("下载完成");
    Ok(())
}

// 读取并解析yaml文件
async fn read_yaml(file_path: &str) -> Result<Config> {
    let content = fs::read_to_string(file_path).await?;
    let config = serde_yaml::from_str(&content)?;
    return Ok(config);
}
// 获取配置文件
async fn get() -> Result<()> {
    //   let url = "https://mirror.ghproxy.com/https://raw.githubusercontent.com/ssrsub/ssr/master/Clash.yml";
    let url = "https://github.com/qjlxg/SubCrawler/raw/main/sub/share/vless";
    let content = get_config(url).await?;
    fs::write("./clash.yaml", &content).await?;
    Ok(())
}

// 获取配置文件
async fn get_yaml(url: &str, filename: &str) -> Result<()> {
    let content = get_config(url).await?;
    fs::write(filename, &content).await?;
    let content: Config = serde_yaml::from_str(&content)?;

    let clash = read_yaml("./clash.yaml").await?;
    let mut templ = read_yaml("./templ.yaml").await?;

    templ.proxies = content.proxies.clone();
    templ.rules = clash.rules;

    let groups: Mapping = content
        .proxy_groups
        .iter()
        .filter_map(|group| {
            if let Value::Mapping(group) = group {
                Some((group["name"].clone(), group["proxies"].clone()))
            } else {
                None
            }
        })
        .collect();

    for group in &mut templ.proxy_groups {
        if !group.is_mapping() {
            continue;
        }
        let group = group.as_mapping_mut().unwrap();
        let name = group["name"].as_str().unwrap_or("").to_owned();

        if let Some(proxies) = group["proxies"].as_sequence_mut() {
            match name.as_str() {
                "🔰 节点选择" => {
                    proxies.extend(
                        groups["手动切换"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "♻️ 自动选择" => {
                    proxies.extend(
                        groups["自动选择"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "负载均衡" => {
                    proxies.extend(
                        groups["负载均衡"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "🌍 国外媒体" => {
                    proxies.extend(
                        groups["负载均衡"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "🌏 国内媒体" => {
                    proxies.extend(
                        groups["中国节点"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "Ⓜ️ 微软服务" => {
                    proxies.extend(
                        groups["手动切换"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "📲 电报信息" => {
                    proxies.extend(
                        groups["手动切换"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "🍎 苹果服务" => {
                    proxies.extend(
                        groups["手动切换"]
                            .as_sequence()
                            .unwrap_or(&vec![])
                            .iter()
                            .cloned(),
                    );
                }
                "🎯 全球直连" => {}
                "🛑 全球拦截" => {}
                "🐟 漏网之鱼" => {}
                _ => return Err(anyhow::anyhow!("unsupported group: {:?}", group)),
            }
        } else {
            return Err(anyhow::anyhow!("proxies not found"));
        }
    }
    let config = serde_yaml::to_string(&templ).unwrap();

    fs::write(format!("config_{filename}"), &config).await?;
    Ok(())
}
