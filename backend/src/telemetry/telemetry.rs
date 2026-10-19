#[cfg(feature = "telemetry")]
use reqwest::blocking::Client;
use serde_json::json;
use std::time::Duration;

#[cfg(feature = "telemetry")]
const PLAUSIBLE_API: &str = "https://analytics.bitptr.com/api/event";
#[cfg(feature = "telemetry")]
const DOMAIN: &str = "inkheart";

pub fn send_heartbeat(version: &str, library_size: usize, telemetry_enabled: bool) {
    #[cfg(feature = "telemetry")]
    {
        if !telemetry_enabled {
            return;
        }

        let version = version.to_string();
        std::thread::spawn(move || {
            if let Err(e) = send_event(&version, library_size) {
                eprintln!("Telemetry error: {}", e);
            }
        });
    }

    #[cfg(not(feature = "telemetry"))]
    {
        // No-op when feature disabled
        let _ = (version, library_size, telemetry_enabled);
    }
}

#[cfg(feature = "telemetry")]
fn send_event(version: &str, library_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

    let library_bucket = get_library_size_bucket(library_size);

    let payload = json!({
        "n": "pageview",
        "d": DOMAIN,
        "u": "app://inkheart",
        "v": 33,
        "p": {
            "Version": version,
            "LibrarySize": library_bucket,
            "Architecture": std::env::consts::ARCH,
        }
    });

    let user_agent = format!("Inkheart/{}", version);

    let response = client
        .post(PLAUSIBLE_API)
        .header("User-Agent", user_agent)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Plausible returned status: {}", response.status()).into());
    }

    Ok(())
}

#[cfg(feature = "telemetry")]
fn get_library_size_bucket(size: usize) -> &'static str {
    match size {
        0 => "empty",
        1..=100 => "1-100",
        101..=500 => "100-500",
        501..=1000 => "500-1k",
        1001..=5000 => "1k-5k",
        _ => "5k+",
    }
}

#[cfg(all(test, feature = "telemetry"))]
mod tests {
    use super::*;

    #[test]
    fn test_library_buckets() {
        assert_eq!(get_library_size_bucket(0), "empty");
        assert_eq!(get_library_size_bucket(50), "1-100");
        assert_eq!(get_library_size_bucket(250), "100-500");
        assert_eq!(get_library_size_bucket(750), "500-1k");
        assert_eq!(get_library_size_bucket(2500), "1k-5k");
        assert_eq!(get_library_size_bucket(10000), "5k+");
    }
}
