from werkzeug import security
from flask import request, redirect, flash, url_for, jsonify, session
import flask_login

from .models import User
from www import app, db, login_manager
from requests_oauthlib import OAuth2Session

@login_manager.user_loader
def load_user(uid):
	return User.query.get(uid)

API_BASE_URL = 'https://discordapp.com/api'
OAUTH2_CLIENT_ID = app.config['DISCORD_CLIENT_ID']
OAUTH2_CLIENT_SECRET = app.config['DISCORD_CLIENT_SECRET']
BASE_AUTH_URL = API_BASE_URL + '/oauth2/authorize'
TOKEN_URL = API_BASE_URL + '/oauth2/token'
SCOPES = ['identify', 'guilds']

import os; os.environ['OAUTHLIB_INSECURE_TRANSPORT'] = 'true'

def oauth2_token_updater(token):
	session['oauth2_token'] = token

def oauth2_session(token=None, state=None, scope=None):
	return OAuth2Session(
		client_id=OAUTH2_CLIENT_ID,
		token=token,
		state=state,
		scope=scope,
		redirect_uri=url_for('authorized', _external=True),
		auto_refresh_kwargs = {
			'client_id': OAUTH2_CLIENT_ID,
			'client_secret': OAUTH2_CLIENT_SECRET,
		},
		auto_refresh_url=TOKEN_URL,
		token_updater=oauth2_token_updater)

def get_user():
	discord = oauth2_session(token=session.get('oauth2_token'))
	user_dat = discord.get(API_BASE_URL + '/users/@me').json()

	return User(user_dat['id'], user_dat['username'], user_dat['discriminator'])

@app.route('/login')
def login():
	discord = oauth2_session(scope=SCOPES)
	auth_url, state = discord.authorization_url(BASE_AUTH_URL)
	session['oauth2_state'] = state

	return redirect(auth_url)

@app.route('/login/authorized')
def authorized():
	if request.values.get('error'):
		error = request.values['error']
		if error == 'access_denied':
			flash('OAuth flow cancelled :(', 'error')
		else:
			flash('Error in OAuth flow: {}'.format(error), 'error')
		return redirect(url_for('index'))

	discord = oauth2_session(state=session['oauth2_state'])
	token = discord.fetch_token(TOKEN_URL,
		client_secret=OAUTH2_CLIENT_SECRET,
		authorization_response=request.url)

	session['oauth2_token'] = token

	user = get_user()
	flask_login.login_user(user)
	db.session.merge(user)
	db.session.commit()

	flash('Successful OAuth login!', 'success')
	return redirect(url_for('index'))
