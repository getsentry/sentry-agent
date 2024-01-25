use std::time::Duration;
use sentry::metrics::Metric;
use sentry::metrics::FractionUnit;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
		let _guard = sentry::init(("https://55353bf1f574fa979e96733a2238eda6@o1.ingest.sentry.io/4506634220929024", sentry::ClientOptions {
			release: sentry::release_name!(),
			..Default::default()
		}));

    loop {
        sys.refresh_memory();
        let memory = sys.used_memory() as f64 / sys.total_memory() as f64;
        println!("sending {}", memory);
        Metric::distribution("memory", memory).with_unit(FractionUnit::Ratio).send();
        std::thread::sleep(Duration::from_secs(15));
    }
}
