//Melisa Mendizabal 23778

use raylib::prelude::*;
use crate::line::line;
use crate::framebuffer::Framebuffer;

pub fn poligono(
    framebuffer: &mut Framebuffer,
    lista: &[Vector2],

) -> Vec<Vector2> {
    let mut contorno: Vec<Vector2> = vec![];

    for i in 0..lista.len() - 1 {
        let inicio = lista[i];
        let fin = lista[i+1];

        let mut rastroLinea = line(
            framebuffer,
            inicio,
            fin,
        );
        contorno.append(&mut rastroLinea);
        
    }

    let mut rastroLinea = line(
        framebuffer,
        lista[lista.len()-1],
        lista[0],
        
    );

    contorno.append(&mut rastroLinea);
    return contorno;

}