# json-2-plantuml

Outil en ligne de commande (Rust) qui lit un annuaire téléphonique au format JSON, range les numéros dans un **Trie** (arbre de préfixes, un chiffre par nœud) et génère une **mindmap PlantUML** de cet arbre.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) — optionnel, pour générer le SVG

## Format d'entrée

Un tableau JSON d'objets `nb` / `name`, placé dans le dossier `data/` :

```json
[
    { "nb": "0412578440", "name": "Alice" },
    { "nb": "112", "name": "Urgences" }
]
```

## Utilisation

```sh
cargo run
```

Le programme demande le nom du fichier **sans l'extension** (ex : `data` pour `data/data.json`) et redemande tant que le fichier est introuvable ou mal formé.

Le résultat est écrit dans `output/output.puml`. Par exemple, pour le seul contact `112` :

```
@startmindmap
* 1
** 1
*** 2
**** Urgences
@endmindmap
```

Pour le convertir en SVG (`output/output.svg`) :

```sh
docker compose up -d
```

## Structure

| Fichier | Rôle |
|---|---|
| `src/input.rs` | Saisie du nom de fichier |
| `src/json.rs` | Chargement et désérialisation du JSON |
| `src/user_info.rs` | Modèle d'un contact (`name`, `nb`) |
| `src/trie.rs` | Construction du Trie |
| `src/plantuml.rs` | Génération du fichier `.puml` |

## Tests

```sh
cargo test
```

La CI GitHub Actions lance les tests sur chaque pull request vers `main` et `develop`.
