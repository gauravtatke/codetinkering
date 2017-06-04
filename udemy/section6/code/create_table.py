import sqlite3

connection = sqlite3.connect('data.db')
cursor = connection.cursor()

# there is no difference b/w INTEGER & int
# But if we need the int to be auto incremented, then INTEGER should be used
create_table = 'CREATE TABLE IF NOT EXISTS users (uid INTEGER PRIMARY KEY, uname text, upwd text)'
cursor.execute(create_table)

# create_table = 'CREATE TABLE IF NOT EXISTS items (iname text, price real)'
create_table = 'CREATE TABLE IF NOT EXISTS items (i_id INTEGER PRIMARY KEY, iname text, price real)'
cursor.execute(create_table)

connection.commit()

connection.close()
