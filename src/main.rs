mod schema; mod gap; mod world;

use bevy::prelude::*;
use world::*;
use gap::*;
use std::sync::{Arc,Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init();
  log::info!("Starting GAP ECS server...");
  let mut app = App::new();
  app.add_plugins(MinimalPlugins)
     .insert_resource(Tick{ n:0, hz:30 })
     .insert_resource(Ui{ can_act: true })
     .add_systems(Startup, setup)
     .add_systems(Update, (tick,).chain());

  // Create shared world state for WebSocket server
  let mut shared_world = World::new();
  shared_world.insert_resource(Tick{ n:0, hz:30 });
  shared_world.insert_resource(Ui{ can_act: true });
  setup_world(&mut shared_world);
  
  let world_ptr = WorldPtr(Arc::new(Mutex::new(shared_world)));
  let app_state = AppState(world_ptr.clone(), 30);
  log::info!("Spawning WebSocket server on ws://127.0.0.1:7777");
  let server = spawn_server(app_state).await?;
  app.insert_resource(server);
  log::info!("Server spawned successfully, starting game loop");

  // run headless game loop
  loop {
    app.update();
    
    // Update shared world with current app world state
    {
      let mut shared = world_ptr.0.lock().unwrap();
      if let Some(tick) = app.world().get_resource::<Tick>() {
        shared.insert_resource(*tick);
      }
      if let Some(ui) = app.world().get_resource::<Ui>() {
        shared.insert_resource(ui.clone());
      }
    }
    
    tokio::time::sleep(Duration::from_millis(33)).await;
  }
  Ok(())
}
