use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::WithExportConfig as _;
use tracing::{info};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_logs_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint("http://localhost:4318")
        .build()?;
    let logs_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_batch_exporter(otlp_logs_exporter)
        .build();

    let otel_layer_logs = OpenTelemetryTracingBridge::new(&logs_provider);

    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(otel_layer_logs)
        .init();

    info!("Please log me");

    logs_provider.shutdown()?;
    Ok(())
}
