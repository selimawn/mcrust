# ECS (Entity Component System)

Le moteur représente tout objet simulé comme **entité** + **composants** + **systèmes** qui les parcourent.

## Choix technique

Candidats Rust :

- **bevy_ecs** (sans bevy renderer) — écosystème riche, queries typées.
- **hecs** — plus minimal.

Décision à figer au premier `cargo new mcrust-core` ; l’architecture reste la même.

## Entités

`Entity` = identifiant opaque (u64 ou wrapper).  
Recycle des IDs après despawn pour éviter la saturation (pool).

## Composants (première vague)

| Composant | Données |
|-----------|---------|
| `Transform` | position (f64 ou fixed), yaw, pitch |
| `Velocity` | dx, dy, dz |
| `OnGround` | bool |
| `Player` | `PlayerId`, gamemode interne |
| `EntityType` | mob, item, projectile — phase ultérieure |
| `ChunkObserver` | rayon de vue, dimension |
| `NetworkSync` | dirty flags pour broadcast |

Pas de logique dans les composants — données pures.

## Systèmes

Fonctions ou `System` trait qui :

- query `(&mut Transform, &Velocity, &Player)`
- lisent les ressources globales (`World`, `TickConfig`)

Exécution **séquentielle** dans le tick (pas de parallèle sur le même `World` au début).

## Joueur = entité

À `PlayerJoin`, le core :

1. Alloue `PlayerId`.
2. Spawn `Entity` avec `Player`, `Transform`, `ChunkObserver`, etc.
3. Émet `OutboundCommand::SpawnPlayer` / équivalent pour les autres clients.

Voir [player.md](player.md).

## Relation monde ↔ ECS

Les **blocs** ne sont en général **pas** une entité par bloc (trop lourd).  
Ils vivent dans `Chunk` (palette + sections).  
Les entités mobiles (joueurs, mobs) sont dans l’ECS.

Exceptions futures : entités bloc (lit, coffre) si tu modélises l’interaction comme entité.

## Nettoyage

`PlayerLeave` → despawn entité + retrait des index `PlayerId → Entity`.