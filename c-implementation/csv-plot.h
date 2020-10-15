

typedef struct Point
{
    double  this;
    Point  *next;
} Point; 


typedef struct Data
{
    int     indices[3];  // positions in csv string of relevant data points
    Point **points;
    Data   *next;
} Data;


/**  main.c  */
void help();


/**  parseArgs.c - generate an appropriate data structure based on the 
                   program's command line arguments                     */
Data *parseArgs( char **argv );  // generate Data structure


/**  accumulate.c - accumulate points on stdin into a buffer  */
Data *accumulatePoints( Data *first );
char *element( int index, char separator, char *row );


/**  updatePlot.c - generate and update gnuplot session  */
void updatePlot( Data *first );
