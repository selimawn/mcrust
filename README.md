# mcrust

Serveur Minecraft **cross-play** (Java + Bedrock) écrit en Rust, sans proxy Geyser/Floodgate.

## Documentation

| Dossier | Contenu |
|---------|---------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Vue d’ensemble, couches, flux de données |
| [docs/architecture/](docs/architecture/) | Cœur : joueur, monde, tick, ECS |
| [docs/network/](docs/network/) | Java, Bedrock, bridge, auth officielle |
| [docs/server/conf.txt.md](docs/server/conf.txt.md) | Paramètres serveur (`conf.txt`) |
| [conf.txt.example](conf.txt.example) | Exemple de configuration |

## Objectifs

- **20 TPS** : boucle de jeu déterministe, isolée du réseau async.
- **Core agnostique** : le moteur ne connaît ni Java ni Bedrock.
- **Compatibilité native** : deux frontends réseau, un registre et un monde unifiés.

## Références protocole

- Java : [wiki.vg — Protocol](https://wiki.vg/Protocol)
- Bedrock : [Mojang/bedrock-protocol-docs](https://github.com/Mojang/bedrock-protocol-docs)

## Crédits

Projet **mcrust** — https://github.com/selimawn/mcrust  
Les œuvres dérivées ou renommées doivent créditer ce projet en tête de leur documentation (voir [LICENSE](LICENSE)).

## État du projet

Workspace Rust : `mcrust-wire`, `mcrust-protocol`, `mcrust-server`. Voir [docs/DECISIONS.md](docs/DECISIONS.md) et [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).