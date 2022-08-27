use bluer::{Adapter, AdapterEvent, Address, Device, DeviceEvent};
use futures::{pin_mut, stream::SelectAll, StreamExt};
use anyhow::Result;
use std::{collections::HashSet, env};
use std::thread;
use std::time::Duration;

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


pub async fn rssi_by_inquiry() -> Result<String>{

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!("Discovering devices using Bluetooth adapater {}\n", adapter.name());
    adapter.set_powered(true).await.unwrap();
    println!("powered on");

    let device_events = adapter.discover_devices().await?;
    


    println!("Scanning for bluetooth addresses");
    thread::sleep(Duration::from_secs(10));

    let mut addrs = adapter.device_addresses().await.unwrap();
    let mut named_addrs: Vec<(String,i16)> = Vec::new();


    for addr in addrs.iter(){
        let device = adapter.device(addr.clone())?;
        match device.name().await?{
            Some(x) => {
                println!("{}",x.clone());
                println!("{}",device.rssi().await?.unwrap());
                named_addrs.push((x,device.rssi().await?.unwrap()));
                


            }
            None =>()
        }
    }
    named_addrs.sort_by(|(n1,r1),(n2,r2)|r2.cmp(r1));



    return Ok(named_addrs[0].0.clone());



    
}

