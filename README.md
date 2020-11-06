# Repeatrrrr

```
repeatrrrr 0.1.0
Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>


USAGE:
    repeatrrrr [OPTIONS] <PATTERN> <COUNT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --write <OUTPUT_FILE>    The outputfile to write to

ARGS:
    <PATTERN>    The pattern to repeat
    <COUNT>      The number of times to generate
```

Needed while trying to debug redis.
Simply execute `repeatrrrr "SET key[] val[]" 5` and it print something like below to the output.
You can specify `--write filename` to write it to a file instead.

```somefile.txt
SET key1 val1
SET key2 val2
SET key3 val3
SET key4 val4
SET key5 val5
```
