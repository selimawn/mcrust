# Décisions d’architecture (ADR)

Enregistrement des choix validés (widget + discussion). Date de référence : 2026-06.

## Produit & versions

| ID | Décision |
|----|----------|
| D-001 | Premier jalon jouable : **Java 1.21.1** + **Bedrock 1.21.x** (monde plat, join). |
| D-002 | Java : **multi-versions** (négociation handshake / plage supportée), pas une seule `protocol-version` figée. |
| D-003 | Bedrock : supporter **2 patchs** 1.21.x (deux entiers protocole + tables registry). |

## Rust & crates (commit initial)

| ID | Décision |
|----|----------|
| D-004 | Workspace v1 : **`mcrust-wire`**, **`mcrust-protocol`**, **`mcrust-server`** (binaire + config). |
| D-005 | ECS (phase core) : **`bevy_ecs`** sans renderer. |
| D-006 | Tick : **thread dédié** + **channels** (tokio pour I/O uniquement). |
| D-007 | RakNet : **crate Rust existante** derrière façade `mcrust-bedrock` (crate à choisir à l’implémentation Bedrock). |

## Données & wire

| ID | Décision |
|----|----------|
| D-008 | Registry : **JSON Bedrock Mojang** + **table Java dérivée** (`assets/registries/`). |
| D-009 | NBT : dépendance **`fastnbt`** (ou équivalent) encapsulée dans `mcrust-wire`. |

## Auth & dev

| ID | Décision |
|----|----------|
| D-010 | Auth **officielle** Java et Bedrock en prod (`online-mode`, `bedrock-online-mode`). |
| D-011 | Dev : **les deux modes** configurables en parallèle via `conf.txt`. |
| D-012 | Modèle **`Player` unique** avec champ **`platform: Java \| Bedrock`**. |

## Ops & qualité

| ID | Décision |
|----|----------|
| D-013 | Logs : **`tracing`** + **`tracing-subscriber`**. |
| D-014 | CI : **GitHub Actions** — `fmt`, `clippy`, `test`. |
| D-015 | Licence : **MIT** + **attribution obligatoire** (nom + lien mcrust en tête des crédits / README des dérivés). Voir [LICENSE](../LICENSE). |

## Prochaine priorité code (validée)

| ID | Décision |
|----|----------|
| D-016 | Enchaîner **workspace + mcrust-wire** (VarInt, string MC, re-export NBT). |

## Liens

- [ARCHITECTURE.md](ARCHITECTURE.md)
- [server/conf.txt.md](server/conf.txt.md)