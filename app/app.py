import webview
import bottle
from bottle import static_file, response, request, run
from core.client import HappyClient
import asyncio
import json
import sys
from pathlib import Path

app = bottle.Bottle()
client = HappyClient()


def fetch_resource(resource_path: Path | str) -> Path:
    if isinstance(resource_path, str):
        resource_path = Path(resource_path)
    try:  # running as *.exe; fetch resource from temp directory
        base_path = Path(sys._MEIPASS)
    except AttributeError:  # running as script; return unmodified path
        return resource_path.absolute()
    else:  # return temp resource path
        return (base_path / resource_path).absolute()


class EnableCors(object):
    def apply(self, fn, context):

        def _enable_cors(*args, **kwargs):
            # set CORS headers
            response.headers['Access-Control-Allow-Origin'] = '*'
            response.headers['Access-Control-Allow-Methods'] = 'PUT, GET, POST, DELETE'
            response.headers['Access-Control-Allow-Headers'] = 'Authorization, Origin, Accept, Content-Type, X-Requested-With'

            if request.method != 'OPTIONS':
                # actual request; reply with the actual response
                return fn(*args, **kwargs)
        return _enable_cors


def run_sync(async_task):
    loop = asyncio.new_event_loop()
    return loop.run_until_complete(async_task)


def jsonify(data: dict):
    response.content_type = 'application/json'
    return json.dumps(data)


@app.route('/static/<filepath:path>')
def serve_static(filepath):
    filepath = Path('static') / filepath
    filepath = fetch_resource(filepath)
    print(filepath)
    return static_file(str(filepath.name), root=str(filepath.parent))


@app.route('/assets/<filepath:path>')
def serve_static(filepath):
    filepath = Path('static/assets') / filepath
    filepath = fetch_resource(filepath)
    print(filepath)
    return static_file(str(filepath.name), root=str(filepath.parent))


@app.route('/modes')
def get_modes():
    modes = client.get_modes()
    return jsonify(modes)


@app.route('/set_mode', method=['POST', 'OPTIONS'])
def set_mode():
    data = request.json
    mode = data.get('value')
    run_sync(client.set_mode(mode))
    return jsonify({})


@app.route('/')
def index():
    root = fetch_resource('./static')
    return static_file('index.html', root=str(root.absolute()))


@app.route('/scan')
def scan():
    devices = run_sync(client.scan())
    devices = [{'name': d.name, 'address': d.address} for d in devices]
    return jsonify(devices)


@app.route('/connect', method=['POST', 'OPTIONS'])
def connect():
    data = request.json
    address = data.get('address')
    run_sync(client.connect(address))
    return jsonify({})


@app.route('/disconnect', method=['POST', 'GET'])
def disconnect():
    asyncio.run(client.disconnect())
    return jsonify({})


@app.route('/set_power', method=['POST', 'OPTIONS'])
def set_power():
    data = request.json
    state = data.get('state')
    run_sync(client.set_power(state))
    return jsonify({})


@app.route('/set_rgb', method=['POST', 'OPTIONS'])
def set_rgb():
    data = request.json
    r, g, b = data.get('r'), data.get('g'), data.get('b')
    run_sync(client.set_rgb(r, g, b))
    return jsonify({})


def main():
    app.install(EnableCors())
    DEV_MODE = False
    if DEV_MODE:
        run(app, port=8573)
    else:
        webview.create_window('HappyLight', url=app, http_port=8573)
        webview.start(debug=DEV_MODE)


if __name__ == '__main__':
    main()
