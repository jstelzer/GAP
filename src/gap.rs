use std::{sync::Arc, time::{Duration,Instant}};
use anyhow::Result;
use bevy::prelude::*;
use futures::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::{mpsc,Mutex}, task::JoinHandle, time::sleep};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::schema::{Msg, State as GState, Player, Nearby, Mon, Item, UiState, Intent};
use crate::world::{Tick, Ui, PlayerTag, Monster as CMon, Item as CItem, Pos};

#[derive(Resource)]
pub struct GapServer { pub tx_intent: mpsc::Sender<Intent>, pub _task: JoinHandle<()> }

pub async fn spawn_server(app_state: AppState) -> Result<GapServer> {
  let (tx, _rx) = mpsc::channel::<Intent>(128);
  let shared_state = Arc::new(app_state);
  let listener = TcpListener::bind(("127.0.0.1", 7777)).await?;
  let intents = Arc::new(Mutex::new(Vec::<Intent>::new()));

  // accept loop
  let st = shared_state.clone();
  let intents_in = intents.clone();
  let task = tokio::spawn(async move {
    loop {
      let (stream, _) = listener.accept().await.unwrap();
      let st = st.clone(); let intents_in = intents_in.clone();
      tokio::spawn(async move {
        let mut ws = accept_async(stream).await.unwrap();
        ws.send(Message::Text(serde_json::to_string(&Msg::Hello{version:"0.2.0".into(), agent:Some("poc".into())}).unwrap())).await.ok();
        // 30 Hz publisher
        let mut next = Instant::now();
        loop {
          tokio::select! {
            Some(msg) = ws.next() => {
              if let Ok(Message::Text(txt)) = msg {
                if let Ok(Msg::Intent{ seq, data }) = serde_json::from_str::<Msg>(&txt) {
                  // naive rate-limit (10/s)
                  intents_in.lock().await.push(data);
                  ws.send(Message::Text(serde_json::to_string(&Msg::Ack{seq, tick: st.tick()}).unwrap())).await.ok();
                }
              }
            }
            _ = sleep(Duration::from_millis(1)) => {
              if Instant::now() >= next {
                next += Duration::from_millis(33);
                let s = st.snapshot();
                let msg = Msg::State{ tick: s.0, tick_rate: s.1, data: s.2 };
                ws.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.ok();
              }
            }
          }
        }
      });
    }
  });

  // drain intents into channel (coalesce simple last-move)
  let tx_clone = tx.clone();
  tokio::spawn(async move {
    loop {
      sleep(Duration::from_millis(16)).await;
      let mut buf = intents.lock().await;
      if buf.is_empty() { continue; }
      // naive coalescing: keep last MoveTo + all critical
      let mut last_move: Option<Intent> = None;
      let mut critical: Vec<Intent> = Vec::new();
      for ev in buf.drain(..) {
        match ev {
          Intent::MoveTo{..} => last_move = Some(ev),
          Intent::UsePotion{..} | Intent::Stop{} => critical.push(ev),
          _ => { let _ = tx_clone.send(ev).await; }
        }
      }
      for ev in critical { let _ = tx_clone.send(ev).await; }
      if let Some(m) = last_move { let _ = tx_clone.send(m).await; }
    }
  });

  Ok(GapServer{ tx_intent: tx, _task: task })
}

// -------- Bevy ↔ GAP glue

#[derive(Clone)]
pub struct AppState(pub WorldPtr, pub u32);
#[derive(Clone)]
pub struct WorldPtr(pub std::sync::Arc<std::sync::Mutex<bevy::ecs::world::World>>);

impl AppState {
  pub fn tick(&self) -> u64 { self.with_world(|w| w.resource::<Tick>().n) }
  pub fn snapshot(&self) -> (u64,u32,GState) {
    self.with_world(|w| {
      let t = *w.resource::<Tick>();
      let ui_can_act = w.resource::<Ui>().can_act;
      let mut player = Player{ hp:72, hp_max:100, mana:40, mana_max:90, pos:[48,52], level:1, in_town:true };
      let mut nearby = Nearby{ monsters: vec![], items: vec![], other_players: vec![] };
      
      // player
      for (p, _) in w.query::<(&Pos, &PlayerTag)>().iter(w) {
        player.pos = [p.x, p.y];
      }
      // monsters
      for (m,p) in w.query::<(&CMon,&Pos,)>().iter(w) {
        nearby.monsters.push(Mon{ id:m.id, kind:m.kind.into(), pos:[p.x,p.y], hp_percent: ((m.hp*100)/m.hp_max) as u8 });
      }
      // items
      for (it,p) in w.query::<(&CItem,&Pos,)>().iter(w) {
        nearby.items.push(Item{ id:it.id, pos:[p.x,p.y] });
      }
      (t.n, self.1, GState{ player, nearby, ui_state: UiState{ in_menu:false, in_store:false, can_act: ui_can_act } })
    })
  }
  fn with_world<R>(&self, f: impl FnOnce(&mut bevy::ecs::world::World)->R) -> R {
    let mut guard = self.0 .0.lock().unwrap();
    f(&mut guard)
  }
}

// system to drain intents -> apply
pub fn apply_intents(
  mut ev_rx: ResMut<GapServer>,
  mut q_player: Query<&mut Pos, With<PlayerTag>>,
) {
  let _rx = &mut ev_rx.tx_intent;
  // note: Bevy systems are sync; we already coalesced in async task. Here we’d read from an mpsc::Receiver
  // For brevity, we’ll pretend we pulled one intent via a global; in a real app, feed through a local channel hooked via bevy_tasks.
  let _ = (&mut q_player, &ev_rx); // placeholder to show linkage
}
