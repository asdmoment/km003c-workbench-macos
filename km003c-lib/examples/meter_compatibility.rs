//! Read-only compatibility check: no reset, firmware writes, or calibration writes.
use km003c_lib::{Attribute, AttributeSet, DeviceConfig, GraphSampleRate, KM003C};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vendor = std::env::args().any(|arg| arg == "--vendor");
    let config = if vendor {
        DeviceConfig::vendor()
    } else {
        DeviceConfig::hid()
    };
    let mut device = KM003C::new(config.skip_reset()).await?;
    if let Some(state) = device.state() {
        println!(
            "Model: {}; firmware: {}; streaming: {}",
            state.model(),
            state.firmware_version(),
            state.adcqueue_enabled
        );
    }
    println!("{:?}", device.request_adc_data().await?);
    if vendor {
        for rate in [
            GraphSampleRate::Sps2,
            GraphSampleRate::Sps10,
            GraphSampleRate::Sps50,
            GraphSampleRate::Sps1000,
        ] {
            device.start_graph_mode(rate).await?;
            tokio::time::sleep(std::time::Duration::from_millis(700)).await;
            let response = device.request_data(AttributeSet::single(Attribute::AdcQueue)).await;
            device.stop_graph_mode().await?;
            let packet = response?;
            let count = packet.get_adc_queue().map_or(0, |queue| queue.samples.len());
            println!("{rate:?}: {count} samples");
            if count == 0 {
                return Err("stream returned no samples".into());
            }
        }
    }
    Ok(())
}
