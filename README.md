# rust-guessing-game-over-net
Popular Rust Guessing Game, over tcp connection


# Protocol

## Protocol Header
Literal: "GG010"
|  G|G  |0 | 1 | 0
|--|--|--|--|--|
|  u8 |u8 |u8 |u8 |u8 |

## Frame format

|class| method |*payload* |  frame-End|
|--|--|--|--|
| u8 | u8 | - | u8 |

## Payload

| size |content  |
|--|--|
|  u8| [u8] |

## Class 1 - Server Commands
C - received from client
S - send to client

|  class|method|peer |name | description
|--|--|--|--|--|
| 1 | 1 | S | Registration |Signals the client that it can start the user registration|
|1|2|C|Registration.Ok|Send user data registration|
|1|3|S|User|New user data|

### Registration method payload
|size| constraints |
|--|--|
|u32|[u8]|
| constraint size in bytes | Json[1] as bytes, containing registration constraints  |

### Registration.Ok method payload
|size| user name |
|--|--|
|u8|[u8]|
| user name size | User name as bytes |

### User method payload
|size| user |
|--|--|
|u32|[u8]|
| user size in bytes | Json User as bytes |

**[1] this protocol will not use field-table-like implementation to describe complex data structure, the goal is to focus in others aspects of the program.**

### Basic interaction

 - Client sends: "Protocol Header" - Server Responds: "Registration" - Client sends: "Registration.Ok"
 - Server sends: "User"

