#include <cassert>
#include <climits>
#include <iostream>
#include <cstring>
#include <thread>
#include <unistd.h>         // close()
#include <sys/socket.h>
#include <arpa/inet.h>      // inet_pton()
#include <iostream>
#include <string>
#include <cstdlib>  // for strtol
#include <cerrno>   // for errno
#include <vector>

const char* ws = " \t\n\r\f\v";
const int DEFAULT_CLIENT_COUNT = 1;

// trim from end of string (right)
inline std::string& rtrim(std::string& s, const char* t = ws) {
    s.erase(s.find_last_not_of(t) + 1);
    return s;
}

enum class SendMode {
    NONE,
    STRING,
    INTEGER,
    PROCESS,
};

struct Config {
    SendMode mode;
    int thread_count;
    std::vector<std::string> messages;
};

std::string prepare_message(SendMode m) {
    if (m == SendMode::STRING) {
        return "hello";
    } else if (m == SendMode::INTEGER) {
        return "integer";
    } else if (m == SendMode::PROCESS) {
        return "process";
    }
    assert(false && "Shouldn't prepare message for unknown send mode");
    return "";
}

void connect_to_server(Config c) {
    int sock = 0;
    struct sockaddr_in serv_addr;

    // Create socket
    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        std::cerr << "Socket creation error\n";
        return;
    }

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(8080); // Server port

    // Convert IPv4 address from text to binary form
    if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
        std::cerr << "Invalid address / Address not supported\n";
        return;
    }

    // Connect to the server
    if (connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0) {
        std::cerr << "Connection Failed\n";
        return;
    }

    // send initial message to server
    std::string message = prepare_message(c.mode);
    send(sock, message.c_str(), message.size(), 0);
    std::cout << "[sent] " << message << std::endl;

    // send integer to server when in integer mode
    if (c.mode == SendMode::INTEGER) {
        uint32_t value = 42;
        uint32_t net_value = htonl(value);
        send(sock, &net_value, sizeof(net_value), 0);
    }

    // wait for server response
    char buffer[1024] = {0};
    int valread = read(sock, buffer, 1024);
    std::cout << "[recv]: " << buffer << std::endl;
    close(sock);
}

Config getConfigFromCommandLineArgs(int argc, char*argv[]) {
    Config c = { .mode = SendMode::NONE, .thread_count = 0};

    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <integer>\n";
        return c;
    }

    // quick-and-dirty cli parsing
    SendMode sm = SendMode::STRING;
    int thread_count = DEFAULT_CLIENT_COUNT; 
    std::string message;
    for (int i=1; i<argc; i++) { 
        // printf("processing arg: %s\n", argv[i]);
        std::string s(argv[i]); 
        if (s == "integer") {
            // if we are in integer mode we tell the server we are sending an int via text
            sm = SendMode::INTEGER; 
            continue;
        }

        // if there is an integer as command line arg, that specifies thread number
        char* input = argv[i];
        char* endptr;
        errno = 0;  // Reset errno before call
        long val = std::strtol(input, &endptr, 10);
        if (errno == ERANGE || val > INT_MAX || val < INT_MIN) {
            std::cerr << "Integer out of range.\n";
            continue;
        }
        if (*endptr != '\0') {
            std::cerr << "Invalid characters found: " << endptr << "\n";
            continue;
        }
        thread_count = (int) val;
    }
    c.mode = sm;
    c.thread_count = thread_count;
    return c;
}

int main(int argc, char *argv[]) {
    Config c = getConfigFromCommandLineArgs(argc, argv);
  
    // kick off all the threads
    std::vector<std::thread> threads;
    for (int i=0; i<c.thread_count; i++) {
        std::cout << "opening thread..." << std::endl;
        threads.push_back(std::thread(connect_to_server, c));
    }

    // wait for all threads to finish
    for (std::thread& t : threads) {
        std::cout << "joining thread..." << std::endl;
        t.join();
    }
    std::cout << "finished" << std::endl;

    return 0;
}
