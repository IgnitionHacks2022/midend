use bluer::{Adapter, AdapterEvent, Address, DeviceEvent};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use anyhow::Result;
use std::{collections::HashSet, env};

async fn query_device(adapter: &Adapter, addr: Address) -> Result<()> {
    let device = adapter.device(addr)?;
    println!("    Address type:       {}", device.address_type().await?);
    println!("    Name:               {:?}", device.name().await?);
    println!("    Icon:               {:?}", device.icon().await?);
    println!("    Class:              {:?}", device.class().await?);
    println!("    UUIDs:              {:?}", device.uuids().await?.unwrap_or_default());
    println!("    Paried:             {:?}", device.is_paired().await?);
    println!("    Connected:          {:?}", device.is_connected().await?);
    println!("    Trusted:            {:?}", device.is_trusted().await?);
    println!("    Modalias:           {:?}", device.modalias().await?);
    println!("    RSSI:               {:?}", device.rssi().await?);
    println!("    TX power:           {:?}", device.tx_power().await?);
    println!("    Manufacturer data:  {:?}", device.manufacturer_data().await?);
    println!("    Service data:       {:?}", device.service_data().await?);
    Ok(())
}


pub async fn rssi_by_inquiry() -> Result<()> {

    println!("hello");
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!("Discovering devices using Bluetooth adapater {}\n", adapter.name());
    adapter.set_powered(true).await?;
    println!("powered on");

    let device_events = adapter.discover_devices().await?;
    pin_mut!(device_events);
    

    let mut all_change_events = SelectAll::new();

    loop {
        tokio::select! {
            Some(device_event) = device_events.next() => {
                match device_event {
                    AdapterEvent::DeviceAdded(addr) => {

                        println!("Device added: {}", addr);
                        let res = query_device(&adapter, addr).await;
                        if let Err(err) = res {
                            println!("    Error: {}", &err);
                        }

                        let device = adapter.device(addr)?;
                        let change_events = device.events().await?.map(move |evt| (addr, evt));
                        all_change_events.push(change_events);
                    }
                    AdapterEvent::DeviceRemoved(addr) => {
                        println!("Device removed: {}", addr);
                    }
                    _ => (),
                }
                println!();
            }
            Some((addr, DeviceEvent::PropertyChanged(property))) = all_change_events.next() => {

                println!("Device changed: {}", addr);
                println!("    {:?}", property);
            }
            else => break
        }
    }

    Ok(())
}

