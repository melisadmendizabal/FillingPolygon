//Melisa Mendizabal 23778

mod framebuffer;
use framebuffer::Framebuffer;
use raylib::color::Color;
use raylib::prelude::*;
mod poligono;
use poligono::poligono;
mod relleno;
use relleno::relleno;
mod line;
use line::line;



fn main() {
    let width = 800;
    let height = 600;
    let bgColor = Color::new(0,0,0,0);

    let mut fb = Framebuffer::new(width, height, bgColor);
    let figura1 = vec![
        Vector2::new(165.0, 380.0), 
        Vector2::new(185.0, 360.0), 
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0), 
    ];

    let figura2 = vec![
        Vector2::new(321.0, 335.0), 
        Vector2::new(288.0, 286.0), 
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0), 
    ];

    let figura3 = vec![
        Vector2::new(377.0, 249.0), 
        Vector2::new(411.0, 197.0), 
        Vector2::new(436.0, 249.0),
    ];

    let figura4 = vec![
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0), 
        Vector2::new(502.0, 88.0), 
        Vector2::new(553.0, 53.0), 
        Vector2::new(535.0, 36.0), 
        Vector2::new(676.0, 37.0), 
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0), 
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0), 
        Vector2::new(659.0, 214.0), 
        Vector2::new(615.0, 214.0), 
        Vector2::new(632.0, 230.0), 
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0), 
        Vector2::new(517.0, 144.0), 
        Vector2::new(466.0, 180.0),
    ];

    let figura5 = vec![
        Vector2::new(682.0, 175.0), 
        Vector2::new(708.0, 120.0), 
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0), 
    ];



    fb.set_curret_color(Color::new(162, 90, 199,255));
    poligono(
        &mut fb,
        &figura1,   
    );

    relleno(
        &mut fb,
        &figura1, 
    );

    relleno(
        &mut fb,
        &figura2, 
    );

    relleno(
        &mut fb,
        &figura3, 
    );

    relleno(
        &mut fb,
        &figura4, 
    );

    fb.set_curret_color(bgColor);

    relleno(
        &mut fb,
        &figura5, 
    );

    fb.set_curret_color(Color::new(104, 37, 138, 255));

    poligono(
        &mut fb,
        &figura1, 
    );

    poligono(
        &mut fb,
        &figura2, 
    );

    poligono(
        &mut fb,
        &figura3, 
    );

    poligono(
        &mut fb,
        &figura4, 
    );

    poligono(
        &mut fb,
        &figura5, 
    );

    let output_file = "out.png";
    fb.render_to_file(output_file);

    
    
}
