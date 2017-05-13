from www import db
from flask_login import UserMixin

class User(db.Model, UserMixin):
	id = db.Column(db.Integer, primary_key=True)
	username = db.Column(db.String(64))
	discriminator = db.Column(db.Integer)

	def __init__(self, id, username, discriminator):
		self.id = id
		self.username = username
		self.discriminator = discriminator

	def __repr__(self):
		return '<User {}#{}>'.format(self.username, self.discriminator)
