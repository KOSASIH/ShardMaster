// DistributedTestingFramework.java
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class DistributedTestingFramework {
    private ExecutorService executor;

    public DistributedTestingFramework(int numThreads) {
        executor = Executors.newFixedThreadPool(numThreads);
    }

    public void runTests(TestSuite testSuite) {
        for (Test test : testSuite.getTests()) {
            executor.submit(new TestRunner(test));
        }
        executor.shutdown();
    }

    private static class TestRunner implements Runnable {
        private Test test;

        public TestRunner(Test test) {
            this.test = test;
        }

        @Override
        public void run() {
            test.run();
        }
    }
}
