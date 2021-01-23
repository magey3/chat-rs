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

var data = JSON.parse(httpPOST("https://localhost:8080/json", {index: 1, amount: 25}));
console.log(data);
