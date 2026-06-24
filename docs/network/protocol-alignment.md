# Alignement protocole (Java 1.21.1 / Bedrock)

## Java — protocol **767** (minecraft-data 1.21.1)

Source : `crates/mcrust-java/src/protocol_ids.rs`

| État | Paquet | ID |
|------|--------|-----|
| Login C | disconnect | 0x00 |
| Login C | encryption_begin | 0x01 |
| Login C | login_success | 0x02 |
| Login C | set_compression | 0x03 |
| Login S | login_start | 0x00 |
| Login S | encryption_begin | 0x01 |
| Config C | finish_configuration | 0x03 |
| Play C | login (join) | 0x2b |
| Play C | keep_alive | 0x26 |
| Play C | player_position (teleport) | 0x40 |
| Play S | keep_alive | 0x18 |
| Play S | position / position_look | 0x1a / 0x1b |

Auth : [auth-java.md](auth-java.md) — `hasJoined`, hash SHA-1 style Minecraft (`auth.rs`).

## Bedrock

| Étape | Paquet ID (décimal) |
|-------|---------------------|
| RequestNetworkSettings | 193 (0xc1) |
| NetworkSettings | 143 (0x8f) |
| Login | 1 |
| ServerToClientHandshake | 3 |
| StartGame | 11 |
| SetLocalPlayerAsInitialized | 113 (0x71) |

Auth : [auth-bedrock.md](auth-bedrock.md) — `jwt_auth.rs` (chaîne 3 + Mojang, offline chaîne 1).