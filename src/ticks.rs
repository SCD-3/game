use bevy::{ecs::schedule::ScheduleLabel, prelude::*};


#[derive(Message, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RequestNextTick;
impl RequestNextTick {
    fn eat_request(mut requests: MessageReader<Self>) {
        requests.read();
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tick;
impl Tick {
    
    fn run(world: &mut World) {        
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
        .init_schedule(Tick)
        .init_schedule(PreTick)
        .init_schedule(PostTick)
        .init_schedule(DesireDeclarationTick)
        .init_schedule(DesireExecutionTick)
        
        .add_message::<RequestNextTick>()
        .add_systems(Update, Tick::run
            .run_if(on_message::<RequestNextTick>)
        )
        .add_systems(Update, RequestNextTick::eat_request
            .run_if(on_message::<RequestNextTick>)
            .after(Tick::run)
        );
    }
}