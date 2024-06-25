// ai_test_data_generator.java
import org.deeplearning4j.nn.conf.NeuralNetConfiguration;
import org.deeplearning4j.nn.conf.layers.DenseLayer;
import org.deeplearning4j.nn.multilayer.MultiLayerNetwork;
import org.deeplearning4j.nn.weights.WeightInit;
import org.nd4j.linalg.activations.Activation;
import org.nd4j.linalg.dataset.api.iterator.DataSetIterator;
import org.nd4j.linalg.factory.Nd4j;

public class AiTestDataGenerator {
    private MultiLayerNetwork model;

    public AiTestDataGenerator(int inputSize, int outputSize) {
        NeuralNetConfiguration conf = new NeuralNetConfiguration.Builder()
            .seed(42)
            .weightInit(WeightInit.XAVIER)
            .updater(new Nesterovs(0.01))
            .list()
            .layer(new DenseLayer.Builder()
                .nIn(inputSize)
                .nOut(128)
                .activation(Activation.RELU)
                .build())
            .layer(new DenseLayer.Builder()
                .nIn(128)
                .nOut(outputSize)
                .activation(Activation.IDENTITY)
                .build())
            .pretrain(false)
            .backprop(true)
            .build();

        model = new MultiLayerNetwork(conf);
        model.init();
    }

    public DataSetIterator generateTestData(int batchSize, int numSamples) {
        // Generate test data using the trained neural network
        Nd4j.getRandom().setSeed(42);
        DataSetIterator iterator = new DataSetIterator(batchSize, numSamples);
        for (int i = 0; i < numSamples; i++) {
            INDArray input = Nd4j.rand(inputSize);
            INDArray output = model.output(input);
            iterator.add(input, output);
        }
        return iterator;
    }
}
