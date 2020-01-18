# Installation (Unix)

1. Installer Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Installer `cargo web`

Assurez vous d'avoir un compilateur C à disposition (clang, gccc...)
```
cargo install cargo-web
```

3. Compilation et exécution
```
cd particules/particule_app && cargo web start
```

Si tout c'est bien passé, l'application devrait être disponible à l'adresse suivante : `localhost:8000`

## Todos

- [ ] environement torique
- [ ] collision
- [x] séparation des composant (Yew)
- [x] ajouter des agents par l'interface web
