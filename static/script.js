let sendButton = document.getElementById("send-button");
let connectButton = document.getElementById("connect-button");
let input = document.getElementById("input");

let ws;

let connect = () => {
    ws = new WebSocket("ws://localhost:8080/ws");
    ws.onopen = () => {
        console.log("Connected to server");
    };
    ws.onmessage = (event) => {
        console.log(event.data);
    };
    ws.onclose = () => {
        console.log("Disconnected from server");
    };
};

let send = () => {
    ws.send(input.value);
    input.value = "";
};

sendButton.addEventListener("click", send);

connectButton.addEventListener("click", connect);
