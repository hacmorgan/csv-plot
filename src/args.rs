extern crate clap;

use std::convert::TryInto;
use crate::Dataset;


pub fn initialise() -> (Vec< Dataset >, bool)
{
    fn check_verbose( args : &clap::ArgMatches<'static> ) -> bool
    {
        if args.is_present("verbose") {
            true
        } else {
            false
        }
    }

    fn check_explain( args : &clap::ArgMatches<'static> )
    {
        if args.is_present( "explain" ) {
            explain();
            std::process::exit(0);
        }
    }
    
    let given_args = get_args();
    check_explain( &given_args );
    
    let fields = given_args.value_of("fields").unwrap();
    let style = match given_args.value_of("style") {
        Some(f) => f.to_owned(),
        None    => String::from(""),
    };
    let plot_options = match given_args.value_of("plot-options") {
        Some(f) => f.to_owned(),
        None    => String::from(""),
    };
    let mut datasets : Vec< Dataset > = Vec::new();
    
    for x in find_xs( fields ) {
        datasets.push(
            Dataset {
                columns            : infer_columns( x, fields )       ,
                accumulator_size  : 10                               ,
                points             : Vec::new()                       ,
                plot               : find_plot( x )                   ,
                style              : find_options( x, &style )        ,
                plot_options       : find_options( x, &plot_options ) ,
            }
        );
    }

    (datasets, check_verbose(&given_args))
}


fn replace_first_char( input : &str, replacement : char ) -> String
{
    input
        .chars()
        .enumerate()
        .map( |(i,c)| if i == 0 { replacement } else { c } )
        .collect()
}


/** determine how many datasets there are by counting the number of x_
    fields given, and verify that a matching y_ field was given */
fn find_xs( fields : &str ) -> Vec< &str >
{
    fn is_valid_field( field : &str, separated : &mut std::str::Split <&str> ) -> bool
    {
        if field.chars().next() != Some('x') {
            return false
        }
        let y_equiv : String = replace_first_char( &field, 'y' );        
        if separated.position( |f| f == &y_equiv ) != None {
            true
        } else {
            false
        }
    }
    
    let mut ret : Vec < &str > = Vec::new();
    let mut separated = fields.trim().split(",");
    
    for field in separated.clone() {
        if is_valid_field( &field, &mut separated ) {
            ret.push(field);
        }
    }

    ret
}


fn find_options( x : &str, options : &String )
                -> Option< Vec< (String, String) > >
{
    /** rectify dataset names and construct Vec < (dataset_name, options) > */
    fn parse_label_options( label_options : String ) -> Vec < (String, String) >
    {
        fn iter_to_string( iter : Vec < &str > ) -> String
        {
            let mut flat = String::new();
            for s in iter {
                flat += s;
                flat += ",";
            }
            flat
        }
        
        let mut label_options_vector : Vec < (String, String) > = Vec::new();
        
        for substring in label_options.split(";") {
            let mut fields = substring.split(",");
            if let Some(x) = fields.next() {
                label_options_vector.push( (rectify_x(x), iter_to_string(fields.collect())) )
            } else {
                eprintln!("Received a bad input: {}", substring);
            }
        }
        
        label_options_vector
    }
    
    /** construct options Vec< (name, value) > */
    fn parse_name_value( name_value : String ) -> Vec< (String, String) >
    {
        let mut options : Vec< (String, String) > = Vec::new();
        for opt in name_value.split(",") {
            let mut pair = opt.split("=");
            if let (Some(name), Some(value)) = (pair.next(), pair.next()) {
                options.push( (String::from(name), String::from(value)) );
            }
        }
        options
    }

    let options_vector : Vec< (String, String) > =
        parse_label_options( options.to_string() );
    for (name, value) in options_vector {
        if name == rectify_x(x) {
            return Some( parse_name_value(value) );
        }
    }

    None
}


    /** x -> xa0, x1 -> xa1, xb -> xb0 */
fn rectify_x( x : &str ) -> String
{
    fn get_figure( x : &str ) -> char
    {
        let c = x.chars().nth(1);
        match c {
            None    => 'a',
            Some(c) => {
                if c.is_alphabetic() {
                    c
                } else {
                    'a'
                }
            }
        }
    }

    fn get_dataset( x : &str ) -> char
    {
        let c1 = x.chars().nth(1);
        let c2 = x.chars().nth(2);
        if c1 == None && c2 == None {
            '0'
        } else if c2 == None {
            let c = c1.unwrap();
            if c.is_alphabetic() {
                '0'
            } else {
                c
            }
        } else {
            c2.unwrap()
        }
    }

    let mut rectified = String::new();
    rectified.push( 'x'            );  // first is guaranteed to be x
    rectified.push( get_figure(x)  );  // dataset's letter
    rectified.push( get_dataset(x) );  // dataset's number

    rectified
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
        if field == x {
            columns[0] = extract(count);
        } else if field == replace_first_char( x, 'y' ) {
            columns[1] = extract(count);
        } else if field == replace_first_char( x, 'z' ) {
            columns[2] = extract(count);
        }
    }

    columns
}


fn find_plot( x : &str ) -> char
{
    rectify_x( x ).chars().nth(1).unwrap()
}


fn get_args() -> clap::ArgMatches< 'static > 
{
    clap::App::new( "csv-plot" )
        .version( "1.0" )
        .author( "Hamish Morgan" )
        .about( "\nplot data from stdin using gnuplot" )
        .arg( clap::Arg::with_name("fields")
              .short("f")
              .long("fields")
              .help("style of data on stdin")
              .takes_value(true)
              .default_value("x,y") )
        .arg( clap::Arg::with_name("style")
              .short("s")
              .long("style")
              .help("per-dataset style (e.g. colour, caption)")
              .takes_value(true) )
        .arg( clap::Arg::with_name("plot-options")
              .short("p")
              .long("plot-options")
              .help("per-plot settings (e.g. xlabel, title)")
              .takes_value(true) )
        .arg( clap::Arg::with_name("verbose")
              .short("v")
              .long("verbose")
              .help("more output") )
        .arg( clap::Arg::with_name("explain")
              .short("e")
              .long("explain")
              .help("print usage examples and more detailed explanations") )
        .get_matches()
}


fn explain()
{
    let help = "
SPECIFYING INPUT COLUMNS (--fields)
csv-plot can plot in 2D and 3D, and will automatically choose based on the given
input columns. These are specified via a comma-separated list of strings, known 
as `column-specifiers`.

Column-specifiers are three characters long, but can be abbreviated for 
convenience. 
1. The first character specifies the axis, so can only be x, y, or z
   - This character is mandatory
2. The second is a character [a-z] which can be used to draw multiple plots from
   a single invocation of csv-plot.
   - Default: a   
3. The third is an integer which allows multiple datasets to be drawn on the 
   same plot. 
   - Default: 0

Abbreviation examples:
- x  = xa0
- y1 = ya1
- zb = zb0


PARSING MULTIPLE OPTIONS IN ONE STRING
In order to specify options for multiple datasets, the following grammar is used
to parse options for --style and --plot-settings:

    <dlabel>,<option>[,<option>];<dlabel>,<option>

- a dataset's <dlabel> is the column specifier for its x data
- each <option> is of the form <name>=<value>
- multiple comma-separated options can be specified per xlabel
- multiple semicolon-separated sets of xlabels and options can be specified


PER-DATASET SETTINGS (--style)
- colour=<colour>; colour of line or points 
  - accepts common colours such as `red`, or hexadecimal codes
  - default: `red`
- type=<type>; plot type
  - accepts `lines` or `points`
  - default: `lines`
- caption=<caption>; name of the dataset (displayed in the legend)
  - default: \"\" (none)


PER-PLOT SETTINGS (--plot-options)
- title=<title>; plot title
- xlabel=<xlabel>; label for the x-axis
- ylabel=<ylabel>; label for the y-axis
- zlabel=<zlabel>; label for the z-axis (if 3D data is present)


EXAMPLES
plot two sets of 2D data in red and blue
    cat data.csv \\
        | csv-plot --fields \"x0,y0,x1,y1\" \\
                   --style \"x0,colour=red;x1,colour=blue\"

plot three 3D datasets, give each a caption and colour one of them
    cat data.csv \\
        | csv-plot --fields \"x,x1,x2,y,y1,y2,z,z1,z2\" \\
                   --style \"x,caption=foo;x1,caption=bar;x2,caption=baz, \\
                             colour=brown\"

plot a scatter plot using data from the 2nd and 5th comma-separated columns, and
label the axes
    cat data.csv \\
        | csv-plot --fields \",x,,,y\" \\
                   --style \"x,type=points\" \\
                   --plot-options \"xa0,xlabel=some x data,ylabel=some y data\"
";
    eprintln!( "{}", help );
}
