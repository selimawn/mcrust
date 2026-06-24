# Réseau mcrust

Cette section documente tout ce qui touche aux **clients** : protocoles Java et Bedrock, transport, et le **bridge** qui relie le réseau au moteur.

## Fichiers

| Fichier | Sujet |
|---------|--------|
| [java.md](java.md) | TCP, machine d’états, paquets, chiffrement |
| [bedrock.md](bedrock.md) | UDP, RakNet, GamePackets, cycle de vie |
| [bridge.md](bridge.md) | Sessions, files, mapping, erreurs |
| [auth-java.md](auth-java.md) | Online-mode Mojang (`hasJoined`) |
| [auth-bedrock.md](auth-bedrock.md) | JWT / Xbox Live officiel |

## Ports par défaut

| Plateforme | Protocole | Port |
|------------|-----------|------|
| Java | TCP | 25565 |
| Bedrock | UDP | 19132 |

## Principe commun

Les deux frontends partagent le même modèle mental :

1. **Lire** des octets depuis le socket.
2. **Décoder** en structure protocole (IDs, champs).
3. **Traduire** en `InboundEvent` (`mcrust-protocol`).
4. À l’inverse : `OutboundCommand` → encoder → écrire.

Le **core** ne voit jamais les octets ni les IDs de paquets plateforme.

## Tokio

- `TcpListener` + une tâche par connexion Java.
- `UdpSocket` + état RakNet (souvent une tâche ou pool par endpoint logique).
- Écriture async avec backpressure (limiter la taille des files par joueur).

## Jalons réseau

1. **Découverte** : Java Status + Bedrock Unconnected Ping (serveur « en ligne »).
2. **Auth** : login (offline puis Mojang / Xbox).
3. **Play** : join game, chunks, entités, chat, déconnexion propre.