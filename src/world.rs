use bevy::prelude::*;

#[derive(Component)] pub struct PlayerTag;
#[derive(Component)] pub struct Monster { pub id:i32, pub hp:i32, pub hp_max:i32, pub kind:&'static str }
#[derive(Component)] pub struct Item { pub id:i32 }
#[derive(Component)] pub struct Pos { pub x:i32, pub y:i32 }
#[derive(Resource,Default)] pub struct Ui { pub can_act: bool }
#[derive(Resource,Default)] pub struct Tick { pub n:u64, pub hz:u32 }

pub fn setup(mut c: Commands) {
  c.spawn((PlayerTag, Pos{ x:48, y:52 }));
  c.spawn((Monster{ id:1, hp:60, hp_max:100, kind:"SK" }, Pos{ x:51, y:54 }));
  c.spawn((Item{ id:101 }, Pos{ x:45, y:50 }));
}

pub fn tick(mut t: ResMut<Tick>) { t.n = t.n.wrapping_add(1); }

pub fn apply_move(target: (i32,i32), mut q: Query<&mut Pos, With<PlayerTag>>) {
  if let Ok(mut p) = q.get_single_mut() { p.x = target.0; p.y = target.1; }
}
