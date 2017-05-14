from www import db
from flask_login import UserMixin

class User(db.Model, UserMixin):
	id = db.Column(db.Integer, primary_key=True)
	username = db.Column(db.String(64))
	avatar = db.Column(db.String(32))
	discriminator = db.Column(db.Integer)

	def __init__(self, id, username, discriminator, avatar):
		self.id = id
		self.username = username
		self.discriminator = discriminator
		self.avatar = avatar

	@property
	def avatar_url(self):
		return 'https://cdn.discordapp.com/avatars/{}/{}.png'.format(self.id, self.avatar)

	def __repr__(self):
		return '<User {}#{}>'.format(self.username, self.discriminator)
