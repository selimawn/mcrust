# P6 Bedrock — implémentation

## Fait

- **RakNet** : `rust-raknet` (`RaknetListener`, sessions `RaknetSocket`)
- **Flux** : `RequestNetworkSettings` → `NetworkSettings` → `Login` → `ServerToClientHandshake` → `ClientToServerHandshake` → resource packs vides → `StartGame` + `PlayStatus` + `SetLocalPlayerAsInitialised`
- **Auth** : parsing chain JWT + `extraData` ; `bedrock-online-mode=true` exige chaîne **≥ 3** liens (Xbox) ; offline accepte chaîne **1** lien (self-signed)
- **Core** : `PlayerJoin` Bedrock, mouvement `PlayerAuthInput` (id 0x94) → `BroadcastMovement` → `MovePlayer`
- Protocoles annoncés : **685–688** (ajuster selon client 1.21.x)

## Limites connues (à durcir)

| Sujet | État |
|--------|------|
| Vérification **cryptographique** JWT (ES384, clé Mojang) | Partielle — parsing claims, pas de vérif signature complète |
| Handshake **ECDH** chiffrement paquets | JWT handshake **offline stub** |
| `StartGame` | Encodeur **minimal** — champs récents 1.21 peuvent manquer → crash client si version stricte |
| Resource packs | Stack vide (OK pour serveur custom) |

## Fichiers

- `crates/mcrust-bedrock/src/raknet_server.rs` — listener
- `session.rs` — machine à états
- `auth.rs` — chain / identity
- `packets.rs` / `codec.rs` — batch + gamepackets

## Test

`cargo run -p mcrust-server` avec `enable-bedrock=true`, `bedrock-online-mode=false` pour LAN ; client Bedrock **1.21.x** sur `:19132`.

Cross-play : 1 Java + 1 Bedrock dans le même monde via `mcrust-core` (positions broadcast).