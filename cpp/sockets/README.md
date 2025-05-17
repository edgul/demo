
client and server written in c++
* client collects user input from cli arguments and
* sends a message (text) to server
* if "integer" was specified then the client also sends an integer
* server recieves the text, if necessary the integer and the responds with text.

lessons:
* sending ints over the wire, consider endian-ness 
* UNIX socket is for local communication (bypasses the network stack), uses IPC addressed by filename under the hood
* don't assume you will read all the data in one recv() due to TCP segmentation 
* don't recv from file descriptor, use socket from accept()
