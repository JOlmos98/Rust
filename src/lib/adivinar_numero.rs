use rand::Rng;
use std::io;

pub fn adivinar_numero(){
    let numero_secreto = rand::thread_rng().gen_range(1..=100);
    let mut intento = String::new();
    let mut correcto = false;

    println!("¡Adivina el número (entre 1 y 100)!");

    while !correcto {
        println!("Introduce tu intento:");
        intento.clear();
        io::stdin()
            .read_line(&mut intento)
            .expect("Error al leer la entrada");

        let intento: i32 = match intento.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce un número válido.");
                continue;
            }
        };

        if intento < numero_secreto {
            println!("El número es mayor.");
        } else if intento > numero_secreto {
            println!("El número es menor.");
        } else {
            println!("¡Correcto! El número era {}", numero_secreto);
            correcto = true;
        }
    }
}