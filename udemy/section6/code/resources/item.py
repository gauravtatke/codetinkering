# import sqlite3

from flask_restful import Resource, reqparse
from flask_jwt import jwt_required
from models.item import ItemModel


class Item(Resource):
    parser = reqparse.RequestParser()
    parser.add_argument('price',
                        type=float,
                        required=True,
                        help="This cannot be left blank"
                        )

    parser.add_argument('store_id',
                        type=int,
                        required=True,
                        help="Every item must have a store"
                        )

    @jwt_required()
    def get(self, name):
        item = ItemModel.find_by_name(name)
        if item:
            return item.json()
        return {"message": "Item not found"}, 404

    def post(self, name):
        if ItemModel.find_by_name(name):
            return {'message': 'an item with name {0} already exists'.format(name)}, 400

        data = Item.parser.parse_args()
        item = ItemModel(name, data['price'], data['store_id'])
        try:
            # item.insert()
            item.save_to_db()
        except:
            # internal server error
            return {"message": "insertion failed for item"}, 500
        return item.json(), 201

    def delete(self, name):
        # connection = sqlite3.connect('data.db')
        # cursor = connection.cursor()
        # query = "DELETE FROM items WHERE iname=?"
        # cursor.execute(query, (name,))
        # connection.commit()
        # connection.close()
        # return {"message": "item deleted"}
        item = ItemModel.find_by_name(name)
        if item:
            item.delete_from_db()
            return {"message": "item deleted"}
        return {"message": "item with this name is not found"}

    def put(self, name):
        data = Item.parser.parse_args()
        item = ItemModel.find_by_name(name)
        # updated_item = ItemModel(name, data["price"])
        if item is None:
            # try:
            #     updated_item.insert()
            # except:
            #     return {"message": "error inserting item"}, 500
            item = ItemModel(name, data['price'], data['store_id'])
        else:
            # try:
            #     updated_item.update()
            # except:
            #     return {'message': 'error updating item'}, 500
            item.price = data['price']
        # return updated_item.json()
        item.save_to_db()
        return item.json()


class ItemList(Resource):
    def get(self):
        # connection = sqlite3.connect('data.db')
        # cursor = connection.cursor()
        # query = "SELECT * FROM items"
        # result = cursor.execute(query)
        # items = []
        # for item in result:
        #     items.append({'name': item[0], 'price': item[1]})
        #
        # connection.close()
        # return {'items': items}
        return {'items': [item.json() for item in ItemModel.query.all()]}
