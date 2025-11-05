use tauri::State;
use tracing::{info, instrument};

use crate::error::BackendError;
use crate::grpc::client::{GrpcClient, Node};
use crate::grpc::socks5::{Socks5Settings, HttpRpcSettings, Socks5Status};

#[instrument(skip_all)]
#[tauri::command]
pub async fn enable_socks5(
    grpc: State<'_, GrpcClient>,
    socks5_settings: Socks5Settings,
    http_rpc_settings: HttpRpcSettings,
    exit: Node,
) -> Result<(), BackendError> {
    info!("enabling SOCKS5 proxy with exit_node: {}", exit);
    grpc.enable_socks5(socks5_settings, http_rpc_settings, exit).await?;
    Ok(())
}

#[instrument(skip_all)]
#[tauri::command]
pub async fn disable_socks5(grpc: State<'_, GrpcClient>) -> Result<(), BackendError> {
    info!("disabling SOCKS5 proxy");
    grpc.disable_socks5().await?;
    Ok(())
}

#[instrument(skip_all)]
#[tauri::command]
pub async fn get_socks5_status(
    grpc: State<'_, GrpcClient>,
) -> Result<Socks5Status, BackendError> {
    let status = grpc.get_socks5_status().await?;
    Ok(status.into())
}

