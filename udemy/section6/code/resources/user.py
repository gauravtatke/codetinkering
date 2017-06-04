# import sqlite3

from flask_restful import Resource, reqparse
from models.user import UserModel


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

        if UserModel.find_by_username(data['username']):
            return {"message": "user already exist with this username"}, 400

        # connection = sqlite3.connect('data.db')
        # cursor = connection.cursor()
        #
        # # Null is used for auto-inc id
        # query = 'INSERT INTO users VALUES (NULL, ?, ?)'
        # cursor.execute(query, (data['username'], data['password']))
        #
        # connection.commit()
        # connection.close()

        user = UserModel(data['username'], data['password'])
        user.save_to_db()
        return {'message': 'user created'}, 201
