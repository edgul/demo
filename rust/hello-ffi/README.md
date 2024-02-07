# hello-ffi

This very simple rust project calls a dynamically linked c library.

### Compile the lib
cc -c src/myc.c
cc -shared myc.o libmyc.so

### Compile the program
rustc -l myc -L . src/main.rs

### Install the shared lib 
cp libmyc.so /usr/local/lib

### Update the system shared lib cache
ldconfig 
