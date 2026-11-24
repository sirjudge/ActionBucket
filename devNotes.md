## **11/21/2026 8:45:00 pm EST**
started initial design of how lobby and player architecture should go. This is the first iteration of this design and serves as a historic point of a larger design, Yippee! 

I think this diagram should be completed BEFORE I continue with the in game and lobby plugin development. Once this document's v1 draft goes to v1 final the current code should be re-evaluated for correctness to this diagram. The current implementation 

## 11/22/2026 9:33:27 am EST
I've finished the V1 architecture diagram to cover most of the interactions I expect in the game. The player input local vs remote is still rocky and should be considered "experimental" while I keep thinking about the best way to do it. 

At this point I've decided to separate the in match plugin and the player input plugins since they do two different tasks. The player input plugin will keep track of the local players (up to 8 local players supported) and will keep track of the player and their respective controller/keyboard device ID. This plugin will additionally be responsible for syncing the remote players (either lobby host or other remote players, doesn't matter in this case). 

When player input is received either locally or remote, both inputs will be matched to the correct fighter in the match and the input event either passed along to the in game plugin or handled specifically within the player input plugin. This part is still up in the air as implementation difficulty and MVP iteration will help determine how this exactly plays out.

## 11/24/2026 2:12:43 pm EST
Today's task was finishing the implementation of adding and disconnecting a player from a lobby based on a player connection message firing.

Still a bit up in the air about how to best go about handling player vs CPU but moved that decision out of Fighter and onto Player struct for now. 

Additionally, worked on getting unit tests started for game_lobby and player as well as starting to increase test coverage on the fighter struct.
