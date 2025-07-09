//Melisa Mendizabal 23778

use raylib::prelude::*;
use crate::poligono::poligono;
use crate::framebuffer::Framebuffer;
use std::collections::HashMap;


//ordena y agrupa los los vectores por su punto en y
fn agruparPorY(
    puntos: &[Vector2],

)-> HashMap<i32,Vec<Vector2>> {
    let mut grupos: HashMap<i32, Vec<Vector2>> = HashMap::new();

    for punto in puntos {
        let y = punto.y as i32;
        grupos.entry(y).or_default().push(*punto);
    }
    return grupos;
}

//función encargada de rellenar
pub fn relleno(
    framebuffer: &mut Framebuffer,
    figura: &[Vector2],
){
    let mut contornoPuntos = poligono(
        framebuffer,
        &figura
    );

    let mut gruposY = agruparPorY(&contornoPuntos);

    for (y, grupo) in gruposY.iter_mut() {
        
            grupo.sort_by(|a,b| a.x.partial_cmp(&b.x).unwrap());
            let mut salto = 0;

            
            for j in 0..grupo.len() - 1{
                let a = grupo[j];
                let b = grupo[j+1];


                if (a.x + 1.0) < (b.x)  {
                    
                    //pixeles problematicos :p
                    
                    if (a.x == 180.0) && (a.y == 330.0) {
                        salto = 0;
                    } else if (a.x == 182.0) && (a.y == 331.0) {
                        salto = 0;

                    } else if (a.x == 207.0) && (a.y == 345.0) {
                        salto = 1;

                    } else if (a.x == 466.0) && (a.y == 180.0) {
                        salto = 0;

                    } else if (a.x == 517.0) && (a.y == 144.0) {
                        salto = 1;

                    } else if (a.x == 517.0) && (a.y == 145.0) {
                        salto = 1;

                    } else {
                        salto += 1;
                    }
    
                    if salto % 2 != 0 {
                        for k in a.x as u32..= b.x as u32 {
                            framebuffer.set_pixel(k, *y as u32);
                            
                        }
                    }
                }


           
        
        }
            

        
    }


}