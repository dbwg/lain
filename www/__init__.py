from flask import Flask, render_template, flash


# -- Set up the Flask application
app = Flask(__name__)
app.config.from_object('www.default_config')
app.config.from_envvar('LAINWWW_CONFIG', silent=True)
app.jinja_env.auto_reload = app.config['TEMPLATES_AUTO_RELOAD']

@app.route('/')
def index():
	return render_template('index.html')

@app.route('/login')
def login():
	return "login? or smth?"

