== Como instalar ( Cargo.toml )
```Crate
[dependencies]
validador_crate_rust = "0.1.0"
```

== Como utilizar
```rust
use validador_crate_rust as vd;
use std::io;


fn main() {
    println!("Digite um CPF");

    let mut cpf = String::new();

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("Você digitou: {}", cpf.trim());
        },
        Err(e) => {
            println!("Erro ao ler entrada: {}", e);
        }
    }

    let validado: bool = vd::validar_cpf(cpf.as_str());

    if validado {
        println!("O CPF é valido")
    } else {
        println!("O CPF é inválido")
    }
}

```
