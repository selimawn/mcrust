# mcrust

Serveur Minecraft **cross-play** (Java + Bedrock) écrit en Rust, sans proxy Geyser/Floodgate.

## Documentation

| Dossier | Contenu |
|---------|---------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Vue d’ensemble, couches, flux de données |
| [docs/architecture/](docs/architecture/) | Cœur : joueur, monde, tick, ECS |
| [docs/network/](docs/network/) | Java (TCP), Bedrock (UDP/RakNet), bridge |

## Objectifs

- **20 TPS** : boucle de jeu déterministe, isolée du réseau async.
- **Core agnostique** : le moteur ne connaît ni Java ni Bedrock.
- **Compatibilité native** : deux frontends réseau, un registre et un monde unifiés.

## Références protocole

- Java : [wiki.vg — Protocol](https://wiki.vg/Protocol)
- Bedrock : [Mojang/bedrock-protocol-docs](https://github.com/Mojang/bedrock-protocol-docs)

## État du projet

Phase **plan / fondations** — voir la roadmap dans [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).