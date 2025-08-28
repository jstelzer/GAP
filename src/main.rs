mod schema; mod gap; mod world;

use bevy::prelude::*;
use schema::*; use world::*;
use gap::*;
use std::sync::{Arc,Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut app = App::new();
  app.insert_resource(Tick{ n:0, hz:30 })
     .insert_resource(Ui{ can_act: true })
     .add_systems(Startup, setup)
     .add_systems(Update, (tick,).chain());

  // share world to WS task for snapshots
  let world_ptr = WorldPtr(Arc::new(Mutex::new(app.world_mut().to_world())));
  let app_state = AppState(world_ptr.clone(), 30);
  let server = spawn_server(app_state).await?;
  app.insert_resource(server);

  // drain intents & apply + publish is already handled in gap.rs / server task
  app.run();
  Ok(())
}
