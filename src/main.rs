use sentry::metrics::FractionUnit;
use sentry::metrics::Metric;
use std::env;
use std::time::Duration;
use sysinfo::System;

fn main() {
		let sentry_dsn = env::var("SENTRY_DSN").expect("SENTRY_DSN not set");
		let _guard = sentry::init((sentry_dsn, sentry::ClientOptions {
			release: sentry::release_name!(),
			..Default::default()
		}));

		let mut sys = System::new_all();
		loop {
				sys.refresh_memory();
				let memory = sys.used_memory() as f64 / sys.total_memory() as f64;
				println!("sending {}", memory);
				Metric::distribution("memory", memory).with_unit(FractionUnit::Ratio).send();
				std::thread::sleep(Duration::from_secs(3));
		}
}
