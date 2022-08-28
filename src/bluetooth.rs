use std::{collections::HashSet, env, thread, time::Duration};

use anyhow::{anyhow, Result};
use bluer::{Adapter, AdapterEvent, Address, Device, DeviceEvent};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use log::info;

pub async fn rssi_by_inquiry() -> Result<Vec<String>> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    info!(
        "Discovering devices using Bluetooth adapater {}...\n",
        adapter.name()
    );
    adapter.set_powered(true).await?;

    let device_events = adapter.discover_devices().await?;
    thread::sleep(Duration::from_secs(3));

    let mut addrs = adapter.device_addresses().await?;
    let mut named_addrs: Vec<(String, i16)> = Vec::new();

    info!("Discovered devices:");
    for addr in addrs.iter() {
        let device = adapter.device(*addr)?;
        match device.name().await? {
            Some(x) => {
                info!("- {}", x.clone());
                let rssi = device.rssi().await?;
                if let Some(rssi) = rssi {
                    named_addrs.push((x, rssi));
                }
            },
            None => (),
        }
    }
    named_addrs.sort_by(|(n1, r1), (n2, r2)| r2.cmp(r1));

    Ok(named_addrs.iter().map(|(id, _)| id.to_owned()).collect())
}
