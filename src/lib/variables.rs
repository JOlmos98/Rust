
//La palabra PUB es como "public" en Java, hace accesible desde otros archivos esa variable o método.
pub const MI_CONSTANTE: i32 = 42; //"pub" hace la variable pública

pub fn saludar() {
    println!("¡Hola desde otro archivo! Concretamente el archivo 'variables.rs'");
}

pub fn desc_variables(){

    /*    Diferencias entre let || let mut || const:
        - let: Variable que se asigna una vez y solo una vez.
        - let mut: Variable que se asigna una vez y puede cambiar.
        - const: Variable que no se puede cambiar.
     */

    let mut edad:i8=26;                 //Un i8 significa un int de 8 bits, es decir, un número entre -128 y 127, es decir, 2^8 (256), teniendo en cuenta el rango de negativos. 
    let horas_jugadas:u8=255;           //La letra "u" representa el valor absoluto, como aquí no hay negativos, podemos usar los 8 bits para llegar al 256 (2^8).
    //Es decir, hay 256 posibilidades en cuanto a los 8 bits, pero uno usa el rango positivo y negativo y otro no.

    //Existen los números ENTEROS: i8, i16, i32, i64, i128, isize || u8, u16, u32, u64, u128, usize. (isize y usize son determinados según si nuestra plataforma de ejecución es x32 o x64...)
    //Existen los números DECIMALES: f32, f64.

    println!("Edad: {} (i8) \nHoras jugadas: {} (u8)", edad, horas_jugadas);

    edad=27;

    print!("Un año más tarde... {}. (Al haberla declarado como mut, se puede cambiar)", edad);
}