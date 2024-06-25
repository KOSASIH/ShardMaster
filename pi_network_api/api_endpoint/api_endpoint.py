from flask import Flask, request, jsonify
from flask_jwt_extended import JWTManager, jwt_required, create_access_token

app = Flask(__name__)
app.config["JWT_SECRET_KEY"] = "super-secret-key"
jwt = JWTManager(app)

@app.route("/login", methods=["POST"])
def login():
    username = request.json.get("username")
    password = request.json.get("password")
    if username == "admin" and password == "password":
        access_token = create_access_token(identity=username)
        return jsonify({"access_token": access_token})

@app.route("/protected", methods=["GET"])
@jwt_required
def protected():
    return jsonify({"message": "Hello, authenticated user!"})

if __name__ == "__main__":
    app.run(debug=True)
