mod identity;
mod ratchet;
mod ble_mesh;
mod tor_service;
mod shard_manager;

use identity::IdentityManager;
use ble_mesh::BleMeshManager;
use tor_service::{TorV3Manager, TorMailboxServer};
use shard_manager::ShardManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[*] Initializing Sarcophagus OS Pulse Pager Engine...");

    // 1. Initialize local app handle and identity seed
    let user_identity = IdentityManager::new("@umbra_fox");
    let profile = user_identity.export_profile();
    println!("[+] Pulse Handle Bound: {}", profile.handle);
    println!("[+] Identity Fingerprint: {}", &profile.public_key_hex[..16]);

    // 2. Initialize BLE Mesh Subsystem
    let mesh_manager = BleMeshManager::new([0x42; 32]);
    let mesh_task = tokio::spawn(async move {
        mesh_manager.start_mesh_listener_loop().await;
    });

    // 3. Initialize Tor V3 Mailbox Subsystem with a pre-shared cryptographic session secret
    let local_mailbox_port = 9052;
    let tor_control_port = 9051;
    let session_secret = [0x42u8; 32]; // Shared symmetric key for E2EE test binding

    let tor_manager = TorV3Manager::new(tor_control_port, local_mailbox_port);
    let mailbox_server = TorMailboxServer::new(local_mailbox_port, session_secret);

    let tor_task = tokio::spawn(async move {
        match tor_manager.provision_ephemeral_onion().await {
            Ok(onion_addr) => println!("[✓] Active Onion Mailbox Address: {}", onion_addr),
            Err(e) => eprintln!("[-] Tor V3 provisioning deferred/failed: {}", e),
        }

        if let Err(e) = mailbox_server.start_mailbox_listener().await {
            eprintln!("[-] Tor Mailbox error: {}", e);
        }
    });

    // 4. Background K-Anonymity Decoy Shard Loop
    let shard_mgr = ShardManager::new([0x77; 32]);
    let decoy_task = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            let decoy_batch = shard_mgr.generate_decoy_batch(4, 256);
            println!("[*] K-ANONYMITY: Cycled batch of {} background decoy shards (IDs: {})", 
                decoy_batch.len(), 
                decoy_batch.first().map(|s| s.shard_id.clone()).unwrap_or_default()
            );
        }
    });

    println!("[✓] Dual-transport active: BLE Mesh & Tor Mailbox ready with E2EE & Sharding.");

    let _ = tokio::try_join!(mesh_task, tor_task, decoy_task)?;

    Ok(())
}