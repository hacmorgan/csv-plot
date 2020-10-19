extern crate clap;

use std::convert::TryInto;
use crate::Dataset;


pub fn initialise() -> Vec < Dataset >
{
    let given_args = get_args();
    let fields = given_args.value_of("fields").unwrap();
    let mut datasets : Vec < Dataset > = Vec::new();
    
    for x in find_xs( fields ) {
        datasets.push(
            Dataset {
                columns : infer_columns( x, fields ),
                _accumulator_size : 10,
                points : Vec::new(),
                colour : "red",
            }
        )
    }

    datasets
}
    
//     Dataset {
//         columns           : infer_columns( given_args.value_of("fields").unwrap() ),
//         _accumulator_size : 10,
//         points            : Vec::new() ,
//         colour            : "red",
//     }
// }


/** determine how many datasets there are by counting the number of x_
    fields given, and verify that a matching y_ field was given */
fn find_xs( fields : &str ) -> Vec < &str >
{
    fn is_valid_field( field : &str, separated : &std::str::Split <&str> ) -> bool
    {
        let y_equiv : String = field
            .chars()
            .enumerate()
            .map( |(i,c)| if i == 0 { 'y' } else { c } )
            .collect();
        
        if separated.position( |f| f == &y_equiv ) != None {
            true
        } else {
            false
        }
    }
    
    let mut ret : Vec < &str > = Vec::new();
    let separated = fields.trim().split(",");
    
    for field in &separated {
        if is_valid_field( &field, &separated ) {
            ret.push(field);
        }
    }

    ret
}


fn get_args() -> clap::ArgMatches< 'static > 
{
    clap::App::new( "csv-plot" )
        .version( "0.0" )
        .author( "Hamish Morgan" )
        .about( "plot data from stdin using gnuplot" )
        .arg( clap::Arg::with_name("fields")
              .short("f")
              .long("fields")
              .help("format of data on stdin")
              .takes_value(true)
              .default_value("x,y") )
        .get_matches()
}


/** return column numbers for fields matching the pattern of x */
fn infer_columns( x : &str , fields : &str ) -> [ i8 ; 3 ]
{
    fn extract( val : usize ) -> i8
    {
        val.try_into().unwrap()
    }
    
    let mut columns : [ i8 ; 3 ] = [ -1, -1, -1 ];
    let separated = fields.split(",");

    for (count, field) in separated.enumerate() {
        match field {
            "x" => columns[0] = extract(count),
            "y" => columns[1] = extract(count),
            "z" => columns[2] = extract(count),
            &_  => continue,
        }
    }

    columns
}
