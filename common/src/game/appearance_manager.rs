use crate::game::{appearance_table::*, EnemyType, FormationIndex, Traj, TrajCommand};
use crate::utils::math::*;
use vector2d::Vector2D;

const UNIT_COUNT: u32 = 3;
const STEP_WAIT: u32 = 32 / 3;

pub struct NewBorn {
    pub enemy_type: EnemyType,
    pub pos: Vector2D<i32>,
    pub angle: i32,
    pub speed: i32,
    pub fi: FormationIndex,
    pub traj: Traj,
}

impl NewBorn {
    fn new(
        enemy_type: EnemyType,
        pos: Vector2D<i32>,
        angle: i32,
        speed: i32,
        fi: FormationIndex,
        traj: Traj,
    ) -> Self {
        NewBorn { enemy_type, pos, angle, speed, fi, traj }
    }
}

#[derive(Clone)]
struct Info {
    time: u32,
    enemy_type: EnemyType,
    fi: FormationIndex,
    offset: Vector2D<i32>,
    flip_x: bool,
    traj_table: &'static [TrajCommand],
    shot_enable: bool,
}

impl Info {
    pub fn new(
        time: u32,
        enemy_type: EnemyType,
        fi: FormationIndex,
        offset: Vector2D<i32>,
        flip_x: bool,
        traj_table: &'static [TrajCommand],
    ) -> Self {
        Self { time, enemy_type, fi, offset, flip_x, traj_table, shot_enable: false }
    }
}

pub trait Accessor {
    fn is_stationary(&self) -> bool;
}

pub struct AppearanceManager {
    stage: u16,
    wait_stationary: bool,
    wait: u32,
    unit: u32,
    time: u32,
    pub done: bool,
    orders: Vec<Info>,
}

impl Default for AppearanceManager {
    fn default() -> Self {
        Self {
            stage: 0,
            wait_stationary: false,
            wait: 30,
            unit: 0,
            time: 0,
            done: false,
            orders: Vec::new(),
        }
    }
}

impl AppearanceManager {
    pub fn update<A: Accessor>(&mut self, accessor: &A) -> Option<Vec<NewBorn>> {
        if self.done {
            return None;
        }

        self.update_main(accessor)
    }

    fn update_main<A: Accessor>(&mut self, accessor: &A) -> Option<Vec<NewBorn>> {
        if self.wait > 0 {
            self.wait -= 1;
            return None;
        }

        if self.wait_stationary {
            if !accessor.is_stationary() {
                return None;
            }
            self.wait_stationary = false;
        }
        if self.unit >= UNIT_COUNT {
            self.done = true;
            return None;
        }

        if self.orders.is_empty() {
            self.set_orders();
            self.time = 0;
        }

        if self.orders.is_empty() {
            return None;
        }

        let mut new_borns: Vec<NewBorn> = Vec::new();
        while self.orders[0].time == self.time {
            let p = &self.orders[0];
            let traj = Traj::new(p.traj_table, &p.offset, p.flip_x, p.fi);

            let enemy = NewBorn::new(p.enemy_type, Vector2D::new(0, 0), 0, 0, p.fi, traj);
            new_borns.push(enemy);

            self.orders.remove(0);

            if self.orders.is_empty() {
                break;
            }
        }

        self.time += 1;
        if self.orders.is_empty() {
            self.orders.clear();
            self.unit += 1;
            self.wait_stationary = true;
            self.wait = 150;
            self.time = 0;
        }

        Some(new_borns)
    }

    fn set_orders(&mut self) {
        self.create_orders();
    }

    fn create_orders(&mut self) {
        let base = self.unit * 4;
        let entry = &UNIT_TABLE[(self.stage as usize) % UNIT_TABLE.len()][self.unit as usize];
        match entry.pat {
            0 | 1 | 2 => {
                for count in 0..4 {
                    let fi = ORDER[(base + count) as usize];
                    let info = self.create_info(fi, count);
                    self.orders.push(info);
                }
            }
            3 => {
                let flip = if entry.flip_x { 1 } else { 0 };
                for count in 0..4 {
                    let side = count & 1;
                    let fi = ORDER[(base + (count / 2 + (side ^ flip) * 4)) as usize];
                    let info = self.create_info(fi, count);
                    self.orders.push(info);
                }
            }

            _ => {
                panic!("Illegal");
            }
        }
    }

    fn create_info(&self, fi: FormationIndex, count: u32) -> Info {
        let entry = &UNIT_TABLE[(self.stage as usize) % UNIT_TABLE.len()][self.unit as usize];
        let enemy_type = EnemyType::Fairy;
        match entry.pat {
            0 => {
                let side = count & 1;
                let time = (count / 2) * STEP_WAIT;
                Info::new(time, enemy_type, fi, Vector2D::new(16 * ONE, 0), side == 0, entry.table)
            }
            1 => {
                let time = count * STEP_WAIT;
                Info::new(
                    time,
                    enemy_type,
                    fi,
                    Vector2D::new(16 * ONE, 0),
                    entry.flip_x,
                    entry.table,
                )
            }
            2 => {
                let time = count * STEP_WAIT;
                Info::new(time, enemy_type, fi, Vector2D::new(0, 0), entry.flip_x, entry.table)
            }
            3 | _ => {
                let side = count & 1;
                let flag = 1 - (side as i32) * 2;
                let time = (count / 2) * STEP_WAIT;
                Info::new(
                    time,
                    enemy_type,
                    fi,
                    Vector2D::new(flag * 16 * ONE, 0),
                    entry.flip_x,
                    entry.table,
                )
            }
        }
    }
}
