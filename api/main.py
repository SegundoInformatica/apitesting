from flask import Flask, jsonify, request
app = Flask(__name__)

@app.route("/", methods=["GET", "POST"])
def _():
    name = request.args.get("name") or request.form.get("name") or "World"
    return jsonify(f"Hello, {name}!")
