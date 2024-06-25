# mysql_connector.py
import mysql.connector

class MySQLConnector:
    def __init__(self, host, port, username, password, database):
        self.host = host
        self.port = port
        self.username = username
        self.password = password
        self.database = database
        self.cnx = mysql.connector.connect(
            user=self.username,
            password=self.password,
            host=self.host,
            port=self.port,
            database=self.database
        )

    def execute_query(self, query):
        cursor = self.cnx.cursor()
        cursor.execute(query)
        return cursor.fetchall()
