# Architecture jeu (cœur)

Documentation du **moteur** : tout ce qui tourne à 20 TPS sans connaître Java ni Bedrock.

## Fichiers

| Fichier | Sujet |
|---------|--------|
| [core.md](core.md) | Crate `mcrust-core`, responsabilités, frontières |
| [tick.md](tick.md) | Boucle 50 ms, ordre des systèmes, lag |
| [ecs.md](ecs.md) | Entités, composants, systèmes |
| [player.md](player.md) | **Player unifié** Java+Bedrock, champ `platform` |
| [world.md](world.md) | Dimensions, chunks, blocs, lumière (phases) |

## Règle d’or

Le core consomme et produit uniquement des types de **`mcrust-protocol`** et **`mcrust-registry`**.

Interdit dans `mcrust-core` :

- `packet_id` Java ou Bedrock
- Appels tokio / sockets
- JSON de status ou MOTD Bedrock

## Lien avec le réseau

Voir [../network/bridge.md](../network/bridge.md) pour les files et sessions.  
Vue globale : [../ARCHITECTURE.md](../ARCHITECTURE.md).