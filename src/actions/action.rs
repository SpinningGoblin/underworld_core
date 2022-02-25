use super::{
    look_at::{LookAtRoom, LookAtTarget},
    quick_look::QuickLookRoom,
};

pub enum Action {
    LookAtTarget(LookAtTarget),
    LookAtRoom(LookAtRoom),
    QuickLookRoom(QuickLookRoom),
}
