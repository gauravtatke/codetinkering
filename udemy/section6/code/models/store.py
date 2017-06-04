# import sqlite3

from db import db


class StoreModel(db.Model):
    __tablename__ = 'stores'

    sid = db.Column('s_id', db.Integer, primary_key=True)
    name = db.Column('s_name', db.String(80))

    items = db.relationship('ItemModel', lazy='dynamic')

    def __init__(self, name):
        self.name = name

    def json(self):
        return {"name": self.name, "items": [item.json() for item in self.items.all()]}

    @classmethod
    def find_by_name(cls, name):
        # connection = sqlite3.connect('data.db')
        # cursor = connection.cursor()
        #
        # query = 'SELECT * FROM items WHERE iname=?'
        #
        # result = cursor.execute(query, (name,))
        # row = result.fetchone()
        # connection.close()
        #
        # if row:
        #     return cls(*row)
        # return None
        return cls.query.filter_by(name=name).first()

    # insert & update method is not needed. save_to_db(self) will insert and update
    # def insert(self):
    #     connection = sqlite3.connect('data.db')
    #     cursor = connection.cursor()
    #     query = "INSERT INTO items VALUES (?, ?)"
    #     cursor.execute(query, (self.name, self.price))
    #     connection.commit()
    #     connection.close()

    # def update(self):
    #     connection = sqlite3.connect('data.db')
    #     cursor = connection.cursor()
    #     query = "UPDATE items SET price=? WHERE iname=?"
    #     cursor.execute(query, (self.price, self.name))
    #     connection.commit()
    #     connection.close()

    def save_to_db(self):
        db.session.add(self)
        db.session.commit()

    def delete_from_db(self):
        db.session.delete(self)
        db.session.commit()
