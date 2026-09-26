use bevy::{ecs::schedule::ScheduleLabel, prelude::*};


#[derive(Message, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RequestNextTick;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tick;
impl Tick {
    
    pub fn run(world: &mut World) {
        world.run_schedule(PreTick);
        world.run_schedule(DesireDeclarationTick);
        world.run_schedule(DesireExecutionTick);
        world.run_schedule(PostTick);
    }

}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PreTick;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PostTick;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DesireDeclarationTick;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DesireExecutionTick;



#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TickManagement;
impl Plugin for TickManagement {
    fn build(&self, app: &mut App) {
        app
        .add_message::<RequestNextTick>();
    }
}