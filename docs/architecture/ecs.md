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
| `PlayerRef` | `PlayerId` — lien vers l’objet [Player](player.md) unifié |
| `EntityType` | mob, item, projectile — phase ultérieure |
| `ChunkObserver` | rayon de vue, dimension |
| `NetworkSync` | dirty flags pour broadcast |

Pas de logique dans les composants — données pures.  
**Pas** de composants séparés `JavaPlayer` / `BedrockPlayer` : `platform` est sur `Player`.

## Systèmes

Fonctions ou `System` trait qui :

- query `(&mut Transform, &Velocity, &PlayerRef)`
- lisent les ressources globales (`World`, `PlayerIndex`, `TickConfig`)

Exécution **séquentielle** dans le tick (pas de parallèle sur le même `World` au début).

## Joueur = entité + Player

À `PlayerJoin` :

1. Insérer `Player` dans `PlayerIndex` (avec `platform`, `uuid`, `xuid`, …).
2. Spawn `Entity` avec `PlayerRef`, `Transform`, `ChunkObserver`, etc.
3. Lier `player.entity = Some(entity)`.
4. Émettre les `OutboundCommand` de visibilité.

## Relation monde ↔ ECS

Les **blocs** ne sont en général **pas** une entité par bloc (trop lourd).  
Ils vivent dans `Chunk` (palette + sections).  
Les entités mobiles (joueurs, mobs) sont dans l’ECS.

## Nettoyage

`PlayerLeave` → despawn entité + retrait `PlayerIndex`.