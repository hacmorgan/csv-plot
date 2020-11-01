use std::io;
use std::convert::TryInto;

use crate::Dataset;

/**
accumulate: collect points on stdin, call update_plot() when buffer is full
 */
// pub fn accumulate( mut datasets : Vec < Dataset > ) -> Vec < Dataset >
pub fn accumulate( mut datasets : &mut Vec < Dataset > ) -> Option< &mut Vec < Dataset > >
{
    fn push_points( line : &String , datasets : &mut Vec < Dataset > )
    {
        for d in datasets {
            d.points.push( get_points(&line, d.columns) );
        }
    }

    let mut input = String::new();
    // loop {
    //     match io::stdin().read_line( &mut input ) {
    //         Ok(0)    => break,  // EOF
    //         Ok(_)    => push_points( &input, &mut datasets ), 
    //         Err(err) => eprintln!( "error: {:?}", err ),
    //     }
    //     input = "".to_string();
    // }
    // datasets
    for i in 1..accumulator_size( &datasets ) {
        match io::stdin().read_line( &mut input ) {
            Ok(0)    => return None,  // EOF
            Ok(_)    => push_points( &input, &mut datasets ), 
            Err(err) => eprintln!( "error: {:?}", err ),
        }
        input = "".to_string();
    }
    Some( &mut datasets )
}


fn accumulator_size( data : &Vec< Dataset > ) -> u16
{
    match data.iter().next() {
        Some(d) => d.accumulator_size,
        None    => {
            eprintln!( "error: no datasets supplied" );
            std::process::exit(1);
        },
    }
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
    let mut processed : [ f32 ; 3 ] = [ 0.0, 0.0, 0.0 ];

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
