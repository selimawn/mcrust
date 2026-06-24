# mcrust-core

Crate du **moteur de jeu** : monde, entités, règles, simulation.

## Responsabilités

| In scope | Out of scope |
|----------|----------------|
| Tick 20 TPS | TCP/UDP, RakNet |
| ECS / entités | Encodage paquets |
| Chunks et modifications | Auth (déléguée au bridge avant `PlayerJoin`) |
| Physique / collisions (phases) | Status ping JSON brut |
| Règles gamerules internes | Mapping runtime Bedrock |

## Structure interne (cible)

```
mcrust-core/
├── world/          # World, Dimension, ChunkStore
├── entity/         # Spawn, despawn, types
├── systems/        # Mouvement, chunk send, block tick
├── physics/        # AABB, gravité simplifiée
└── game_loop.rs    # Orchestration tick
```

## Entrées / sorties

**Entrée (chaque tick)** : drain de `Vec<InboundEvent>` (ou channel).

**Sortie** : `Vec<OutboundCommand>` ou channel ; le bridge consomme après le tick.

Le core ne bloque pas sur le réseau.

## État global

- `GameState` ou `ServerWorld` : contient `World`, `PlayerIndex`, `EntityWorld` (ECS).
- Identifiants joueur stables (`PlayerId`) distincts de `Entity` (plusieurs entités par joueur possible plus tard : véhicules).

## Dimensions

Au minimum une dimension `Overworld` (id interne).  
Nether/End = phases ultérieures avec mêmes abstractions.

## Plugins / hooks (futur)

Points d’extension sans casser les couches :

- callbacks après tick
- commandes admin internes

Pas requis pour P0–P7.

## Performance

- Pas d’allocation massive par tick si évitable (pools, réutilisation buffers côté bridge).
- Chunk loading async **hors** tick : le tick ne fait que consommer des chunks « prêts » (génération en tâche de fond + message `ChunkReady`).

Voir [tick.md](tick.md) pour le modèle async ↔ sync.