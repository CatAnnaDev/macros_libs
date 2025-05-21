# 📦 rust-utils-macros

**Une collection de macros Rust pour booster la productivité, simplifier les logs, manipuler JSON, structurer les données, chronométrer, et plus encore.**

---

## ✨ Fonctionnalités principales

### 🔁 Répétition & Itération

* `repeat_n!(n, expr)`

  > Répète une expression `n` fois.

* `for_each!([a, b, c], var => { ... })`

  > Itère sur chaque élément d'une liste inline.

### 🧾 JSON

* `to_json!(val)`

  > Sérialise une valeur en JSON joliment formaté (nécessite `serde_json`).

* `from_json!(str)`

  > Désérialise une chaîne JSON en une structure (nécessite `serde_json`).

### 📚 Chaînage

* `chain!(val => method1::method2::method3)`

  > Enchaîne des appels de méthodes sur une valeur initiale.

### 🧹 Structures & collections

* `vec_of_strings!("a", "b", "c")`

  > Crée un `Vec<String>`.

* `map!("clé1" => val1, "clé2" => val2)`

  > Crée une `HashMap`.

### 🧪 Debug & chronométrage

* `dbg_expr!(expr)`

  > Affiche l'expression, sa valeur et l'emplacement dans le code.

* `time_it!("description", { block })`

  > Chronomètre un bloc et affiche la durée d'exécution.

* `measure!({ block })`

  > Renvoie `(résultat, durée)` du bloc.

### 🔁 Retry automatique

* `retry!(max_retries, delay_ms, expression)`

  > Réessaie une opération plusieurs fois avec délai avant d’échouer.

### 🌱 Variables d’environnement

* `get_env!("MY_VAR")`

  > Récupère la valeur d'une variable d'environnement.

### 📋 Logging enrichi

* `log_info!(...)`, `log_warn!(...)`, `log_error!(...)`, `log_debug!(...)`, `log_success!(...)`

  > Logs colorés avec timestamp. Possibilité d'enregistrer dans un fichier :

```rust
enable_log_saving("log.txt");
set_max_log_file_size(1024 * 1024); // 1 Mo
set_log_level_filter("INFO"); // Peut être "DEBUG", "WARN", etc.
```

---

## 🚀 Installation

Ajoute cette dépendance dans ton `Cargo.toml` :

```toml
[dependencies]
rust-utils-macros = { path = "./chemin/vers/ta/lib" }
```

Et dans ton code :

```rust
use rust_utils_macros::*;
```

---

## 📂 Exemple d'utilisation

```rust
fn main() {
    enable_log_saving("app.log");
    set_max_log_file_size(100_000);
    set_log_level_filter("INFO");

    log_info!("Application démarrée");

    repeat_n!(3, log_debug!("Tentative de debug")); // Visible en debug uniquement

    let noms = vec_of_strings!("Alice", "Bob");
    for_each!([1, 2, 3], n => {
        log_info!("n = {}", n);
    });

    let tentative = retry!(5, 1000, || -> Result<(), &str> {
        Err("échec temporaire")
    });

    dbg_expr!(tentative);

    let (res, dur) = measure!({
        (0..1000000).sum::<u64>()
    });
    println!("Résultat = {}, durée = {:?}", res, dur);
}
```
