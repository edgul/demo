#include <iostream>
#include <cstring>
#include <unistd.h>         // close()
#include <sys/socket.h>
#include <arpa/inet.h>      // inet_pton()
#include <iostream>
#include <string>

const char* ws = " \t\n\r\f\v";

// trim from end of string (right)
inline std::string& rtrim(std::string& s, const char* t = ws) {
    s.erase(s.find_last_not_of(t) + 1);
    return s;
}

int main(int argc, char *argv[]) {
    int sock = 0;
    struct sockaddr_in serv_addr;

    // Create socket
    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        std::cerr << "Socket creation error\n";
        return 1;
    }

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(8080); // Server port

    // Convert IPv4 address from text to binary form
    if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
        std::cerr << "Invalid address / Address not supported\n";
        return 1;
    }

    // Connect to the server
    if (connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0) {
        std::cerr << "Connection Failed\n";
        return 1;
    }

    enum class SendMode {
      STRING,
      INTEGER
    };

    // prepare message
    SendMode sm = SendMode::STRING;
    std::string message;
    for (int i=1; i<argc; i++) { 
        std::string s(argv[i]); 
        if (s == "integer") {
            // if we are in integer mode we tell the server we are sending an int via text
            sm = SendMode::INTEGER; 
        }
        message.append(s);
        message.append(" ");
    }
    if (message.empty()) {
      const char* hello = "Hello";
      message.append(hello);
    }
    message = rtrim(message);

    // send text to server
    send(sock, message.c_str(), message.size(), 0);
    std::cout << "[sent] " << message << std::endl;

    // send integer to server
    if (sm == SendMode::INTEGER) {
        uint32_t value = 42;
        uint32_t net_value = htonl(value);
        send(sock, &net_value, sizeof(net_value), 0);
    }

    // Receive server response
    char buffer[1024] = {0};
    int valread = read(sock, buffer, 1024);
    std::cout << "[recv]: " << buffer << std::endl;

    close(sock);
    return 0;
}
