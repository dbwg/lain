from flask import render_template, flash
from flask_login import login_required
from www import app

@app.route('/')
def index():
	return render_template('index.html')

@app.route('/dash')
@login_required
def dash():
	return "foooo"

