export function connect(url, event_handler) {
    var socket = new WebSocket(url, ["rust-websocket"]);
    socket.binaryType = "arraybuffer"

    socket.onmessage = function (event) {
        event_handler.on_message(new Uint8Array(event.data));
    }
    socket.onopen = function (event) {
        event_handler.on_open();
    }
    socket.onerror = function (event) {
        event_handler.on_error();
        event_handler.free();
    }
    socket.onclose = function (event) {
        event_handler.on_close();
        event_handler.free();
    }

    return new Binding(socket, event_handler)
}

export class Binding {
    constructor(socket, event_handler) {
        this.socket = socket
    }

    send(data) {
        this.socket.send(data);
    }

    close(code, reason) {
        this.socket.close(code, reason);
        this.event_handler.free();
    }
}