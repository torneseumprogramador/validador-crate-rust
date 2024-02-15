Como instalar ( Cargo.toml )
```Crate
[dependencies]
validador_crate_rust = "1.0.0"
```

Como utilizar
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

    let validado: bool = vd::validadores::cpf(cpf.as_str());
    // let validado: bool = vd::validadores::cnpj(cnpj.as_str()); ou CNPJ

    if validado {
        println!("O CPF é valido")
    } else {
        println!("O CPF é inválido")
    }
}

```
