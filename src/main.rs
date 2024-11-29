
mod variables; //Importamos el módulo variables.

fn main() {
    println!("Hello, world!");
    println!("Imprimimos la variable importada de 'variables': {}", variables::MI_CONSTANTE);
    variables::saludar(); //En Java sería algo como variables.saludar(), aquí se usan los dobles dos puntos "::".



    

    /*let mut edad: String=String::new();

    println!("Inserta tu edad: ");

    //io::stdin().read_line(&mut edad)
        //.expect("Error al leer la entrada");

    //VER SI HAY QUE METER UN CRATE PARA IO O QUE PASA AQUI, VER SI CAMBIANDO EL NOMBRE DE MAIN.RS TAMBIEN FUNCIONA EL MAIN.
    //Cambiamos a la rama main. 

    let edad: i32 = edad.trim().parse()
        .expect("Error al convertir la edad a un número");

    if edad < 18 {
        println!("Eres MENOR de edad.");
    } else {
        println!("Eres MAYOR de edad.");
    }*/

    println!("El programa ha terminado correctamente.");
}
