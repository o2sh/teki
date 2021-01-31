use crate::framework::components::*;
use crate::framework::resources::{EneShotSpawner, Formation, GameInfo};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use teki_common::game::{
    appearance_manager::Accessor as AppearanceManagerAccessor, traj::Accessor as TrajAccessor,
    AppearanceManager, EnemyType, FormationIndex, Traj,
};
use teki_common::utils::math::*;
use teki_common::utils::consts::*;
use vector2d::Vector2D;

const FAIRY_SPRITES: [&str; 4] = ["enemy0", "enemy1", "enemy2", "enemy3"];
const ANIMATION_SPAN: u32 = 10;

impl EnemyBase {
    pub fn new(traj: Option<Traj>) -> Self {
        Self { traj, attack_frame_count: 0 }
    }

    pub fn update_trajectory<A: EneBaseAccessorTrait>(
        &mut self,
        posture: &mut Posture,
        vel: &mut Speed,
        accessor: &mut A,
    ) -> bool {
        if let Some(traj) = &mut self.traj {
            let cont = traj.update(&*accessor.traj_accessor());
            posture.0 = traj.pos();
            posture.2 = traj.angle;
            vel.0 = traj.speed;
            vel.1 = traj.vangle;

            if cont {
                return true;
            }
            self.traj = None;
        }
        false
    }

    pub fn update_attack<A: EneBaseAccessorTrait>(
        &mut self,
        pos: &Vector2D<i32>,
        shot_enable: bool,
        accessor: &mut A,
    ) -> bool {
        self.attack_frame_count += 1;

        let stage_no = accessor.get_stage_no();
        let shot_count = std::cmp::min(2 + stage_no / 8, 5) as u32;
        let shot_interval = 20 - shot_count * 2;

        if self.attack_frame_count <= shot_interval * shot_count
            && self.attack_frame_count % shot_interval == 0
        {
            if shot_enable {
                accessor.fire_shot(pos);
            }
            true
        } else {
            false
        }
    }
}

struct SysAppearanceManagerAccessor<'a, 'b>(&'a mut SubWorld<'b>);

impl<'a, 'b> AppearanceManagerAccessor for SysAppearanceManagerAccessor<'a, 'b> {
    fn is_stationary(&self) -> bool {
        <&Enemy>::query().iter(self.0).all(|x| x.is_formation)
    }
}

pub struct EneBaseAccessorImpl<'l> {
    pub formation: &'l Formation,
    pub eneshot_spawner: &'l mut EneShotSpawner,
    pub stage_no: u16,
}

impl<'l> EneBaseAccessorImpl<'l> {
    pub fn new(
        formation: &'l Formation,
        eneshot_spawner: &'l mut EneShotSpawner,
        stage_no: u16,
    ) -> Self {
        Self { formation, eneshot_spawner, stage_no }
    }
}
pub trait EneBaseAccessorTrait {
    fn fire_shot(&mut self, pos: &Vector2D<i32>);
    fn traj_accessor<'a>(&'a mut self) -> Box<dyn TrajAccessor + 'a>;
    fn get_stage_no(&self) -> u16;
}

impl<'a> EneBaseAccessorTrait for EneBaseAccessorImpl<'a> {
    fn fire_shot(&mut self, pos: &Vector2D<i32>) {
        self.eneshot_spawner.push(pos);
    }

    fn traj_accessor<'b>(&'b mut self) -> Box<dyn TrajAccessor + 'b> {
        Box::new(TrajAccessorImpl { formation: self.formation, stage_no: self.stage_no })
    }

    fn get_stage_no(&self) -> u16 {
        self.stage_no
    }
}

struct TrajAccessorImpl<'a> {
    formation: &'a Formation,
    pub stage_no: u16,
}
impl<'a> TrajAccessor for TrajAccessorImpl<'a> {
    fn get_formation_pos(&self, formation_index: &FormationIndex) -> Vector2D<i32> {
        self.formation.pos(formation_index)
    }
    fn get_stage_no(&self) -> u16 {
        self.stage_no
    }
}

#[system]
#[read_component(Enemy)]
pub fn run_appearance_enemy(
    world: &mut SubWorld,
    #[resource] appearance_manager: &mut AppearanceManager,
    #[resource] enemy_formation: &mut Formation,
    commands: &mut CommandBuffer,
) {
    if appearance_manager.done {
        return;
    }

    let accessor = SysAppearanceManagerAccessor(world);
    let new_borns_opt = appearance_manager.update(&accessor);

    if let Some(new_borns) = new_borns_opt {
        new_borns.into_iter().for_each(|e| {
            let sprite_name = match e.enemy_type {
                EnemyType::Fairy => "enemy0",
            };

            let base = EnemyBase::new(Some(e.traj));
            let enemy = Enemy {
                enemy_type: e.enemy_type,
                formation_index: e.fi,
                index_x: 0,
                state: EnemyState::Appearance,
                base,
                is_formation: false,
            };
            let posture = Posture(e.pos, 0, 0);
            let speed = Speed(0, 0);
            let hit_box = HitBox { size: Vector2D::new(32, 32) };
            let drawable = SpriteDrawable { sprite_name, offset: Vector2D::new(-16, -16) };
            commands.push((enemy, posture, speed, hit_box, drawable));
        });
    }

    if appearance_manager.done {
        enemy_formation.done_appearance = true;
    }
}

