# RC-Collector
Sample tool used to collect raw cover data from syzkaller via syzkaller http server
## Usage
rawcover-collector 1.0.0
Sam
Syzkaller raw cover collector via http request.

USAGE:
    rawcover_collector [OPTIONS] --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --duration <duration>        Raw cover collecting duration / second [default: 15]
    -o, --output_dir <output_dir>    Output dir for raw cover data [default: .]
    -u, --url <url>                  url of Syzkaller http server [default: http://127.0.0.1:56741/rawcover]
