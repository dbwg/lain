from flask import Flask


# -- Set up the Flask application
app = Flask(__name__)
app.config.from_object('www.default_config')
app.config.from_envvar('LAINWWW_CONFIG', silent=True)
app.jinja_env.auto_reload = app.config['TEMPLATES_AUTO_RELOAD']



# -- Import application structure
from . import views
