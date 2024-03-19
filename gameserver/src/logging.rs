use tracing::Instrument;

#[macro_export]
macro_rules! log_error {
    ($e:expr) => {
        if let Err(e) = $e {
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        }
    };
    ($context:expr, $e:expr $(,)?) => {
        if let Err(e) = $e {
            let e = format!("{:?}", ::anyhow::anyhow!(e).context($context));
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        }
    };
    ($ok_context:expr, $err_context:expr, $e:expr $(,)?) => {
        if let Err(e) = $e {
            let e = format!("{:?}", ::anyhow::anyhow!(e).context($err_context));
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        } else {
            tracing::info!($ok_context);
        }
    };
}

pub fn init_tracing() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
}

pub async fn init_system_logging() {
    use std::time::Duration;
    use sysinfo::System;

    tokio::spawn(
        async {
            let mut s = System::new_all();
            s.refresh_all();
            let num_cpus = s.cpus().len();
            loop {
                tokio::time::sleep(Duration::from_millis(20000)).await;
                s.refresh_all();
                let process = s.process(sysinfo::get_current_pid().unwrap()).unwrap();
                tracing::info!(
                    cpu_usage = %format!("{:.2}%", process.cpu_usage() / num_cpus as f32)
                );
                let memory = process.memory();
                let formatted = match memory {
                    m if m < 1024 => format!("{m} B"),
                    m if m < 1024 * 1024 => format!("{:.2} KB", m as f32 / 1024.0),
                    m if m < 1024 * 1024 * 1024 => format!("{:.2} MB", m as f32 / 1024.0 / 1024.0),
                    m => format!("{:.2} GB", m as f32 / 1024.0 / 1024.0 / 1024.0),
                };
                tracing::info!(total_memory = %format!("{formatted}"));
            }
        }
        .instrument(tracing::info_span!("system-usage")),
    );
}
