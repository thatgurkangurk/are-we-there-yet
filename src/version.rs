use ferinth::Ferinth;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;

use anyhow::Result;

use crate::modrinth;

pub async fn check_mods_status_concurrent(
    mods: Vec<String>,
    ferinth: &Ferinth<()>,
    version: &str,
) -> Result<HashMap<String, bool>> {
    let mut results = HashMap::new();

    // map mods into async futures
    let futures = mods.into_iter().map(|mc_mod| {
        let version = version.to_string();
        async move {
            let status = modrinth::is_on_version(ferinth, &mc_mod, &version).await;
            (mc_mod, status)
        }
    });

    let mut stream = FuturesUnordered::from_iter(futures);

    while let Some((mod_name, status)) = stream.next().await {
        match status {
            Ok((name, is_on_version)) => {
                results.insert(name, is_on_version);
            }
            Err(e) => eprintln!("error checking {}: {}", mod_name, e),
        }
    }

    Ok(results)
}