#[system(for_each)]
#[write_component(Posture)]
pub fn move_enemy(
    enemy: &mut Enemy,
    speed: &mut Speed,
    entity: &Entity,
    world: &mut SubWorld,
    #[resource] enemy_formation: &mut Formation,
    #[resource] eneshot_spawner: &mut EneShotSpawner,
    #[resource] game_info: &mut GameInfo,
) {
    do_move_enemy(*entity, enemy, speed, enemy_formation, game_info, eneshot_spawner, world)
}

fn do_move_enemy(
    entity: Entity,
    enemy: &mut Enemy,
    speed: &mut Speed,
    enemy_formation: &Formation,
    game_info: &mut GameInfo,
    eneshot_spawner: &mut EneShotSpawner,
    world: &mut SubWorld,
) {
    match enemy.state {
        EnemyState::Appearance => {
            let mut accessor =
                EneBaseAccessorImpl::new(enemy_formation, eneshot_spawner, game_info.stage);
            let posture = <&mut Posture>::query().get_mut(world, entity).unwrap();
            if !enemy.base.update_trajectory(posture, speed, &mut accessor) {
                enemy.base.traj = None;
                enemy.state = EnemyState::MoveToFormation;
            }
        }
        EnemyState::MoveToFormation => {
            let posture = <&mut Posture>::query().get_mut(world, entity).unwrap();
            let result = move_to_formation(posture, speed, &enemy.formation_index, enemy_formation);
            forward(posture, speed);
            if result {
                enemy.state = EnemyState::Formation;
                enemy.is_formation = true;
            }
        }
        EnemyState::Formation => {
            let posture = <&mut Posture>::query().get_mut(world, entity).unwrap();
            let ang = ANGLE * ONE / 128;
            posture.1 -= clamp(posture.1, -ang, ang);
            enemy.state = EnemyState::Attack(AttackType::Normal);
        }
        EnemyState::Attack(_) => {
            let mut accessor =
                EneBaseAccessorImpl::new(enemy_formation, eneshot_spawner, game_info.stage);
            let posture = <&mut Posture>::query().get_mut(world, entity).unwrap();
            enemy.base.update_attack(&posture.0, true, &mut accessor);
        }
    }
}

#[system(for_each)]
pub fn animate_enemy(
    enemy: &mut Enemy,
    sprite: &mut SpriteDrawable,
    #[resource] game_info: &mut GameInfo,
) {
    do_animate_enemy(enemy, sprite, game_info.frame_count);
}

pub fn do_animate_enemy(enemy: &mut Enemy, sprite: &mut SpriteDrawable, frame_count: u32) {
    if frame_count % ANIMATION_SPAN == 0 {
        enemy.index_x += 1;
        if enemy.index_x > 3 {
            enemy.index_x = 0;
        }
        sprite.sprite_name = match enemy.enemy_type {
            EnemyType::Fairy => FAIRY_SPRITES[enemy.index_x],
        };
    }
}

pub fn forward(posture: &mut Posture, speed: &Speed) {
    posture.0 += calc_velocity(posture.2 + speed.1 / 2, speed.0);
    posture.2 += speed.1;
}

pub fn move_to_formation(
    posture: &mut Posture,
    speed: &mut Speed,
    fi: &FormationIndex,
    formation: &Formation,
) -> bool {
    let target = formation.pos(fi);
    let pos = &mut posture.0;
    let angle = &mut posture.2;
    let spd = &mut speed.0;
    let vangle = &mut speed.1;
    let diff = &target - &pos;
    let sq_distance = square(diff.x >> (ONE_BIT / 2)) + square(diff.y >> (ONE_BIT / 2));
    if sq_distance > square(*spd >> (ONE_BIT / 2)) {
        let dlimit: i32 = *spd * 5 / 3;
        let target_angle = atan2_lut(-diff.y, diff.x);
        let d = diff_angle(target_angle, *angle);
        *angle += clamp(d, -dlimit, dlimit);
        *vangle = 0;
        false
    } else {
        *pos = target;
        *spd = 0;
        *angle = normalize_angle(*angle);
        *vangle = 0;
        true
    }
}


#[system]
#[read_component(Player)]
#[read_component(Posture)]
#[read_component(EneShot)]
pub fn spawn_eneshot(world: &SubWorld, #[resource] eneshot_spawner: &mut EneShotSpawner, #[resource] game_info: &GameInfo, commands: &mut CommandBuffer) {
    eneshot_spawner.update(game_info, world, commands);
}

#[system(for_each)]
pub fn move_eneshot(shot: &mut EneShot, posture: &mut Posture, entity: &Entity, commands: &mut CommandBuffer) {
    do_move_eneshot(shot, posture, *entity, commands);
}


pub fn do_move_eneshot(shot: &EneShot, posture: &mut Posture, entity: Entity, commands: &mut CommandBuffer) {
    posture.0 += shot.0;
    if out_of_screen(&posture.0) {
        commands.remove(entity);
    }
}

fn out_of_screen(pos: &Vector2D<i32>) -> bool {
    pos.x < -16 * ONE || pos.x > (GAME_WIDTH + 16) * ONE ||
        pos.y < -16 * ONE || pos.y > (GAME_HEIGHT + 16) * ONE
}
