from flask import Flask
from flask_sqlalchemy import SQLAlchemy


# -- Set up the Flask application
app = Flask(__name__)
app.config.from_object('www.default_config')
app.config.from_envvar('LAINWWW_CONFIG', silent=True)
app.jinja_env.auto_reload = app.config['TEMPLATES_AUTO_RELOAD']

# -- Set up Flask extensions
db = SQLAlchemy(app)


# -- Import application structure
from . import views
