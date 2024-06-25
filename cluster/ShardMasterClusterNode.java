// ShardMasterClusterNode.java
import java.io.IOException;
import java.net.InetSocketAddress;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.ServerSocketChannel;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class ShardMasterClusterNode {
    private ServerSocketChannel serverChannel;
    private Selector selector;
    private ExecutorService executor;

    public ShardMasterClusterNode(int port) throws IOException {
        serverChannel = ServerSocketChannel.open();
        serverChannel.configureBlocking(false);
        serverChannel.bind(new InetSocketAddress(port));
        selector = Selector.open();
        serverChannel.register(selector, SelectionKey.OP_ACCEPT);
        executor = Executors.newFixedThreadPool(10);
    }

    public void start() {
        while (true) {
            selector.select();
            for (SelectionKey key : selector.selectedKeys()) {
                if (key.isAcceptable()) {
                    ServerSocketChannel channel = (ServerSocketChannel) key.channel();
                    SocketChannel clientChannel = channel.accept();
                    executor.submit(new NodeHandler(clientChannel));
                }
            }
        }
    }

    public void stop() {
        executor.shutdown();
        try {
            serverChannel.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        try {
            selector.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) throws IOException {
        int port = 8081;
        ShardMasterClusterNode node = new ShardMasterClusterNode(port);
        node.start();
    }
}

class NodeHandler implements Runnable {
    private SocketChannel socketChannel;

    public NodeHandler(SocketChannel socketChannel) {
        this.socketChannel = socketChannel;
    }

    @Override
    public void run() {
        // Handle the client connection here
    }
}
