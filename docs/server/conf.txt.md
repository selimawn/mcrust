# Configuration — `conf.txt`

Fichier principal du serveur, **à la racine du répertoire de travail** (ou chemin passé en argument au binaire). Syntaxe inspirée de `server.properties` (Paper/Spigot) : une clé par ligne, `clé=valeur`, `#` pour les commentaires.

Chargement au démarrage ; certaines clés nécessitent un redémarrage (indiqué ci-dessous).

## Général

| Clé | Défaut | Description |
|-----|--------|-------------|
| `server-name` | mcrust | Nom affiché (MOTD partiel) |
| `motd` | A mcrust server | Description liste serveur (Java JSON / Bedrock MOTD) |
| `max-players` | 20 | Joueurs simultanés max |
| `view-distance` | 10 | Rayon chunks (simulation + envoi) |
| `simulation-distance` | 10 | Rayon simulation (≤ view-distance) |
| `gamemode` | survival | `survival`, `creative`, `adventure`, `spectator` (défaut join) |
| `force-gamemode` | false | Forcer le gamemode à la connexion |
| `difficulty` | easy | `peaceful`, `easy`, `normal`, `hard` |
| `hardcore` | false | Ban / spec après mort (comportement à définir en implémentation) |
| `pvp` | true | PvP autorisé |
| `spawn-protection` | 16 | Rayon protection spawn (0 = off) |
| `white-list` | false | Whitelist active |
| `enforce-whitelist` | false | Kick si pas sur whitelist |
| `player-idle-timeout` | 0 | Minutes avant kick idle (0 = désactivé) |

## Réseau — Java

| Clé | Défaut | Description |
|-----|--------|-------------|
| `server-ip` | | IP bind (vide = toutes interfaces) |
| `server-port` | 25565 | Port TCP Java |
| `online-mode` | true | Auth Mojang officielle |
| `prevent-proxy-connections` | false | IP dans `hasJoined` |
| `network-compression-threshold` | 256 | Seuil zlib (-1 = off) |
| `max-tick-time` | 60000 | Watchdog ms (0 = off) |

## Réseau — Bedrock

| Clé | Défaut | Description |
|-----|--------|-------------|
| `enable-bedrock` | true | Écoute UDP Bedrock |
| `bedrock-port` | 19132 | Port UDP |
| `bedrock-online-mode` | true | Auth JWT/Xbox officielle |
| `bedrock-verify-server-address` | true | Vérifier ServerAddress client |
| `server-guid` | (généré) | GUID Unconnected Pong (stable si fixé) |

## Monde

| Clé | Défaut | Description |
|-----|--------|-------------|
| `level-name` | world | Dossier monde |
| `level-seed` | | Seed (vide = aléatoire) |
| `level-type` | default | `default`, `flat`, `void` (selon implémentation) |
| `generate-structures` | true | Structures |
| `allow-nether` | true | Nether activé (phase ultérieure) |
| `spawn-monsters` | true | Spawn mobs hostiles |
| `spawn-animals` | true | Animaux |
| `spawn-npcs` | true | Villageois |

## Performance

| Clé | Défaut | Description |
|-----|--------|-------------|
| `target-tps` | 20 | TPS cible (ne pas monter sans changer la boucle) |
| `max-chunk-send-per-tick` | 4 | Limite envoi chunks / tick / joueur |
| `network-queue-limit` | 1024 | Paquets sortants max par session avant throttle/kick |

## Sécurité / ops

| Clé | Défaut | Description |
|-----|--------|-------------|
| `op-permission-level` | 4 | Niveau OP minimal |
| `enable-command-block` | false | Blocs commande |
| `broadcast-console-to-ops` | true | Log console → ops |

## Versions protocole

| Clé | Défaut | Description |
|-----|--------|-------------|
| `java-supported-versions` | (liste interne) | Plage de versions Java négociées au handshake (voir DECISIONS D-002) |
| `bedrock-supported-protocols` | (liste interne) | Deux patchs 1.21.x — entiers protocole (DECISIONS D-003) |

## Fichiers liés

| Fichier | Rôle |
|---------|------|
| `whitelist.json` | Liste UUID/noms si whitelist |
| `ops.json` | Opérateurs |
| `banned-players.json` | Bans |
| `banned-ips.json` | Ban IP |

## Exemple

Voir [`conf.txt.example`](../../conf.txt.example) à la racine du dépôt.

## Implémentation (`mcrust-server`)

- Parser simple : trim, ignorer lignes vides, pas de quotes obligatoires
- Types : bool (`true`/`false`), i32, u16, string
- Erreur au démarrage si clé inconnue (mode strict) ou warn (mode permissif — choisir strict pour éviter les fautes de frappe)

Les modules `mcrust-java` et `mcrust-bedrock` reçoivent une struct `ServerConfig` partagée, pas de lecture fichier directe dans les crates protocole.