# Monde et chunks

Représentation du terrain et de la dimension simulée.

## World

- Contient une ou plusieurs **dimensions** (`Overworld` en premier).
- Chaque dimension a :
  - une hauteur (min Y, max Y),
  - un `ChunkStore` (map chunk coords → chunk),
  - des règles (gamerules internes).

## Coordonnées

- Blocs : entiers `(x, y, z)`.
- Chunks : `(chunk_x, chunk_z)` = division entière par 16.
- Joueurs : `f64` pour position (interpolation client).

## Chunk

Structure cible (style Minecraft moderne) :

| Partie | Rôle |
|--------|------|
| Section 16×16×16 (ou hauteur section) | Palette de `BlockId` + indices |
| Biome (phase 2) | Simplifié au début |
| Entities (optionnel) | Souvent entités séparées dans ECS |

MVP : **monde plat** — une couche de bedrock + air, ou void plat avec spawn platform.

## BlockId

Toujours la clé **interne** `mcrust-registry::BlockId`.  
Pas de « metadata » legacy Java 1.12 ; les états sont des variantes de bloc si tu modélises les properties.

## Modifications

`set_block(pos, BlockId)` :

1. Met à jour le chunk (création chunk si absent).
2. Marque voisins pour mises à jour lumière (plus tard).
3. Enqueue `OutboundCommand::BlockChange` pour les observateurs.

Le core ne connaît pas qui est connecté : un système « block broadcast » résout les destinataires via `ChunkObserver`.

## Chargement et génération

```mermaid
flowchart LR
  R[Request chunk]
  Q[Queue génération]
  W[Worker async]
  C[ChunkReady event]
  R --> Q --> W --> C
  C --> Tick[Tick intègre chunk]
```

- Le tick demande des chunks autour des joueurs.
- Workers produisent des chunks (bruit, flat, vide).
- `InboundEvent::ChunkReady` ou ressource partagée verrouillée brièvement — **préférer event** pour garder le monde sur le thread tick.

## Sauvegarde (futur)

Format à définir : region-like ou snapshot par dimension.  
Hors scope P0–P7 ; garder l’API `World::load` / `save` en tête.

## Lumière et fluides

Phases avancées.  
MVP : plein jour fixe, pas d’eau/lave simulée.

## Dimensions Bedrock vs Java

Le core expose une **dimension interne** (`DimensionId`).  
Le bridge mappe vers les identifiants protocolaires (Java dimension codec, Bedrock dimension id).