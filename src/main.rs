fn main() {
    let muestra: f64 = 300.0;
    let pasos: u8 = 10;
    let factor_dilusion: i32 = 10;
    let y: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    let x = diluir(muestra, factor_dilusion as f64, pasos as usize);
    let r = calcular_r(&x, &y);

    println!("Las muestras diluidas son: \n{:?}", x);
    println!("El r de la muestra es: {} \nEl r^2 es: {}", r, r.powi(2));
    
}

//Funciones estdisticas-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// Calculas la media de un sett de datos de valor flotante.
fn media(datos: &[f64]) -> f64 {
    datos.iter().sum::<f64>() / datos.len() as f64
}

// Calcula la desviacion estandar usando apartir de la media y el set de datos.
fn devest(datos: &[f64], media: f64) -> f64 {
    let sum: f64 = datos.iter().map(|&dato| (dato - media).powi(2)).sum();
    (sum / (datos.len() as f64 - 1.0)).sqrt()
}

// Calcula el r usando las funciones para calcular la media y la desviacion estandar 
fn calcular_r(x: &[f64], y: &[f64]) -> f64 {
    let mx = media(x);
    let my = media(y);
    let dx: f64 = devest(x, mx);
    let dy: f64 = devest(y, my);

    let mut sum: f64 = 0.0;
    for (i, &item) in x.iter().enumerate() {
        sum += ((item - mx) / dx) * ((y[i] - my) / dy);
    }
    sum / (x.len() as f64 - 1.0)
}

//Funciones de laboratorio----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// Calcula la concentracion de la muestras despues de un numero de diluciones con determinado factor de dilucion y una concentración inicial
fn diluir(muestra: f64, factor: f64, pasos: usize) -> Vec<f64> {
    std::iter::successors(Some(muestra), move |&prev| Some(prev / factor))
        .take(pasos)
        .collect()
}
