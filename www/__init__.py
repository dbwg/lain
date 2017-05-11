from flask import Flask, render_template, flash

app = Flask(__name__)
app.secret_key = "asdf"

@app.route('/')
def index():
	return render_template('index.html')

@app.route('/login')
def login():
	return "login? or smth?"

app.jinja_env.auto_reload = True
app.config['TEMPLATES_AUTO_RELOAD'] = True
