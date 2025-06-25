# Apprentissage 

## Pour faire un appel HTTP, utiliser reqwest et tokio

```rust
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://httpbin.org/get";
    let response = reqwest::get(url).await?;

    let body = response.text().await?;
    println!("Response body:\n{}", body);

    Ok(())
}
```

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

## Pour obtenir une valeur absolue

```rust
fn main() {
    let x: i32 = -42;
    let abs_x = x.abs();
    println!("La valeur absolue de {} est {}", x, abs_x);
}
```

- .abs() n'est disponible que pour les types signés (pas u32, u64, etc.).
- Il peut y avoir un overflow avec i32::MIN.abs() car -i32::MIN n'est pas représentable en i32.

## Trier un vecteur 
dans l'odre croissant
```rust
fn main() {
    let mut vec = vec![3, 1, 4, 1, 5, 9];
    vec.sort(); // tri croissant
    println!("{:?}", vec); // [1, 1, 3, 4, 5, 9]
}
```

ordre croissant dans un struct
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    // Tri par âge croissant
    people.sort_by_key(|p| p.age);

    println!("{:?}", people);
}
```

- .sort() trie in-place et nécessite que le type implémente Ord.
- Pour un tri sans modifier le vecteur original, tu peux cloner ou utiliser sorted() de l'itérateur (via le crate [itertools] si besoin).

Avec itertools
```rust
use itertools::Itertools;

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9];

    // .iter() donne des références, donc on trie &i32
    let sorted: Vec<_> = numbers.iter().sorted().collect();

    // Si tu veux des i32, fais un .cloned() ou .copied()
    let sorted_values: Vec<_> = numbers.iter().copied().sorted().collect();

    println!("{:?}", sorted_values); // [1, 1, 3, 4, 5, 9]
}

```

## compter le nombre d'occurences dans un vecteur

```rust
fn main() {
    let numbers = vec![1, 2, 3, 2, 4, 2, 5];
    let target = 2;

    let count = numbers.iter().filter(|&&x| x == target).count();

    println!("Le nombre {} apparaît {} fois.", target, count);
}
```

## Attendre un resultat asynchrone


En Rust, pour attendre le résultat d'une fonction async, tu dois l’appeler avec .await, à l’intérieur d’un contexte async, comme une fonction async fn, un bloc async, ou dans le main via un runtime asynchrone (comme tokio).

✅ 1. Exemple simple dans une fonction async
```rust
async fn do_something() -> i32 {
    42
}

async fn run() {
    let result = do_something().await;
    println!("Résultat : {}", result);
}
```

🚀 2. Attendre dans main → nécessite un runtime async
Rust ne permet pas un main async tout seul. Tu dois utiliser un runtime asynchrone comme [tokio] ou [async-std].


```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```
Puis dans main.rs :

```rust
#[tokio::main]
async fn main() {
    let result = do_something().await;
    println!("Résultat : {}", result);
}
async fn do_something() -> i32 {
    42
}
```

## Prendre les deux prochains items dans un vecteur

Pour prendre un élément et le suivant dans un vecteur en Rust, tu peux parcourir le vecteur avec un index et accéder à l'élément courant et le suivant via vec[i] et vec[i + 1]. Tu peux aussi utiliser des méthodes d'itération comme .windows(2) qui est plus élégante et sécurisée.

✅ 1. Utiliser .windows(2) pour accéder à chaque paire consécutive
```rust

fn main() {
    let vec = vec![10, 20, 30, 40];
    for window in vec.windows(2) {
        println!("Courant: {}, Suivant: {}", window[0], window[1]);
    }
}
```

## Caster un tableau de string vers des i32

Exemple avec Vec<String> vers Vec<i32>
Si tu as un vecteur de String et que tu veux le convertir en Vec<i32>, tu peux utiliser une itération avec map et la méthode parse().

```rust
fn main() {
let string_array = vec!["10".to_string(), "20".to_string(), "30".to_string()];

    let int_array: Vec<i32> = string_array.iter()
        .map(|s| s.parse::<i32>().unwrap()) // `unwrap` ou gestion d'erreur appropriée
        .collect();

    println!("{:?}", int_array); // [10, 20, 30]
}
```

✅ 1. Compter les éléments qui respectent une condition avec iter().filter().count()
Si tu veux compter combien d'éléments respectent une condition, tu peux utiliser iter() pour parcourir les éléments, filter() pour appliquer la condition, puis count() pour compter les éléments qui passent la condition.

## Exemple : Compter les éléments supérieurs à 10 dans un vecteur
```rust
fn main() {
    let vec = vec![5, 10, 15, 20, 25, 30];

    // Compter les éléments supérieurs à 10
    let count = vec.iter()
        .filter(|&&x| x > 10)  // Condition : x > 10
        .count();

    println!("Nombre d'éléments supérieurs à 10 : {}", count); // Affiche 5
}
```
Explication :
- iter() : Crée un itérateur sur les éléments du vecteur.
- filter() : Applique la condition donnée pour filtrer les éléments. Ici, on veut ceux qui sont supérieurs à 10.
- count() : Compte combien d'éléments ont passé la condition.

## Module system

A crate is the smallest amount of code that the Rust compiler considers at a time.
Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.

A crate can come in one of two forms: a binary crate or a library crate. 
- Binary crates are programs you can compile to an executable that you can run, such as a command line program or a server ( have a function called main)
- Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate
A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. 
Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. 
Cargo passes the crate root files to rustc to build the library or binary.