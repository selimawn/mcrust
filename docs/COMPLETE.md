# mcrust — état « complet » (jalons P5–P7 + durcissement)

## Java Edition

| Fonctionnalité | Implémentation |
|----------------|----------------|
| Status (multi-proto 767/768/769) | `mcrust-java/server.rs` |
| Login offline | UUID offline + compression |
| Login online | RSA 1024, AES-CFB8, `hasJoined` |
| Configuration | `registry_data` minimal + `finish_configuration` + attente `login_acknowledged` |
| Play join | Paquet `login` 0x2b (SpawnInfo), teleport 0x40 |
| Keep-alive / mouvement | VarLong keep-alive, position 0x1a/0x1b |
| Core + bridge | `mcrust-core`, `mcrust-bridge` |

Réf. : [network/auth-java.md](network/auth-java.md), [network/protocol-alignment.md](network/protocol-alignment.md)

## Bedrock Edition

| Fonctionnalité | Implémentation |
|----------------|----------------|
| RakNet | `rust-raknet` |
| NetworkSettings | Compression threshold |
| Login JWT | `jwt_auth.rs` — chaîne 1/3, client data JWT |
| Handshake ECDH | `ecdh.rs` — JWT ES384 + salt, clé SHA256(salt‖shared) |
| Chiffrement batches | AES-256-CTR (gophertunnel-compatible) |
| Resource packs | Info + stack vides |
| StartGame | `start_game.rs` (PMMP field order) |
| Play | Spawn + `SetLocalPlayerAsInitialized` + `PlayerAuthInput` |

Réf. : [network/auth-bedrock.md](network/auth-bedrock.md), [network/P6-bedrock-wip.md](network/P6-bedrock-wip.md)

## Configuration

`conf.txt` — `online-mode`, `bedrock-online-mode`, ports, MOTD.

## Limites connues (production vanilla stricte)

- Java : registres Configuration **minimaux** — clients très récents peuvent exiger plus de `registry_data` / feature flags.
- Bedrock : **OIDC Token** multiplayer (post-1.26) non géré ; handshake production peut exiger clés signées côté client selon version.
- Monde : pas de chunks palette vanilla complets ni générateur — spawn créatif vide.
- `base_game_version` Bedrock : ajuster dans `session.rs` / `start_game` selon client.

## Lancer

```bash
cargo run -p mcrust-server
# conf.txt : online-mode=false et bedrock-online-mode=false pour tests LAN
```