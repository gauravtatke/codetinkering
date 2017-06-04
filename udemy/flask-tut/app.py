from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route('/') # accessing home page
def home():
    return 'My first flask app'


# POST to receive data (at server)
# GET to send data (from server)

# Sample in-memory store
stores = [
            {
                'name': 'My store',
                'items': [
                            {
                                'name': 'my item1',
                                'price': 15.99
                            }
                        ]
            }
]

# POST /store data: {name:}
@app.route('/store', methods=['POST'])
def create_store():
    data = request.get_json()
    new_store = {
                    'name': data['name'],
                    'items': []
                    }
    stores.append(new_store)
    return jsonify({'stores' : stores})

# GET /store/<string:name>
@app.route('/store/<string:name>')  # http://localhost:5000/store/some_name
def get_store(name):
    # iterate over store
    # if match id found, retrurn it. else return error message.
    for st in stores:
        if st['name'] == name:
            return jsonify(st)
    else:
        return jsonify({'message': 'store not found'})


# GET /store
@app.route('/store')
def get_all_stores():
    # Since json is like a dictionary but stores is a list of dictionaries,
    # we have to create a dictionary and convert it to json.
    return jsonify({'stores': stores})

# POST /store/<string:name>/item {name:, price:,}
@app.route('/store/<string:name>/item', methods=['POST'])
def create_item_in_store(name):
    data = request.get_json()
    for storevar in stores:
        if name == storevar['name']:
            newitem = {
                        'name': data['name'],
                        'price': data['price']
                        }
            storevar['items'].append(newitem)
            return jsonify({'stores' : stores})
    else:
        return jsonify({'message' : 'store not found, item could not added'})



# GET /store/<string:name>/item
@app.route('/store/<string:name>/item')
def get_items_in_store(name):
    for store in stores:
        if store['name'] == name:
            return jsonify({'items': store['items']})
    else:
        return jsonify({'message' : 'store not found'})

app.run(port=5000)
