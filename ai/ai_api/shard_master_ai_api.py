# shard_master_ai_api.py
from flask import Flask, request, jsonify
from sklearn.ensemble import RandomForestClassifier
from sklearn.datasets import load_iris
from sklearn.model_selection import train_test_split

app = Flask(__name__)

# Load Iris dataset
iris = load_iris()
X = iris.data
y = iris.target
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Train Random Forest model
model = RandomForestClassifier(n_estimators=100)
model.fit(X_train, y_train)

@app.route('/predict', methods=['POST'])
def predict():
    input_data = request.get_json()
    input_array = np.array(input_data)
    prediction = model.predict(input_array)
    return jsonify({'prediction': prediction.tolist()})

if __name__ == '__main__':
    app.run(debug=True)
