Noggin night Web server that serves action bucket as a game

Rust based bevy game that can be compiled into wasm and served online. Should also consider godot but honestly bevy felt like I could iterate quicker but Godot definitely has a better GUI for UI and other similar tasks but bevy has a better coding heavy workflow style

Consider both points here

Make it so lobbies are self hosted via someone's internet

Make it so a lobby has a "match /in game" which consists of 1-4 players with a set of match configurations settable in the main menu

The lobby should also support (secondary goal) a set of spectators or should be built with that idea in mind. Spectators should be able to view a match but not be displayed in game

Lobby should support voice proximity chat because that would be really funny

Action bucket should be a 2d platformer with a bunch of buckets. Basically imagine brawl and rivals of ether but its a bunch of different buckets

Support for items and different stages should be kept in mind but are not important to the MVP. 

Lobby Configurations should be the following 
- time or stock end conditions with support built in case we add more types
- number of players
- which players are in the lobby
- which players are in the match in game
- support for future conditions or configurations should be considered at time of architecture 
- 
# Design
## MVP 
- load into main menu
- load into lobby or create new lobby. Note : this lobby should be accessible only with password and should follow all best network practices in terms of ports. This is going to be the unfortunate part of I don't host BUT gives everyone else the ability to host through the power of wasm. 
- choose character and stage
- start match
- Match continues until you either run out of time or lives
- lobby redirected to win screen and load back to lobby when all in game players ready up (might want to rethink this)

## stretch goals
- in game proximity chat with lobby being set in the "middle" of the stage and audience being set a bit further back automatically. If nothing else at least put in the middle if the further way/muffled effect of distance is not easily achievable 
- 
