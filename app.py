import webview
import bottle
from bottle import static_file, response, request
from core.client import HappyClient
import asyncio
import json

app = bottle.Bottle()
client = HappyClient()

def run_sync(async_task):
    loop = asyncio.new_event_loop()
    return loop.run_until_complete(async_task)

def jsonify(data: dict):
    response.content_type = 'application/json'
    return json.dumps(data)

@app.route('/static/<filepath:path>')
def serve_static(filepath):
    return static_file(filepath, root='./static')

@app.route('/')
def index():
    return static_file('index.html', root='./static')

@app.get('/scan')
def scan():
    devices = run_sync(client.scan())
    devices = [{'name': d.name, 'address': d.address} for d in devices]
    return jsonify(devices)

@app.post('/connect')
def connect():
    data = request.json
    address = data.get('address')
    run_sync(client.connect(address))
    return jsonify({})

@app.post('/disconnect')
def disconnect():
    print('disconnecting...')
    run_sync(client.disconnect())
    print('done')
    return jsonify({})

@app.post('/set_power')
def set_power():
    data = request.json
    state = data.get('state')
    run_sync(client.set_power(state))
    return jsonify({})

@app.post('/set_rgb')
def set_rgb():
    data = request.json
    r, g, b = data.get('r'), data.get('g'), data.get('b')
    run_sync(client.set_rgb(r, g ,b))
    return jsonify({})

def main():
    webview.create_window('HappyLight', url=app)
    webview.start()

if __name__ == '__main__':
    main()