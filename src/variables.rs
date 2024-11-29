
//La palabra PUB es como "public" en Java, hace accesible desde otros archivos esa variable o método.
pub const MI_CONSTANTE: i32 = 42; //"pub" hace la variable pública

pub fn saludar() {
    println!("¡Hola desde otro archivo! Concretamente el archivo 'variables.rs'");
}