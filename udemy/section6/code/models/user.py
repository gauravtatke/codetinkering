import sqlite3

from db import db


class UserModel(db.Model):
    __tablename__ = 'users'
    id = db.Column('uid', db.Integer, primary_key=True)
    username = db.Column('uname', db.String(80))
    password = db.Column('upwd', db.String(80))

    def __init__(self, username, passwd):
        # self.id = uid
        self.username = username
        self.password = passwd

    @classmethod
    def find_by_username(cls, username):
        # cursor = connection.cursor()
        #
        # query = 'SELECT * FROM users WHERE uname=?'
        # # param to execute should be tuple
        # result = cursor.execute(query, (username,))
        # row = result.fetchone()
        # if row:
        #     user = cls(*row)
        # else:
        #     user = None
        # connection.close()
        # return user
        return cls.query.filter_by(username=username).first()

    @classmethod
    def find_by_id(cls, uid):
        # connection = sqlite3.connect('data.db')
        # cursor = connection.cursor()
        #
        # query = 'SELECT * FROM users WHERE uid=?'
        # # param to execute should be tuple
        # result = cursor.execute(query, (uid,))
        # row = result.fetchone()
        # if row:
        #     user = cls(*row)
        # else:
        #     user = None
        # connection.close()
        # return user
        return cls.query.filter_by(id=uid).first()

    def save_to_db(self):
        db.session.add(self)
        db.session.commit()

    def delete_from_db(self):
        db.session.delete(self)
        db.session.commit()
