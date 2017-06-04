import sqlite3

from flask_restful import Resource, reqparse


class User:
    def __init__(self, uid, username, passwd):
        self.id = uid
        self.username = username
        self.password = passwd

    @classmethod
    def find_by_username(cls, username):
        connection = sqlite3.connect('data.db')
        cursor = connection.cursor()

        query = 'SELECT * FROM users WHERE uname=?'
        # param to execute should be tuple
        result = cursor.execute(query, (username,))
        row = result.fetchone()
        if row:
            user = cls(*row)
        else:
            user = None
        connection.close()
        return user

    @classmethod
    def find_by_id(cls, uid):
        connection = sqlite3.connect('data.db')
        cursor = connection.cursor()

        query = 'SELECT * FROM users WHERE uid=?'
        # param to execute should be tuple
        result = cursor.execute(query, (uid,))
        row = result.fetchone()
        if row:
            user = cls(*row)
        else:
            user = None
        connection.close()
        return user


class UserRegister(Resource):
    parser = reqparse.RequestParser()
    parser.add_argument('username',
                        required=True,
                        type=str,
                        help='Username is required'
                        )
    parser.add_argument('password',
                        required=True,
                        type=str,
                        help='Password is required'
                        )

    def post(self):
        data = UserRegister.parser.parse_args()

        if User.find_by_username(data['username']):
            return {"message": "user already exist with this username"}, 400

        connection = sqlite3.connect('data.db')
        cursor = connection.cursor()

        # Null is used for auto-inc id
        query = 'INSERT INTO users VALUES (NULL, ?, ?)'
        cursor.execute(query, (data['username'], data['password']))

        connection.commit()
        connection.close()

        return {'message': 'user created'}, 201
