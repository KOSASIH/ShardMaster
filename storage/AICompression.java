// storage/AICompression.java
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.zip.Deflater;
import org.deeplearning4j.nn.conf.NeuralNetConfiguration;
import org.deeplearning4j.nn.conf.layers.DenseLayer;
import org.deeplearning4j.nn.multilayer.MultiLayerNetwork;
import org.deeplearning4j.nn.weights.WeightInit;
import org.nd4j.linalg.activations.Activation;
import org.nd4j.linalg.factory.Nd4j;
import org.nd4j.linalg.lossfunctions.LossFunctions;

public class AICompression {
    private MultiLayerNetwork model;

    public AICompression() {
        NeuralNetConfiguration conf = new NeuralNetConfiguration.Builder()
           .seed(42)
           .weightInit(WeightInit.XAVIER)
           .updater(new Nesterovs(0.01))
           .list()
           .layer(new DenseLayer.Builder()
               .nIn(1024)
               .nOut(512)
               .activation(Activation.RELU)
               .build())
           .layer(new DenseLayer.Builder()
               .nIn(512)
               .nOut(256)
               .activation(Activation.RELU)
               .build())
           .layer(new DenseLayer.Builder()
               .nIn(256)
               .nOut(128)
               .activation(Activation.RELU)
               .build())
           .pretrain(false)
           .backprop(true)
           .build();

        model = new MultiLayerNetwork(conf);
        model.init();
    }

    public byte[] compressData(byte[] data) throws IOException {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        Deflater deflater = new Deflater();
        deflater.setInput(data);
        deflater.finish();
        byte[] buffer = new byte[1024];
        while (!deflater.finished()) {
            int count = deflater.deflate(buffer);
            bos.write(buffer, 0, count);
        }
        bos.close();
        return bos.toByteArray();
    }

    public byte[] decompressData(byte[] compressedData) throws IOException {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        Inflater inflater = new Inflater();
        inflater.setInput(compressedData);
        byte[] buffer = new byte[1024];
        while (!inflater.finished()) {
            int count = inflater.inflate(buffer);
            bos.write(buffer, 0, count);
        }
        bos.close();
        return bos.toByteArray();
    }
}
