use std::io;
use std::convert::TryInto;

use crate::{ plot, Dataset };

/**
accumulate: collect points on stdin, call update_plot() when buffer is full
 */
pub fn accumulate( mut data : Dataset ) 
{
    let mut input = String::new();
    loop {
        match io::stdin().read_line( &mut input ) {
            Ok(0)    => break,  // EOF
            Ok(_)    => data.points.push( get_points(&input, data.columns) ),
            Err(err) => eprintln!( "error: {:?}", err ),
        }
        input = "".to_string();
    }
    plot::update_plot( &data );
}


fn get_points( line : &String , columns : [ i8 ; 3 ] ) -> [ f32 ; 3 ]
{
    fn invalid_index_error( col : i8 , pts_len : i8 )
    {
        eprint!( "error: looked for a value in column {}, ", col );
        eprintln!( "but found only {} columns on stdin", pts_len );
    }

    fn parse_value( row : &Vec < &str > , index : i8 ) -> f32
    {
        let index : usize = index.try_into().unwrap();
        let cast : Result < f32, _ > = row[index].parse();
        match cast {
            Ok(n) => return n,
            Err(err) => {
                eprintln!( "error: {:?}", err );
                std::process::exit(1);
            },
        }
    }
    
    let pts : Vec < &str > = line.trim().split(",").collect();
    let len : i8 = pts.len().try_into().unwrap();
    let mut processed : [ f32 ; 3 ] = [ -1.0, -1.0, -1.0 ];

    for i in 0..3 {
        let col = columns[i];
        if col < 0 {
            continue;
        } else if col > (len - 1) {
            invalid_index_error( col, len );
        } else {
            processed[i] = parse_value( &pts, col );
        }
    }

    processed
}
