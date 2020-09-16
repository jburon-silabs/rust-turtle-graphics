use std::fs::File;
use std::io::{Write, Error};

struct Point {
    x : isize, 
    y : isize
}

struct Line {
    start : Point, 
    end : Point
}

/// draw_the_world(
/// world: &Vec<Line>, 
/// world_width: isize, 
/// world_height: isize) -> Result<(), Error>
/// 
/// # Example
/// 
/// ```rust
/// let status = draw_the_world(&world, world_width, world_height);
/// ```
fn draw_the_world(world: &Vec<Line>, world_width: isize, world_height: isize) -> Result<(), Error> {
    let path = "svg_rendering.html"; // output file name

    let mut output = File::create(path)?;
    let html_header = "<html><body><h1>Some SVG</h1>";
    let html_footer = "</body></html>";
    let svg_footer = "</svg>";

    write!(output, "{}\n", html_header)?; // write html header to file
    write!(output, "<svg width=\"{}\" height=\"{}\">\n", world_width, world_height)?; // write svg header to file
    // write line from vector to file
        for lines in world {
            write!(output, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(255,0,0);stroke-width:2\"/>\n", 
            lines.start.x, 
            lines.start.y, 
            lines.end.x, 
            lines.end.y)?;
        }     
    write!(output, "{}\n", svg_footer)?; // write svg footer to file
    write!(output, "{}\n", html_footer)?; // write html footer to file

    Ok(())
}


/// # Example
/// Test data
/// ```svg
/// <line x1="0" y1="10" x2="20" y2="20" />
/// <line x1="20" y1="20" x2="40" y2="10" />
/// <line x1="40" y1="10" x2="60" y2="80" />
/// <line x1="60" y1="80" x2="80" y2="90" />
/// <line x1="80" y1="90" x2="100" y2="20" />
/// ```
fn main() {

    // Lets build some 200x200px 5 lines world :)
    let mut world: Vec<Line> = Vec::new();
    let world_width: isize = 200;
    let world_height: isize = 200;
    
    // Lets define lines
    let line0 = Line {
        start: Point {x:0, y:10},
        end: Point {x:20, y:20}
    };

    let line1 = Line {
        start: Point {x:20, y:20},
        end: Point {x:40, y:10}
    };

    let line2 = Line {
        start: Point {x:40, y:10},
        end: Point {x:60, y:80}
    };

    let line3 = Line {
        start: Point {x:60, y:80},
        end: Point {x:80, y:90}
    };

    let line4 = Line {
        start: Point {x:80, y:90},
        end: Point {x:100, y:20}
    };
  
    // Push lines to world vector
    world.push(line0);
    world.push(line1);
    world.push(line2);
    world.push(line3);
    world.push(line4);

    // Lets render this world!    
    let status = draw_the_world(&world, world_width, world_height);

    match status {
        Ok(v) => println!("Done {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
}

