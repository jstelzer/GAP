use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(tag="type")]
pub enum Msg {
  #[serde(rename="hello")]
  Hello { version: String, agent: Option<String> },
  #[serde(rename="state")]
  State { tick: u64, tick_rate: u32, data: State },
  #[serde(rename="intent")]
  Intent { seq: u64, data: Intent },
  #[serde(rename="ack")]
  Ack { seq: u64, tick: u64 },
  #[serde(rename="error")]
  Error { seq: Option<u64>, reason: String },
  #[serde(rename="ping")]
  Ping,
  #[serde(rename="pong")]
  Pong,
}

#[derive(Serialize,Deserialize,Clone,Debug,Default)]
pub struct State {
  pub player: Player,
  pub nearby: Nearby,
  pub ui_state: UiState,
}

#[derive(Serialize,Deserialize,Clone,Debug,Default)]
pub struct Player {
  pub hp: i32, pub hp_max: i32,
  pub mana: i32, pub mana_max: i32,
  pub pos: [i32;2], pub level: i32, pub in_town: bool
}

#[derive(Serialize,Deserialize,Clone,Debug,Default)]
pub struct Nearby {
  pub monsters: Vec<Mon>,
  pub items: Vec<Item>,
  pub other_players: Vec<()>,
}
#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Mon { pub id:i32, pub kind:String, pub pos:[i32;2], pub hp_percent:u8 }
#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Item { pub id:i32, pub pos:[i32;2] }

#[derive(Serialize,Deserialize,Clone,Debug,Default)]
pub struct UiState { pub in_menu: bool, pub in_store: bool, pub can_act: bool }

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(tag="cmd")]
pub enum Intent {
  #[serde(rename="move_to")] MoveTo { x:i32, y:i32, #[serde(rename="targetTick")] target_tick: Option<u64> },
  #[serde(rename="use_potion")] UsePotion { slot: Option<u8> },
  #[serde(rename="say")] Say { text:String },
  #[serde(rename="stop")] Stop {},
}
