/// the source of the log message
/// this is used to identify where the log message came from
/// and can be used to filter log messages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogSource {
    Inventory,
    Enemy,
    Input,
    Player,
    Creature,
    Companion,
    NPC,
    Farm,
    World,
    Interface,
    State,
    Quest,
    Time,
    Ldtk,
}
