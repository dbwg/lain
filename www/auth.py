from requests_oauthlib import OAuth2Session
from flask import Blueprint, request, redirect, abort, flash, url_for, session
from flask_login import login_user, logout_user, login_required

from www import app, db, login_manager
from .models import User
from .utils import is_safe_url

API_BASE_URL = 'https://discordapp.com/api'
OAUTH2_CLIENT_ID = app.config['DISCORD_CLIENT_ID']
OAUTH2_CLIENT_SECRET = app.config['DISCORD_CLIENT_SECRET']
BASE_AUTH_URL = API_BASE_URL + '/oauth2/authorize'
TOKEN_URL = API_BASE_URL + '/oauth2/token'
REQUESTED_SCOPES = ['identify', 'guilds']

import os; os.environ['OAUTHLIB_INSECURE_TRANSPORT'] = 'true'

@login_manager.user_loader
def load_user(uid):
	return User.query.get(uid)

def oauth2_token_updater(token):
	session['oauth2_token'] = token

def oauth2_session(token=None, state=None, scope=None):
	return OAuth2Session(
		client_id=OAUTH2_CLIENT_ID,
		token=token,
		state=state,
		scope=scope,
		redirect_uri=url_for('.authorized', _external=True),
		auto_refresh_kwargs = {
			'client_id': OAUTH2_CLIENT_ID,
			'client_secret': OAUTH2_CLIENT_SECRET,
		},
		auto_refresh_url=TOKEN_URL,
		token_updater=oauth2_token_updater)

def discord_get_user():
	discord = oauth2_session(token=session.get('oauth2_token'))
	user_dat = discord.get(API_BASE_URL + '/users/@me').json()

	return User(user_dat['id'], user_dat['username'],
		user_dat['discriminator'], user_dat['avatar'])

# -- routes
auth = Blueprint('auth', __name__)

@auth.route('/login')
def login():
	discord = oauth2_session(scope=REQUESTED_SCOPES)
	auth_url, state = discord.authorization_url(BASE_AUTH_URL)
	session['oauth2_state'] = state
	session['next'] = request.args.get('next')

	return redirect(auth_url)

@auth.route('/logout')
@login_required
def logout():
	logout_user()
	return redirect(url_for('index'))

@auth.route('/oauth2/authorized')
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

	user = discord_get_user()
	login_user(user)
	db.session.merge(user)
	db.session.commit()


	next = session['next']
	if not is_safe_redirect(next):
		return abort(400)

	flash('Successful OAuth login!', 'success')
	return redirect(next or url_for('index'))
