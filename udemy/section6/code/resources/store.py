from flask_restful import Resource
from models.store import StoreModel


class Store(Resource):
    def get(self, sname):
        store = StoreModel.find_by_name(sname)
        if store:
            return store.json()
        return {"message": "store does not exist"}, 404

    def post(self, sname):
        store = StoreModel.find_by_name(sname)
        if store:
            return {"message": "a store with name {} already exist".format(sname)}, 400
        else:
            store = StoreModel(sname)
            try:
                store.save_to_db()
            except:
                return {"message": "An error occurred"}, 500
            else:
                return store.json(), 201

    def delete(self, sname):
        store = StoreModel.find_by_name(sname)
        if store:
            try:
                store.delete_from_db()
            except:
                return {"message": "error occurred while deleing"}, 500
            else:
                return {"message": "store deleted"}
        return {"message": "store does not exist with name = {}".format(sname)}


class StoreList(Resource):
    def get(self):
        return {"stores": [store.json() for store in StoreModel.query.all()]}
