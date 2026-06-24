# P6 Bedrock — état actuel

## Fait

- Unconnected ping/pong (`mcrust-bedrock/src/ping.rs`)
- UDP hybride `run_bedrock_hybrid` (ping + stub expérimental)

## À faire pour P6 complet

1. Intégrer **RakNet** connecté (`rust-raknet` / `tokio-raknet`) derrière façade
2. `NetworkSettings` + paquet `Login` (chain JWT)
3. Auth officielle : [auth-bedrock.md](auth-bedrock.md)
4. Handshake ECDH + `StartGame` / spawn Bedrock
5. Encoder mouvement Bedrock depuis `OutboundCommand` (comme Java)

Jusqu’à P6 complet, les clients Bedrock **ne peuvent pas rejoindre le monde** comme en Java ; le cross-play réel = **Java + Java** ou attendre P6.