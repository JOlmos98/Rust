

pub fn operaciones(){

    //Como en la mayoría de lenguajes, en Rust se pueden declarar variables sin tipar.

    let a=2;
    let b=3;
    let suma_a_b=a+b;

    println!("La suma de A(2) y B(3) es {}", suma_a_b);
    println!("El resultado de la operación usando la función sumar es {}", sumar(a, b));
    

    let f_a:f32=a as f32;
    let f_b:f32=b as f32;
    let c:f32=0 as f32;

    //Cargo nos avisa de que no se puede reasignar a una variable declarada como f32.
    println!("El resultado de la resta (a-b) inicializando C en esta propia línea es {}", c=f_a-f_b);
    println!("El resultado de la multiplicación (a*b) reasignando C en esta propia línea es {}", c=f_a*f_b);
    println!("El resultado de la división (a/b) reasignando C en esta propia línea es {}", c=f_a/f_b);

    /*
    Una de las ventajas (o desventajas) de Rust es que nos da
    avisos por cualquier cosa, quiere que lo programemos todo
    de la mejor forma posible. Más bien es Cargo el que nos
    avisa de todo eso.
     */
}

//Esta línea significa que recibe un i32 (a), otro i32 (b) y devuelve (->) un i32.
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

