#include <iostream>
#include <cstring>
#include <unistd.h>         // close()
#include <sys/socket.h>
#include <netinet/in.h>     // sockaddr_in
#include <arpa/inet.h>      // inet_ntoa()

int main() {
  int server_fd, new_socket;
  struct sockaddr_in address;
  int opt = 1;
  int addrlen = sizeof(address);

  // Create socket file descriptor
  if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
      std::cerr << "Socket failed\n";
      return 1;
  }

  // Attach socket to the port 8080
  if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt))) {
      std::cerr << "setsockopt failed\n";
      return 1;
  }

  address.sin_family = AF_INET;
  address.sin_addr.s_addr = inet_addr("127.0.0.1");
  address.sin_port = htons(8080);

  // Bind the socket
  if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
      std::cerr << "Bind failed\n";
      return 1;
  }

  // Start listening
  if (listen(server_fd, 1) < 0) {
      std::cerr << "Listen failed\n";
      return 1;
  }

  std::cout << "Server is listening on 127.0.0.1:8080...\n";

  int count = 0;
  while (true) {
    // accept one incoming connection
    if ((new_socket = accept(server_fd, (struct sockaddr *)&address, (socklen_t*)&addrlen)) < 0) {
        std::cerr << "Accept failed\n";
        return 1;
    }

    char buffer[1024] = {0};
    int valread = read(new_socket, buffer, 1024);
    std::cout << "[recv]: " << buffer << std::endl;

    // do some "processing" for the client
    std::string reply("hello");
    if (!strcmp(buffer, "process")) {
      std::cout << "processing..." << std::endl;
      sleep(3);
      reply = "I processed this for you " + std::to_string(count);
      count++;
    } 

    // get an integer from the buffer if client is sending an int
    if (!strcmp(buffer, "integer")) {
      std::cout << "receiving integer..." << std::endl;
      uint32_t net_value;
      size_t total_received = 0;
      char* buffer = reinterpret_cast<char*>(&net_value);
      while (total_received < sizeof(net_value)) {
        ssize_t bytes = recv(new_socket, buffer + total_received, sizeof(net_value) - total_received, 0);
        if (bytes <= 0) {
            perror("recv");
            std::cerr << "Connection closed or error." << bytes << "\n";
            break;
        }
        total_received += bytes;
      }
      uint32_t host_value = ntohl(net_value);
      std::cout << "Received int: " << host_value << "\n";
      reply = "I received an int from you";
    }

    // send goodbye
    send(new_socket, reply.c_str(), reply.size(), 0);
    std::cout << "[sent]: " << reply << "\n";
    close(new_socket);

  }
  close(server_fd);

  return 0;
}
