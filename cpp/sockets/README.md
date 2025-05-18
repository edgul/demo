
Multi-threaded client and server programs written in c++, featuring:
* client collects user input from cli arguments
* client spins up a number (default: 1) of client threads that connect to a server
* the server passes the new connection to a newly created thread for handling 
* each client thread sends a message (text/binary-integer) to it's server thread
* each server thread responds appropriately
* the client parent waits for all threads to finish before closing and terminating
* the server continues to wait for future connections 

lessons:
* sending ints over the wire, consider endian-ness 
* UNIX socket is for local communication (bypasses the network stack), uses IPC addressed by filename under the hood
* don't assume you will read all the data in one recv() due to TCP segmentation 
* don't recv from file descriptor, use socket from accept()
* std::stoi and std::strtol for int parsing from cli
* passing connctions across threads is easy
* std::threads are pretty easy to spinup

ideas:
* thread pool
* make threads more efficient with select()/poll()
* client-side cache
* file upload/download
* image upload/download
* TLS?
* compression

