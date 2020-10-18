extern crate clap;

use std::convert::TryInto;
use crate::Dataset;


pub fn initialise() -> Dataset
{
    let given_args = get_args();
    
    Dataset {
        columns          : infer_columns( given_args.value_of("fields").unwrap() ),
        accumulator_size : 10,
        points           : Vec::new() ,
        colour           : b"red",
    }
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


fn infer_columns( fields : &str ) -> [ i8 ; 3 ]
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
