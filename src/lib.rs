#[macro_use]
extern crate lalrpop_util;

pub mod ast;

use std::fs::File;
use std::io::{Write, Error};

lalrpop_mod!(pub parser);

pub fn compile(input: &str) -> Result<ast::Program, String> {
    match parser::ProgramParser::new().parse(input) {
        Ok(s) => Ok(ast::Program::new(s)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub fn parse_turtle_commands(source: String)
{
    let world: Vec<ast::Line> =
    match compile(&source) {
        Ok(c) => {
            match c.interpret() {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Runtime error: {}", e);
                    std::process::exit(1);
                },
            }
        },
        Err(_) => {
            //eprintln!("Compilation error: {}", e);
            //std::process::exit(0);
            return
        },
    };

    let status = draw_the_world(&world, None, None); // Use this for default world size (200x200)
    // let status = draw_the_world(&world, Some(300), Some(300)); // Use this for custom world size

    match status {
        Ok(v) => println!("Done {:?}", v),
        Err(e) => println!("Error writing file: {:?}", e),
    }
}

//// draw_the_world(&vec!<Line>, world_width: Option<isize>, world_height: Option<isize> ) - 
pub fn draw_the_world(world: &Vec<ast::Line>, world_width: Option<isize>, world_height: Option<isize>) -> Result<(), Error> {
    let path = "svg_rendering.html";

    let mut output = File::create(path)?;
    let html_header = "<html><body><h1>Some SVG</h1>";
    let html_footer = "</body></html>";
    let svg_footer = "</svg>";

    write!(output, "{}\n", html_header)?;
    write!(output, "<svg width=\"{}\" height=\"{}\">\n", world_width.unwrap_or(200), world_height.unwrap_or(200))?;
    // write line from vector 
        for lines in world {
            write!(
                output,
                "<line
                    x1=\"{}\"
                    y1=\"{}\"
                    x2=\"{}\"
                    y2=\"{}\"
                    style=\"stroke:rgb({},{},{});stroke-width:2\"
                />\n",
                lines.start.x,
                lines.start.y,
                lines.end.x,
                lines.end.y,
                lines.color.r,
                lines.color.g,
                lines.color.b,
            )?;
        }     
    write!(output, "{}\n", svg_footer)?; 
    write!(output, "{}\n", html_footer)?;

    Ok(())
}