# rust-guessing-game-over-net
Popular Rust Guessing Game, over tcp connection


# Protocol 

Inspired by AMQP 0.9.1

## Protocol Header
Literal: "GG010"
|  G|G  |0 | 1 | 0
|--|--|--|--|--|
|  u8 |u8 |u8 |u8 |u8 |

## Frame format

| *FrameHeader* |*payload* |  frame-End|
|--|--|--|
| - | - | u8 |

## FrameHeader
|class| method|
|--|--|
| u8 | u8 |

## Payload

| size |content as bytes |
|--|--|
|  u8| [u8] |

## Class 1 - Connection
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
|u32|String|
| constraint size in bytes | Json[1] containing registration constraints  |

### Registration.Ok method payload
|size| user name |
|--|--|
|u8|String|
| user name size | User name |

### User method payload
|size| user |
|--|--|
|u32|String|
| user size in bytes | Json User |

**[1] this protocol will not use field-table-like implementation to describe complex data structure, the goal is to focus in others aspects of the program.**

### Basic interaction

 - Client sends: "Protocol Header" - Server Responds: "Registration" - Client sends: "Registration.Ok"
 - Server sends: "User"

