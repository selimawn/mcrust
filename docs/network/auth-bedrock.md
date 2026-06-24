# Authentification Bedrock (officielle / Xbox Live)

Références : [Mojang/bedrock-protocol-docs](https://github.com/Mojang/bedrock-protocol-docs), [Bedrock Edition protocol — Login](https://minecraft.wiki/w/Bedrock_Edition_protocol), implémentations de référence (gophertunnel, Geyser, PMMP).

Bedrock **n’utilise pas** `hasJoined` Java. L’identité est prouvée par **JWT** (chaîne Xbox/Mojang et/ou token multiplayer OIDC).

## Paramètres `conf.txt`

| Clé | Effet |
|-----|--------|
| `bedrock-online-mode` | `true` → vérification cryptographique obligatoire ; `false` → LAN / dev (chaîne self-signed, risque sécurité) |
| `bedrock-verify-server-address` | Si `true`, valider `ServerAddress` dans client data JWT vs host/port annoncé |

## Paquet Login (connection request)

Après RakNet + éventuellement `Network Settings` :

1. Version protocole (BE)
2. Blob **connection request** :
   - longueur + **chain JSON** (UTF-8)
   - longueur + **client data JWT** (skin, device, `ServerAddress`, …)

### Enveloppe chain (moderne)

```json
{
  "Certificate": { "chain": [ "<JWT>", "<JWT>", ... ] },
  "AuthenticationType": 0,
  "Token": "<OIDC JWT ou vide>"
}
```

Legacy possible : `{ "chain": [ ... ] }` seul.

## Chemin A — Chaîne JWT legacy (3 liens, online)

| Étape | Vérification |
|-------|----------------|
| JWT 0 | Clé dans header `x5u` ; claims valides |
| JWT 1 | Signé par clé dérivée ; `iss` = `"Mojang"` ; fenêtre temporelle |
| JWT 2 | `extraData` (displayName, identity UUID, **XUID**, titleId, …) + `identityPublicKey` |
| Racine Mojang | Clé ECDSA publique Mojang (PKIX connue) — sinon rejet |

**Règle** : en online-mode, chaîne longueur **1** (self-signed) = **refus** sauf `bedrock-online-mode=false`.

Identité **de confiance** : uniquement `extraData` / claims vérifiés — **pas** le username dans client data seul.

## Chemin B — Token multiplayer OIDC (récent)

Si `Token` non vide :

- Vérifier JWT OIDC (JWKS / clés configurées, ES384 ou RS256 selon version)
- Claims typiques : `xid` (XUID), `xname`, `cpk` (clé client), `mid`/`tid` PlayFab
- UUID interne souvent dérivé : MD5 sur `"pocket-auth-1-xuid:" + xuid` avec bits version UUID

Combiner avec chain legacy si présente (ex. `titleId`).

## Client data JWT

- Vérifié avec la clé publique client extraite de la chain / `cpk`
- Contenu : skin, cape, device model, `ServerAddress`
- Anti-relay : comparer `ServerAddress` à `server-ip` / `server-port` / `bedrock-port` du `conf.txt` si option activée

## Handshake post-login

1. **Server To Client Handshake** — JWT serveur + sel ECDH
2. **Client To Server Handshake** — confirmation
3. **Play Status** — succès
4. Chiffrement paquets jeu selon version

`mcrust-bedrock` gère cette phase ; le bridge ne crée le `Player` qu’après **auth + handshake** réussis.

## Vers l’objet `Player` unifié

- `platform = Bedrock`
- `xuid` = XUID Xbox (string, non vide en online)
- `uuid` = identity UUID vérifié (ou dérivé XUID selon chemin auth)
- `name` = display name **vérifié** (extraData / OIDC)
- Skin : client data JWT → encodage spawn Bedrock ; pour les viewers Java, conversion/limites côté bridge (phase ultérieure)

Même struct **`Player`** que Java — seul `platform` et les champs remplis diffèrent.

## Guest mode

`AuthenticationType == 1` : non supporté (refus explicite, log).

## Clés et cache

- Clé racine Mojang ECDSA embarquée + rotation via docs Mojang si applicable
- OIDC : fetch JWKS avec cache TTL (`bedrock-oidc-jwks-url` ou défaut écosystème)
- Optionnel : `GET https://api.minecraftservices.com/publickeys` pour alignement services Microsoft

## Différence critique vs Java

| | Java online | Bedrock online |
|--|-------------|----------------|
| Preuve | HTTP `hasJoined` + RSA/AES | JWT chain / OIDC |
| Identifiant fort | UUID Mojang | XUID + UUID identity |
| HTTP serveur | Oui | Non (vérif locale crypto) |

## Tests

- JWT fixtures (3-chain valide, chaîne 1 rejetée en online)
- Client data signature invalide → disconnect
- Client Bedrock réel sur réseau local avec `bedrock-online-mode=true`