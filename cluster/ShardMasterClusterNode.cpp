// ShardMasterClusterNode.cpp
#include <boost/asio.hpp>
#include <boost/thread.hpp>
#include <boost/bind.hpp>

using boost::asio::ip::tcp;

class ShardMasterClusterNode {
public:
    ShardMasterClusterNode(boost::asio::io_service& io_service, short port)
        : acceptor_(io_service, tcp::endpoint(tcp::v4(), port)) {
        do_accept();
    }

    void start() {
        io_service_.run();
    }

    void stop() {
        io_service_.stop();
    }

private:
    void do_accept() {
        acceptor_.async_accept([this](boost::system::error_code ec, tcp::socket socket) {
            if (!ec) {
                std::make_shared<ShardMasterNodeSession>(std::move(socket))->run();
            }
            do_accept();
        });
    }

    tcp::acceptor acceptor_;
    boost::asio::io_service io_service_;
};

class ShardMasterNodeSession : public std::enable_shared_from_this<ShardMasterNodeSession> {
public:
    ShardMasterNodeSession(tcp::socket socket)
        : socket_(std::move(socket)) {}

    void run() {
        socket_.async_read_some(
            boost::asio::buffer(data_, max_length),
            [sp = shared_from_this()](boost::system::error_code ec, std::size_t length) {
                if (!ec) {
                    // Handle the data here
                }
            });
    }

private:
    tcp::socket socket_;
    char data_[max_length];
};

int main(int argc, char* argv[]) {
    try {
        boost::asio::io_service io_service;
        ShardMasterClusterNode node(io_service, 8082);
        node.start();
    } catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << "\n";
    }

    return 0;
}
