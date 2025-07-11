# ghstats

CLI Rust pour générer des statistiques journalières à partir des événements GitHub publics fournis par [gharchive.org](https://www.gharchive.org).

## 🔧 Fonctionnement

Le programme télécharge, décompresse et analyse les fichiers `.json.gz` d’un jour donné (1 par heure, 24 total), entièrement en streaming. Il passe chaque événement à un ou plusieurs "collectors" qui extraient des statistiques spécifiques.

La sortie est un **fichier JSON unique imprimé sur stdout**.

## 📦 Exemple d'utilisation

```bash
# Générer les stats du 1er juillet 2025
ghstats 2025-07-01 > stats-2025-07-01.json

# Générer un mois complet en parallèle
seq -w 01 31 | parallel -j 4 "ghstats 2025-07-{} > stats/2025-07-{}.json"
````

## 🎯 Principes

* Full streaming : HTTP → gzip → JSON → struct
* Aucune écriture disque intermédiaire
* Architecture modulaire via `Collector` trait
* Mono-thread pour performance simple et fiable
* Extensible par ajout de collectors spécialisés

## 📤 Sortie

Le programme écrit un JSON structuré unique sur `stdout`. Aucun log, métrique ou trace n’est imprimé.

## 📄 Licence

MIT
