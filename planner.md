# Work Planner

This is not for the assignment in any way, just to plan my path forward and important resources I might use later.

## Database

Need to come up with architecture, will probably use a relational database like postgress or something.

Have to think about how to store a chess game efficiently,

### database api planning:

**User Management**

POST /api/users - Register a new user
GET /api/users/:id - Get user profile
PUT /api/users/:id - Update user profile
GET /api/users/:id/stats - Get user statistics

**Game Management**

POST /api/games - Create a new game
GET /api/games/:id - Get game details
PUT /api/games/:id - Update game (e.g., result)
GET /api/games - List games with filters

**Move Management**

POST /api/games/:id/moves - Add a move to a game
GET /api/games/:id/moves - Get all moves for a game
GET /api/games/:id/pgn - Export game in PGN format

**Search and Analytics**

GET /api/openings - Search for games by opening
GET /api/users/:id/games - Get games for a specific user
GET /api/analysis/:id - Get analysis for a game

## frontend

i need to add a login system, since I plan to deploy this on the web. I also want to integate users into the database to store their games and stats.

actually i might not use a relational database as it is overkill for this project, i might just use a json file to store the games and stats. this can then be deployed into a mongodb database or something of the sort later.

found a usefull stackoverflow article on how to store a chess game efficiently:

[stackoverflow](https://stackoverflow.com/questions/76429164/how-to-store-a-chess-game-efficiently)
o

## backend

i need to figure out how to strucutre the REST API, i found a samp.e one online that i might base mine off of

**request**

URI: https://stockfish.online/api/s/v2.php
FEN: rn1q1rk1/pp2b1pp/2p2n2/3p1pB1/3P4/1QP2N2/PP1N1PPP/R4RK1 b - - 1 11
Depth: 20

**response**
{"success":true,"evaluation":0.97,"mate":null,"bestmove":"bestmove b7b6 ponder a1e1","continuation":"b7b6 a1e1 f6e4 g5e7 d8e7 c3c4 e7d6 c4d5 e4d2 f3d2 c6d5"}
