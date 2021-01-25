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

const Messages = {
	data() {
		return {
			items: JSON.parse(httpPOST("/json", {index: 1, amount: 100})).messages
		}
	},
	methods: {
		pollData () {
			items: setInterval(() => {
				this.items = JSON.parse(httpPOST("/json", {index: 1, amount: 100})).messages
			}, 3000)
		}
	},
	beforeUnmount: function (){
		clearInterval(items)
	},
	created: function () {
		this.pollData()
	}
}
Vue.createApp(Messages).mount("#messages");
var messages = document.getElementById("messages");
messages.scrollTop = messages.scrollHeight;
