function httpPOST(url, payload){
	var r = new XMLHttpRequest();
	r.open("POST", url, false);
	r.setRequestHeader("Content-Type", "application/json");
	var d = JSON.stringify(payload);
	r.send(d);
	return r.responseText;
}

function sendMessage() {
	var date = new Date();
	console.log(date.toISOString());
	document.getElementById("time").value = date.toISOString();
	document.getElementById("id").value = 1;
	return true;
}

function renderMessages(){
	var par = document.getElementById("messages");
	var messages = JSON.parse(httpPOST("https://localhost:8080/json", {index: 1, amount: 25}));
	for(var i = 0; i < messages.messages.length; i++) {
		var el = document.createElement("div");
		var text = document.createTextNode(messages.messages[i].content);
		el.appendChild(text);
		par.appendChild(el);
	}
}

const Messages = {
	data() {
		return {
			items: JSON.parse(httpPOST("/json", {index: 1, amount: 25})).messages
		}
	}
}
console.log(JSON.parse(httpPOST("/json", {index: 1, amount: 25})).messages);
//var data = JSON.parse(httpPOST("https://localhost:8080/json", {index: 1, amount: 25}));
//console.log(data);
//renderMessages();
Vue.createApp(Messages).mount("#messages");
